//! Per-pixel shader execution pipeline
//!
//! Executes per-pixel equations on the GPU using dynamically compiled shaders.

use crate::error::{RenderError, Result};
use wgpu::util::DeviceExt;

/// Per-pixel variables uniform buffer
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PixelVarsUniform {
    // Coordinates
    pub x: f32,
    pub y: f32,
    pub rad: f32,
    pub ang: f32,

    // Audio
    pub bass: f32,
    pub mid: f32,
    pub treb: f32,
    pub bass_att: f32,
    pub mid_att: f32,
    pub treb_att: f32,

    // Time
    pub time: f32,
    pub frame: f32,
    pub fps: f32,

    // Padding for alignment
    pub _padding: f32,

    // Custom variables (64 floats as 16 vec4s)
    pub q: [[f32; 4]; 16],
}

impl Default for PixelVarsUniform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            rad: 0.0,
            ang: 0.0,
            bass: 0.0,
            mid: 0.0,
            treb: 0.0,
            bass_att: 0.0,
            mid_att: 0.0,
            treb_att: 0.0,
            time: 0.0,
            frame: 0.0,
            fps: 60.0,
            _padding: 0.0,
            q: [[0.0; 4]; 16],
        }
    }
}

/// Per-pixel rendering pipeline
#[allow(dead_code)]
pub struct PerPixelPipeline {
    device: wgpu::Device,
    queue: wgpu::Queue,

    // Pipeline state
    render_pipeline: Option<wgpu::RenderPipeline>,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: Option<wgpu::BindGroup>,

    // Uniform buffer
    vars_buffer: wgpu::Buffer,
    vars: PixelVarsUniform,

    // Textures
    input_texture: Option<wgpu::Texture>,
    output_texture: Option<wgpu::Texture>,
    sampler: wgpu::Sampler,

    // Resolution
    width: u32,
    height: u32,
}

impl PerPixelPipeline {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, width: u32, height: u32) -> Result<Self> {
        // Create uniform buffer
        let vars = PixelVarsUniform::default();
        let vars_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Per-Pixel Vars Buffer"),
            contents: bytemuck::cast_slice(&[vars]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Per-Pixel Bind Group Layout"),
            entries: &[
                // Uniform buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Input texture
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });

        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Per-Pixel Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            device,
            queue,
            render_pipeline: None,
            bind_group_layout,
            bind_group: None,
            vars_buffer,
            vars,
            input_texture: None,
            output_texture: None,
            sampler,
            width,
            height,
        })
    }

    /// Set the shader module from compiled WGSL
    pub fn set_shader(&mut self, shader_module: &wgpu::ShaderModule) -> Result<()> {
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Per-Pixel Pipeline Layout"),
                bind_group_layouts: &[&self.bind_group_layout],
                push_constant_ranges: &[],
            });

        self.render_pipeline = Some(self.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Per-Pixel Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: shader_module,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader_module,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
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
                multiview: None,
                cache: None,
            },
        ));

        Ok(())
    }

    /// Update uniform variables
    pub fn update_vars(&mut self, vars: PixelVarsUniform) {
        self.vars = vars;
        self.queue
            .write_buffer(&self.vars_buffer, 0, bytemuck::cast_slice(&[self.vars]));
    }

    /// Set input texture
    pub fn set_input_texture(&mut self, texture: wgpu::Texture) {
        self.input_texture = Some(texture);
        self.update_bind_group();
    }

    /// Update bind group with current textures
    fn update_bind_group(&mut self) {
        if let Some(ref input_texture) = self.input_texture {
            let input_view = input_texture.create_view(&wgpu::TextureViewDescriptor::default());

            self.bind_group = Some(self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Per-Pixel Bind Group"),
                layout: &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.vars_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&input_view),
                    },
                ],
            }));
        }
    }

    /// Render per-pixel effects
    pub fn render(&mut self, output_view: &wgpu::TextureView) -> Result<()> {
        let pipeline = self
            .render_pipeline
            .as_ref()
            .ok_or_else(|| RenderError::RenderFailed("No shader set".to_string()))?;

        let bind_group = self
            .bind_group
            .as_ref()
            .ok_or_else(|| RenderError::RenderFailed("No bind group".to_string()))?;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Per-Pixel Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Per-Pixel Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: output_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.draw(0..6, 0..1); // Full-screen quad (2 triangles)
        }

        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_vars_size() {
        // Verify struct size is correct (312 bytes)
        assert_eq!(std::mem::size_of::<PixelVarsUniform>(), 312);
    }

    #[test]
    fn test_pixel_vars_default() {
        let vars = PixelVarsUniform::default();
        assert_eq!(vars.fps, 60.0);
        assert_eq!(vars.time, 0.0);
    }
}
