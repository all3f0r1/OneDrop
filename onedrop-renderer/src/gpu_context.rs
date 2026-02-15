//! GPU context management.

use crate::config::RenderConfig;
use crate::error::{RenderError, Result};
use std::sync::Arc;

/// GPU context containing device, queue, and resources.
pub struct GpuContext {
    /// WGPU device
    pub device: Arc<wgpu::Device>,

    /// Command queue
    pub queue: Arc<wgpu::Queue>,

    /// Render configuration
    pub config: RenderConfig,

    /// Main render texture
    pub render_texture: wgpu::Texture,

    /// Render texture view
    pub render_texture_view: wgpu::TextureView,

    /// Previous frame texture (for feedback effects)
    pub prev_texture: wgpu::Texture,

    /// Previous frame texture view
    pub prev_texture_view: wgpu::TextureView,
}

impl GpuContext {
    /// Create a new GPU context.
    pub async fn new(config: RenderConfig) -> Result<Self> {
        // Create instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| {
                RenderError::DeviceCreationFailed("No suitable GPU adapter found".to_string())
            })?;

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Milkdrop Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        // Create render textures
        let render_texture = Self::create_texture(&device, &config, "Render Texture");
        let render_texture_view =
            render_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let prev_texture = Self::create_texture(&device, &config, "Previous Frame Texture");
        let prev_texture_view = prev_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Self {
            device,
            queue,
            config,
            render_texture,
            render_texture_view,
            prev_texture,
            prev_texture_view,
        })
    }

    /// Create a GPU context from an existing device and queue.
    /// This is useful when sharing a GPU context between multiple components.
    pub fn from_device(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        config: RenderConfig,
    ) -> Self {
        // Create render textures
        let render_texture = Self::create_texture(&device, &config, "Render Texture");
        let render_texture_view =
            render_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let prev_texture = Self::create_texture(&device, &config, "Previous Frame Texture");
        let prev_texture_view = prev_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            device,
            queue,
            config,
            render_texture,
            render_texture_view,
            prev_texture,
            prev_texture_view,
        }
    }

    /// Create a texture with the given configuration.
    fn create_texture(device: &wgpu::Device, config: &RenderConfig, label: &str) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: config.texture_format.to_wgpu(),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        })
    }

    /// Swap render and previous textures (for feedback effects).
    pub fn swap_textures(&mut self) {
        std::mem::swap(&mut self.render_texture, &mut self.prev_texture);
        std::mem::swap(&mut self.render_texture_view, &mut self.prev_texture_view);
    }

    /// Copy current render texture to previous texture.
    pub fn copy_to_prev(&self, encoder: &mut wgpu::CommandEncoder) {
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &self.render_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &self.prev_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
        );
    }

    /// Resize textures.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;

        self.render_texture = Self::create_texture(&self.device, &self.config, "Render Texture");
        self.render_texture_view = self
            .render_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.prev_texture =
            Self::create_texture(&self.device, &self.config, "Previous Frame Texture");
        self.prev_texture_view = self
            .prev_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
    }

    /// Get aspect ratio.
    pub fn aspect_ratio(&self) -> f32 {
        self.config.width as f32 / self.config.height as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_context_creation() {
        let config = RenderConfig::default();
        let context = pollster::block_on(GpuContext::new(config));
        assert!(context.is_ok());
    }

    #[test]
    fn test_aspect_ratio() {
        let config = RenderConfig {
            width: 1920,
            height: 1080,
            ..Default::default()
        };
        let context = pollster::block_on(GpuContext::new(config)).unwrap();
        assert!((context.aspect_ratio() - 16.0 / 9.0).abs() < 0.01);
    }
}
