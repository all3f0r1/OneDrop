//! Per-vertex shader execution pipeline
//!
//! Executes per-vertex equations on the GPU using dynamically compiled shaders.

use crate::error::{RenderError, Result};
use wgpu::util::DeviceExt;

/// Per-vertex variables uniform buffer
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexVarsUniform {
    /// Time in seconds
    pub time: f32,
    /// Frame number
    pub frame: f32,
    /// Frames per second
    pub fps: f32,
    /// Bass level (0.0-1.0)
    pub bass: f32,
    
    /// Mid level (0.0-1.0)
    pub mid: f32,
    /// Treble level (0.0-1.0)
    pub treb: f32,
    /// Bass attenuated
    pub bass_att: f32,
    /// Mid attenuated
    pub mid_att: f32,
    
    /// Treble attenuated
    pub treb_att: f32,
    /// Padding for alignment
    pub _padding1: f32,
    pub _padding2: f32,
    pub _padding3: f32,
    
    /// Custom variables q1-q64 (as 16 vec4s for GPU alignment)
    pub q: [[f32; 4]; 16],
}

impl Default for VertexVarsUniform {
    fn default() -> Self {
        Self {
            time: 0.0,
            frame: 0.0,
            fps: 60.0,
            bass: 0.0,
            mid: 0.0,
            treb: 0.0,
            bass_att: 0.0,
            mid_att: 0.0,
            treb_att: 0.0,
            _padding1: 0.0,
            _padding2: 0.0,
            _padding3: 0.0,
            q: [[0.0; 4]; 16],
        }
    }
}

/// Per-vertex shader execution pipeline
pub struct PerVertexPipeline {
    device: wgpu::Device,
    queue: wgpu::Queue,
    
    /// Uniform buffer for vertex variables
    uniform_buffer: wgpu::Buffer,
    
    /// Bind group layout
    bind_group_layout: wgpu::BindGroupLayout,
    
    /// Bind group
    bind_group: Option<wgpu::BindGroup>,
    
    /// Render pipeline
    render_pipeline: Option<wgpu::RenderPipeline>,
    
    /// Vertex buffer
    vertex_buffer: wgpu::Buffer,
    
    /// Index buffer
    index_buffer: wgpu::Buffer,
    
    /// Number of indices
    num_indices: u32,
}

impl PerVertexPipeline {
    /// Create a new per-vertex pipeline
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        vertex_count: u32,
    ) -> Result<Self> {
        // Create uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Vars Uniform Buffer"),
            size: std::mem::size_of::<VertexVarsUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Per-Vertex Bind Group Layout"),
            entries: &[
                // Uniform buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        // Create vertex buffer (for waveform points)
        let vertices = Self::create_waveform_vertices(vertex_count);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        // Create index buffer
        let indices = Self::create_waveform_indices(vertex_count);
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        
        Ok(Self {
            device,
            queue,
            uniform_buffer,
            bind_group_layout,
            bind_group: None,
            render_pipeline: None,
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        })
    }
    
    /// Create waveform vertices
    fn create_waveform_vertices(count: u32) -> Vec<[f32; 3]> {
        let mut vertices = Vec::with_capacity(count as usize);
        
        for i in 0..count {
            let t = i as f32 / (count - 1) as f32;
            let x = t * 2.0 - 1.0; // -1 to 1
            let y = 0.0;
            let z = 0.0;
            vertices.push([x, y, z]);
        }
        
        vertices
    }
    
    /// Create waveform indices (line strip)
    fn create_waveform_indices(count: u32) -> Vec<u16> {
        let mut indices = Vec::with_capacity((count * 2) as usize);
        
        for i in 0..(count - 1) {
            indices.push(i as u16);
            indices.push((i + 1) as u16);
        }
        
        indices
    }
    
    /// Set the shader for this pipeline
    pub fn set_shader(&mut self, shader_module: &wgpu::ShaderModule) -> Result<()> {
        // Create bind group
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Per-Vertex Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
            ],
        });
        
        // Create pipeline layout
        let pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Per-Vertex Pipeline Layout"),
            bind_group_layouts: &[&self.bind_group_layout],
            push_constant_ranges: &[],
        });
        
        // Create render pipeline
        let render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Per-Vertex Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                        ],
                    },
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
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
        });
        
        self.bind_group = Some(bind_group);
        self.render_pipeline = Some(render_pipeline);
        
        Ok(())
    }
    
    /// Update vertex variables
    pub fn update_vars(&mut self, vars: VertexVarsUniform) {
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[vars]));
    }
    
    /// Render per-vertex effects
    pub fn render(&mut self, output_view: &wgpu::TextureView) -> Result<()> {
        let pipeline = self.render_pipeline.as_ref()
            .ok_or_else(|| RenderError::RenderFailed("No shader set".to_string()))?;
        
        let bind_group = self.bind_group.as_ref()
            .ok_or_else(|| RenderError::RenderFailed("No bind group".to_string()))?;
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Per-Vertex Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Per-Vertex Render Pass"),
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
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vertex_vars_size() {
        // Verify struct size is correct (304 bytes)
        assert_eq!(std::mem::size_of::<VertexVarsUniform>(), 304);
    }
    
    #[test]
    fn test_vertex_vars_default() {
        let vars = VertexVarsUniform::default();
        assert_eq!(vars.fps, 60.0);
        assert_eq!(vars.time, 0.0);
    }
    
    #[test]
    fn test_waveform_vertices() {
        let vertices = PerVertexPipeline::create_waveform_vertices(100);
        assert_eq!(vertices.len(), 100);
        assert_eq!(vertices[0][0], -1.0); // First vertex at x=-1
        assert_eq!(vertices[99][0], 1.0); // Last vertex at x=1
    }
    
    #[test]
    fn test_waveform_indices() {
        let indices = PerVertexPipeline::create_waveform_indices(100);
        assert_eq!(indices.len(), 198); // (100-1) * 2
    }
}
