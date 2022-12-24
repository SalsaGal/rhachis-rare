pub mod camera;
pub mod light;
pub mod material;
pub mod model;

use std::{path::Path, sync::Arc};

use camera::Camera;
use glam::{Mat4, Vec3};
use light::{Light, LightUniform};
use material::Material;
use model::{Model, TextureVertex};
use rhachis::{
    graphics::{Bindable, BufferData, SamplerType},
    renderers::{SimpleRenderer, Texture, Transform},
    GameData, IdMap,
};
use wgpu::{BindGroup, Color, RenderPipeline};

pub struct Renderer {
    pub models: IdMap<Model>,
    pub error_material: Arc<Material>,
    pub camera: BufferData<Camera>,
    pub lights: BufferData<Light>,
    pub pipeline: Pipeline,
    depth_texture: Texture,
    camera_bind_group: BindGroup,
    lights_bind_group: BindGroup,
    texture_pipeline: RenderPipeline,
    wireframe_pipeline: RenderPipeline,
    render_pipeline: RenderPipeline,
}

impl Renderer {
    pub fn new(data: &GameData) -> Self {
        let depth_texture = Texture::depth_texture(data, data.get_window_size());

        let debug_shader =
            data.graphics
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("debug.wgsl"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("debug.wgsl").into()),
                });

        let shader = data
            .graphics
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("shader.wgsl"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        let texture_pipeline_layout =
            data.graphics
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[
                        &Material::bind_group_layout(data),
                        &Transform::bind_group_layout(data),
                    ],
                    push_constant_ranges: &[],
                });

        let texture_pipeline =
            data.graphics
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("texture_pipeline"),
                    layout: Some(&texture_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &debug_shader,
                        entry_point: "texture_vertex",
                        buffers: &[TextureVertex::desc(), Transform::desc()],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(wgpu::FragmentState {
                        entry_point: "texture_fragment",
                        module: &debug_shader,
                        targets: &[Some(wgpu::ColorTargetState {
                            format: data.graphics.config.format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    multiview: None,
                });

        let wireframe_pipeline_layout =
            data.graphics
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[
                        &Material::bind_group_layout(data),
                        &Transform::bind_group_layout(data),
                    ],
                    push_constant_ranges: &[],
                });

        let wireframe_pipeline =
            data.graphics
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("wireframe_pipeline"),
                    layout: Some(&wireframe_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &debug_shader,
                        entry_point: "texture_vertex",
                        buffers: &[TextureVertex::desc(), Transform::desc()],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Line,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(wgpu::FragmentState {
                        entry_point: "texture_fragment",
                        module: &debug_shader,
                        targets: &[Some(wgpu::ColorTargetState {
                            format: data.graphics.config.format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    multiview: None,
                });

        let render_pipeline_layout =
            data.graphics
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[
                        &Material::bind_group_layout(data),
                        &Transform::bind_group_layout(data),
                        &LightUniform::bind_group_layout(data),
                    ],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            data.graphics
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("render_pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vertex_main",
                        buffers: &[TextureVertex::desc(), Transform::desc()],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(wgpu::FragmentState {
                        entry_point: "fragment_main",
                        module: &shader,
                        targets: &[Some(wgpu::ColorTargetState {
                            format: data.graphics.config.format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    multiview: None,
                });

        let camera = BufferData::new(
            data,
            vec![Camera::default()],
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );

        let camera_bind_group =
            data.graphics
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &Mat4::bind_group_layout(data),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            camera.buffer.as_entire_buffer_binding(),
                        ),
                    }],
                });

        let lights = BufferData::new(
            data,
            vec![Light {
                pos: Vec3::new(3.0, 0.0, 2.0),
                color: Color::RED,
            }],
            wgpu::BufferUsages::STORAGE,
        );

        let lights_bind_group =
            data.graphics
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &LightUniform::bind_group_layout(data),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            lights.buffer.as_entire_buffer_binding(),
                        ),
                    }],
                });

        Self {
            models: IdMap::new(),
            error_material: Arc::new(Material::error(data)),
            camera,
            lights,
            pipeline: Pipeline::Normal,
            depth_texture,
            camera_bind_group,
            lights_bind_group,
            texture_pipeline,
            wireframe_pipeline,
            render_pipeline,
        }
    }

    pub fn load_gltf<P: AsRef<Path>>(
        &mut self,
        data: &GameData,
        path: P,
        scene: usize,
    ) -> Vec<usize> {
        let scene = &easy_gltf::load(path).unwrap()[scene];
        self.models.append(
            scene
                .models
                .iter()
                .map(|model| {
                    let vertices = model
                        .vertices()
                        .iter()
                        .map(|vertex| TextureVertex {
                            pos: vertex.position.into(),
                            tex_coords: vertex.tex_coords.into(),
                            normals: vertex.normal.into(),
                        })
                        .collect();
                    let indices = model
                        .indices()
                        .unwrap()
                        .iter()
                        .map(|index| *index as u16)
                        .collect();
                    let material = Arc::new(Material {
                        color: Texture::from_path(data, "examples/test.png", &SamplerType::Linear)
                            .unwrap(),
                    });
                    Model::new(
                        data,
                        vertices,
                        indices,
                        material,
                        vec![Transform::default()],
                    )
                })
                .collect(),
        )
    }

    pub fn with_gltf<P: AsRef<Path>>(mut self, data: &GameData, path: P, scene: usize) -> Self {
        self.load_gltf(data, path, scene);
        self
    }

    pub const FEATURES: wgpu::Features = wgpu::Features::POLYGON_MODE_LINE;
}

impl rhachis::graphics::Renderer for Renderer {
    fn render<'a, 'b: 'a>(&'b self, render_pass: &'a mut wgpu::RenderPass<'b>) {
        macro_rules! default_render_routine {
            () => {
                for model in &self.models {
                    render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, model.transforms.buffer.slice(..));
                    render_pass.set_index_buffer(
                        model.indices.buffer.slice(..),
                        wgpu::IndexFormat::Uint16,
                    );
                    render_pass.set_bind_group(0, &model.material.color.bind_group, &[]);
                    render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                    render_pass.draw_indexed(0..model.indices.buffer_len, 0, 0..1);
                }
            };
        }
        match self.pipeline {
            Pipeline::Normal => {
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(2, &self.lights_bind_group, &[]);
                default_render_routine!();
            }
            Pipeline::Texture => {
                render_pass.set_pipeline(&self.texture_pipeline);
                default_render_routine!();
            }
            Pipeline::Wireframe => {
                render_pass.set_pipeline(&self.wireframe_pipeline);
                default_render_routine!();
            }
            _ => todo!(),
        }
    }

    fn make_render_pass<'a>(
        &'a self,
        view: &'a wgpu::TextureView,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass {
        let depth_texture = match self.pipeline {
            Pipeline::Wireframe => None,
            _ => Some(&self.depth_texture.view),
        };
        SimpleRenderer::render_pass(view, encoder, depth_texture)
    }

    fn update(&mut self, data: &GameData) {
        self.camera.values[0].update_aspect(data);
        for model in &mut self.models {
            model.transforms.update(data);
        }
    }

    fn resize(&mut self, data: &GameData, size: glam::UVec2) {
        self.depth_texture = Texture::depth_texture(data, size);
    }
}

pub enum Pipeline {
    Normal,
    Texture,
    Wireframe,
    Color,
}
