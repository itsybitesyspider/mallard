use super::headless::*;
use std::sync::Arc;

pub struct BasicRenderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

pub trait LikeBasicRenderer : LikeHeadlessRenderer {
    fn surface(&self) -> &wgpu::Surface;
}

impl LikeHeadlessRenderer for BasicRenderer {
    fn device(&self) -> &wgpu::Device {
        &self.device
    }

    fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

impl<R> LikeBasicRenderer for &R
where R: LikeBasicRenderer {
    fn surface(&self) -> &wgpu::Surface {
        (*self).surface()
    }    
}

impl<R> LikeBasicRenderer for Arc<R>
where R: LikeBasicRenderer {
    fn surface(&self) -> &wgpu::Surface {
        self.as_ref().surface()
    }   
}

impl BasicRenderer {
    pub async unsafe fn new<W: raw_window_handle::HasRawWindowHandle>(w: W) -> Arc<Self> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(&w) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: None,
            },
        ).await.unwrap();
        let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();
        
        Arc::new(
            BasicRenderer {
                surface,
                device, 
                queue
            }
        )
    }
}