//! Main renderer implementation.

use crate::config::{RenderConfig, RenderState};
use crate::error::{RenderError, Result};
use crate::gpu_context::GpuContext;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

/// Main Milkdrop renderer.
pub struct MilkRenderer {
    /// GPU context
    gpu: GpuContext,
    
    /// Composite pipeline
    composite_pipeline: wgpu::RenderPipeline,
    
    /// Composite bind group
    composite_bind_group: wgpu::BindGroup,
    
    /// Uniform buffer for composite shader
    composite_uniforms_buffer: wgpu::Buffer,
    
    /// Sampler for textures
    sampler: wgpu::Sampler,
    
    /// Current render state
    state: RenderState,
}

impl MilkRenderer {
    /// Create a new renderer.
    pub async fn new(config: RenderConfig) -> Result<Self> {
        let gpu = GpuContext::new(config).await?;
        
        // Create sampler
        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        
        // Create composite shader
        let composite_shader = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Composite Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/composite.wgsl").into()),
        });
        
        // Create uniform buffer
        let composite_uniforms_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Composite Uniforms"),
            size: std::mem::size_of::<CompositeUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create bind group layout
        let bind_group_layout = gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Composite Bind Group Layout"),
            entries: &[
                // Uniforms
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
                // Previous frame texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        
        // Create bind group
        let composite_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Composite Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: composite_uniforms_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&gpu.prev_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        
        // Create pipeline layout
        let pipeline_layout = gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Composite Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        
        // Create render pipeline
        let composite_pipeline = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Composite Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &composite_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &composite_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: gpu.config.texture_format.to_wgpu(),
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        
        Ok(Self {
            gpu,
            composite_pipeline,
            composite_bind_group,
            composite_uniforms_buffer,
            sampler,
            state: RenderState::default(),
        })
    }
    
    /// Update render state.
    pub fn update_state(&mut self, state: RenderState) {
        self.state = state;
    }
    
    /// Render a frame.
    pub fn render(&mut self) -> Result<()> {
        // Create command encoder
        let mut encoder = self.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        // Update uniforms
        let uniforms = CompositeUniforms {
            resolution: [self.gpu.config.width as f32, self.gpu.config.height as f32],
            time: self.state.time,
            decay: 0.98,
            zoom: self.state.motion.zoom,
            rot: self.state.motion.rot,
            cx: self.state.motion.cx,
            cy: self.state.motion.cy,
            dx: self.state.motion.dx,
            dy: self.state.motion.dy,
            sx: self.state.motion.sx,
            sy: self.state.motion.sy,
            warp: self.state.motion.warp,
            _padding: 0.0,
        };
        
        self.gpu.queue.write_buffer(&self.composite_uniforms_buffer, 0, bytemuck::bytes_of(&uniforms));
        
        // Render composite pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Composite Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.gpu.render_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            render_pass.set_pipeline(&self.composite_pipeline);
            render_pass.set_bind_group(0, &self.composite_bind_group, &[]);
            render_pass.draw(0..4, 0..1);
        }
        
        // Copy current frame to previous frame for next render
        self.gpu.copy_to_prev(&mut encoder);
        
        // Submit commands
        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        
        // Increment frame
        self.state.frame += 1;
        
        Ok(())
    }
    
    /// Get the current render texture.
    pub fn render_texture(&self) -> &wgpu::Texture {
        &self.gpu.render_texture
    }
    
    /// Get render state.
    pub fn state(&self) -> &RenderState {
        &self.state
    }
    
    /// Resize the renderer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
        // TODO: Recreate bind groups with new texture views
    }
}

/// Uniforms for composite shader.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CompositeUniforms {
    resolution: [f32; 2],
    time: f32,
    decay: f32,
    zoom: f32,
    rot: f32,
    cx: f32,
    cy: f32,
    dx: f32,
    dy: f32,
    sx: f32,
    sy: f32,
    warp: f32,
    _padding: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let config = RenderConfig::default();
        let renderer = pollster::block_on(MilkRenderer::new(config));
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_render_frame() {
        let config = RenderConfig::default();
        let mut renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
        
        let result = renderer.render();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_render_texture() {
        let config = RenderConfig::default();
        let width = config.width;
        let height = config.height;
        let renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
        
        let texture = renderer.render_texture();
        assert_eq!(texture.width(), width);
        assert_eq!(texture.height(), height);
    }
    
    #[test]
    fn test_multiple_renders() {
        let config = RenderConfig::default();
        let mut renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
        
        // Render multiple frames
        for _ in 0..10 {
            let result = renderer.render();
            assert!(result.is_ok());
        }
        
        // Verify state progressed
        assert_eq!(renderer.state().frame, 10);
    }
}
