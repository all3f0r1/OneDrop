//! OneDrop GUI - Graphical user interface for Milkdrop visualizations

use anyhow::Result;
use milk_engine::{EngineConfig, MilkEngine, PresetManager, RenderConfig};
use std::sync::Arc;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

struct App {
    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'static>>,
    surface_config: Option<wgpu::SurfaceConfiguration>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    engine: Option<MilkEngine>,
    preset_manager: PresetManager,
    last_frame: Instant,
    frame_count: u32,
}

impl App {
    fn new() -> Self {
        let mut preset_manager = PresetManager::new();
        
        // Add some default presets if available
        if let Ok(entries) = std::fs::read_dir("../test-presets") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("milk") {
                    preset_manager.add_preset(path);
                }
            }
        }
        
        Self {
            window: None,
            surface: None,
            surface_config: None,
            device: None,
            queue: None,
            engine: None,
            preset_manager,
            last_frame: Instant::now(),
            frame_count: 0,
        }
    }
    
    fn init_graphics(&mut self, window: Arc<Window>) -> Result<()> {
        let size = window.inner_size();
        
        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Create surface
        let surface = instance.create_surface(window.clone())?;
        
        // Request adapter
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or_else(|| anyhow::anyhow!("Failed to find adapter"))?;
        
        // Request device and queue
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        ))?;
        
        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);
        
        // Create engine
        let engine_config = EngineConfig {
            render_config: RenderConfig {
                width: size.width,
                height: size.height,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let engine = pollster::block_on(MilkEngine::new(engine_config))?;
        
        self.window = Some(window);
        self.surface = Some(surface);
        self.surface_config = Some(config);
        self.device = Some(device);
        self.queue = Some(queue);
        self.engine = Some(engine);
        
        // Load first preset if available
        if let Some(preset_path) = self.preset_manager.current_preset() {
            if let Some(engine) = &mut self.engine {
                if let Err(e) = engine.load_preset(preset_path) {
                    log::error!("Failed to load preset: {}", e);
                } else {
                    log::info!("Loaded preset: {}", preset_path.display());
                }
            }
        }
        
        Ok(())
    }
    
    fn render(&mut self) -> Result<()> {
        let surface = self.surface.as_ref().unwrap();
        let device = self.device.as_ref().unwrap();
        let queue = self.queue.as_ref().unwrap();
        let engine = self.engine.as_mut().unwrap();
        
        // Calculate delta time
        let now = Instant::now();
        let delta_time = (now - self.last_frame).as_secs_f32();
        self.last_frame = now;
        
        // Generate audio samples (sine wave for demo)
        let audio_samples: Vec<f32> = (0..1024)
            .map(|i| {
                let t = (self.frame_count * 1024 + i) as f32 * 0.001;
                (t * 2.0 * std::f32::consts::PI * 60.0).sin() * 0.5
            })
            .collect();
        
        // Update engine
        engine.update(&audio_samples, delta_time)?;
        
        // Get surface texture
        let output = surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        // Simple render pass (just clear for now)
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
        
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        self.frame_count += 1;
        
        Ok(())
    }
    
    fn handle_keyboard(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Space => {
                // Toggle play/pause (not implemented yet)
                log::info!("Space pressed");
            }
            KeyCode::ArrowRight | KeyCode::KeyN => {
                // Next preset
                if let Some(preset_path) = self.preset_manager.next_preset() {
                    if let Some(engine) = &mut self.engine {
                        if let Err(e) = engine.load_preset(preset_path) {
                            log::error!("Failed to load preset: {}", e);
                        } else {
                            log::info!("Loaded preset: {}", preset_path.display());
                        }
                    }
                }
            }
            KeyCode::ArrowLeft | KeyCode::KeyP => {
                // Previous preset
                if let Some(preset_path) = self.preset_manager.prev_preset() {
                    if let Some(engine) = &mut self.engine {
                        if let Err(e) = engine.load_preset(preset_path) {
                            log::error!("Failed to load preset: {}", e);
                        } else {
                            log::info!("Loaded preset: {}", preset_path.display());
                        }
                    }
                }
            }
            KeyCode::KeyR => {
                // Reset
                if let Some(engine) = &mut self.engine {
                    engine.reset();
                    log::info!("Engine reset");
                }
            }
            KeyCode::Escape | KeyCode::KeyQ => {
                // Quit
                if let Some(window) = &self.window {
                    window.set_visible(false);
                }
            }
            _ => {}
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("OneDrop - Milkdrop Visualizer")
                .with_inner_size(winit::dpi::LogicalSize::new(1280, 720));
            
            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    let window = Arc::new(window);
                    if let Err(e) = self.init_graphics(window) {
                        log::error!("Failed to initialize graphics: {}", e);
                        event_loop.exit();
                    }
                }
                Err(e) => {
                    log::error!("Failed to create window: {}", e);
                    event_loop.exit();
                }
            }
        }
    }
    
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log::info!("Close requested");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(key_code),
                    state: ElementState::Pressed,
                    ..
                },
                ..
            } => {
                self.handle_keyboard(key_code);
            }
            WindowEvent::Resized(physical_size) => {
                if physical_size.width > 0 && physical_size.height > 0 {
                    if let (Some(surface), Some(device), Some(config)) = 
                        (&self.surface, &self.device, &mut self.surface_config) {
                        config.width = physical_size.width;
                        config.height = physical_size.height;
                        surface.configure(device, config);
                        
                        if let Some(engine) = &mut self.engine {
                            engine.resize(physical_size.width, physical_size.height);
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(e) = self.render() {
                    log::error!("Render error: {}", e);
                }
                
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    log::info!("Starting OneDrop GUI...");
    
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut app = App::new();
    
    event_loop.run_app(&mut app)?;
    
    Ok(())
}
