pub mod camera;
pub mod material;
pub mod model;

use std::sync::Arc;

use camera::Camera;
use glam::Mat4;
use material::Material;
use model::{Model, TextureVertex};
use rhachis::{
    graphics::{Bindable, BufferData},
    renderers::Transform,
    GameData, IdMap,
};
use wgpu::{BindGroup, RenderPipeline};

pub struct Renderer {
    pub models: IdMap<Model>,
    pub error_material: Arc<Material>,
    pub camera: BufferData<Camera, [[f32; 4]; 4]>,
    camera_bind_group: BindGroup,
    unshaded_pipeline: RenderPipeline,
}

impl Renderer {
    pub fn new(data: &GameData) -> Self {
        let debug_shader =
            data.graphics
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("debug.wgsl"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("debug.wgsl").into()),
                });

        let unshaded_pipeline_layout =
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

        let unshaded_pipeline =
            data.graphics
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("unshaded_pipeline"),
                    layout: Some(&unshaded_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &debug_shader,
                        entry_point: "unshaded_vertex",
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
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(wgpu::FragmentState {
                        entry_point: "unshaded_fragment",
                        module: &debug_shader,
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

        Self {
            models: IdMap::new(),
            error_material: Arc::new(Material::error(data)),
            camera,
            camera_bind_group,
            unshaded_pipeline,
        }
    }
}

impl rhachis::graphics::Renderer for Renderer {
    fn render<'a, 'b: 'a>(&'b self, render_pass: &'a mut wgpu::RenderPass<'b>) {
        render_pass.set_pipeline(&self.unshaded_pipeline);
        for model in &self.models {
            render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, model.transforms.buffer.slice(..));
            render_pass.set_index_buffer(model.indices.buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &model.material.color.bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.draw_indexed(0..model.indices.buffer_len, 0, 0..1);
        }
    }

    fn update(&mut self, data: &GameData) {
        for model in &mut self.models {
            model.transforms.update(data);
        }
    }
}
