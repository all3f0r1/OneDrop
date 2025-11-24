//! Waveform rendering module.

use wgpu::util::DeviceExt;

/// Waveform rendering modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveformMode {
    /// Centered waveform (default)
    Centered = 0,
    /// Left channel only
    LeftChannel = 1,
    /// Right channel only
    RightChannel = 2,
    /// Spectrum analyzer
    Spectrum = 3,
    /// Circular waveform
    Circular = 4,
    /// Line waveform
    Line = 5,
}

/// Waveform point data.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WavePoint {
    pub position: [f32; 2],
    pub value: f32,
    pub _padding: f32,
}

/// Waveform renderer.
pub struct WaveformRenderer {
    /// Render pipeline
    pipeline: wgpu::RenderPipeline,
    
    /// Dots pipeline
    dots_pipeline: wgpu::RenderPipeline,
    
    /// Uniform buffer
    uniform_buffer: wgpu::Buffer,
    
    /// Wave data buffer
    wave_buffer: wgpu::Buffer,
    
    /// Bind group
    bind_group: wgpu::BindGroup,
    
    /// Number of samples
    num_samples: usize,
}

impl WaveformRenderer {
    /// Create a new waveform renderer.
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        num_samples: usize,
    ) -> Self {
        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Waveform Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../shaders/waveform_advanced.wgsl").into()
            ),
        });
        
        // Create uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Waveform Uniform Buffer"),
            size: 64, // Enough for uniforms
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create wave data buffer
        let wave_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Wave Data Buffer"),
            size: (num_samples * std::mem::size_of::<WavePoint>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Waveform Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Waveform Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wave_buffer.as_entire_binding(),
                },
            ],
        });
        
        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Waveform Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        
        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Waveform Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        
        // Create dots pipeline
        let dots_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Waveform Dots Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_dots",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_dots",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        
        Self {
            pipeline,
            dots_pipeline,
            uniform_buffer,
            wave_buffer,
            bind_group,
            num_samples,
        }
    }
    
    /// Update waveform data.
    pub fn update_wave_data(&self, queue: &wgpu::Queue, samples: &[f32]) {
        // Convert samples to wave points
        let mut points = Vec::with_capacity(samples.len().min(self.num_samples));
        
        for (i, &sample) in samples.iter().take(self.num_samples).enumerate() {
            let x = i as f32 / self.num_samples as f32;
            points.push(WavePoint {
                position: [x, 0.5],
                value: sample,
                _padding: 0.0,
            });
        }
        
        // Pad if necessary
        while points.len() < self.num_samples {
            points.push(WavePoint {
                position: [0.0, 0.5],
                value: 0.0,
                _padding: 0.0,
            });
        }
        
        queue.write_buffer(&self.wave_buffer, 0, bytemuck::cast_slice(&points));
    }
    
    /// Render waveform.
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        use_dots: bool,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Waveform Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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
        
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        
        if use_dots {
            render_pass.set_pipeline(&self.dots_pipeline);
        } else {
            render_pass.set_pipeline(&self.pipeline);
        }
        
        // Draw 6 vertices per point (2 triangles = 1 quad)
        let vertex_count = (self.num_samples * 6) as u32;
        render_pass.draw(0..vertex_count, 0..1);
    }
}
