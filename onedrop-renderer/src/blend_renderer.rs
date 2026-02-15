//! Blend renderer for double-preset visualization.
//!
//! This module implements rendering of two presets simultaneously with
//! 27 different blending patterns.

use crate::error::{RenderError, Result};
use std::sync::Arc;
use wgpu::{Device, Queue, TextureView};

/// Blend renderer for double-presets.
pub struct BlendRenderer {
    device: Arc<Device>,
    queue: Arc<Queue>,
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    uniform_buffer: wgpu::Buffer,
    sampler: wgpu::Sampler,
    /// Cached bind group key and bind group for texture pair
    cached_bind_group: Option<CachedBindGroup>,
}

/// Holds cached bind group data
struct CachedBindGroup {
    key_a: usize,
    key_b: usize,
    bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct BlendUniforms {
    blend_pattern: u32,
    blend_amount: f32,
    time: f32,
    _padding: f32,
}

impl BlendRenderer {
    /// Create a new blend renderer.
    pub fn new(device: Arc<Device>, queue: Arc<Queue>, texture_format: wgpu::TextureFormat) -> Result<Self> {
        // Create uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Blend Uniform Buffer"),
            size: std::mem::size_of::<BlendUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Blend Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        
        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Blend Bind Group Layout"),
            entries: &[
                // Texture A
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Texture B
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
                // Uniforms
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Blend Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/blend.wgsl").into()),
        });
        
        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Blend Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        
        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Blend Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
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
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
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
        });
        
        Ok(Self {
            device,
            queue,
            pipeline,
            bind_group_layout,
            uniform_buffer,
            sampler,
            cached_bind_group: None,
        })
    }

    /// Check if cached bind group matches texture pair
    fn is_cached(&self, key_a: usize, key_b: usize) -> bool {
        if let Some(ref cached) = self.cached_bind_group {
            cached.key_a == key_a && cached.key_b == key_b
        } else {
            false
        }
    }

    /// Get or create a cached bind group for the texture pair.
    fn get_cached_bind_group(&self) -> Option<&wgpu::BindGroup> {
        self.cached_bind_group.as_ref().map(|c| &c.bind_group)
    }

    /// Create and cache a new bind group for the texture pair.
    fn create_and_cache_bind_group(
        &mut self,
        texture_a: &TextureView,
        texture_b: &TextureView,
    ) {
        // Use pointer addresses as cache keys (texture views are immutable)
        let key_a = texture_a as *const _ as usize;
        let key_b = texture_b as *const _ as usize;

        // Create new bind group
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Blend Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_a),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(texture_b),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
            ],
        });

        // Cache for future use
        self.cached_bind_group = Some(CachedBindGroup {
            key_a,
            key_b,
            bind_group,
        });
    }

    /// Render blended output.
    pub fn render(
        &mut self,
        texture_a: &TextureView,
        texture_b: &TextureView,
        output: &TextureView,
        blend_pattern: u32,
        blend_amount: f32,
        time: f32,
    ) -> Result<()> {
        // Update uniforms
        let uniforms = BlendUniforms {
            blend_pattern,
            blend_amount,
            time,
            _padding: 0.0,
        };
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        // Check if we need to create a new bind group
        let key_a = texture_a as *const _ as usize;
        let key_b = texture_b as *const _ as usize;

        if !self.is_cached(key_a, key_b) {
            self.create_and_cache_bind_group(texture_a, texture_b);
        }

        let bind_group = self.get_cached_bind_group().expect("bind group should be cached");
        
        // Create command encoder
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Blend Encoder"),
        });
        
        // Render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Blend Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: output,
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
            
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }
        
        // Submit commands
        self.queue.submit(std::iter::once(encoder.finish()));
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blend_uniforms_size() {
        assert_eq!(std::mem::size_of::<BlendUniforms>(), 16);
    }
}
