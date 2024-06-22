use std::sync::Arc;

use vulkano::{device::{physical::PhysicalDevice, Device}, format::Format, image::{Image, ImageUsage}, instance::Instance, swapchain::{self, ColorSpace, Surface, SurfaceInfo, Swapchain, SwapchainCreateInfo}};
use winit::{dpi::{LogicalSize, PhysicalSize, Size}, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowAttributes}};

pub struct Context {
    device: Arc<Device>,
    physical_device: Arc<PhysicalDevice>,
    instance: Arc<Instance>,
    window: Option<Arc<Window>>,
    surface: Option<Arc<Surface>>,
    swapchain: Option<Arc<Swapchain>>,
    swap_images: Vec<Arc<Image>>
}

impl Context {
    pub fn new(device: Arc<Device>, physical_device: Arc<PhysicalDevice>, instance: Arc<Instance>) -> Context {
        Context {
            device,
            physical_device,
            instance,
            window: None,
            surface: None,
            swapchain: None,
            swap_images: vec![]
        }
    }
    pub fn init_window_and_swapchain(&mut self, event_loop: &EventLoop<()>, width: u32, height: u32, title: &str) {
        self.window = Some(Arc::new(event_loop.create_window(WindowAttributes::default()
        .with_title(title)
        .with_inner_size(Size::Physical(PhysicalSize::new(width, height)))).expect("Failed to create window")));

        self.surface = Surface::from_window(self.instance.clone(), self.window.clone().unwrap()).ok();
        if self.surface == None {
            panic!("Surface Creation failed");
        }
        let swap_fmt = self.physical_device.surface_formats(self.surface.as_ref().unwrap(), SurfaceInfo::default())
            .expect("Failed to retrieve swap formats")[0];
        let swapchain_and_images = Swapchain::new(self.device.clone(), self.surface.clone().unwrap(),
            SwapchainCreateInfo {
                image_extent: [width, height],
                image_format: swap_fmt.0,
                image_color_space: swap_fmt.1,
                image_usage: ImageUsage::COLOR_ATTACHMENT | ImageUsage::TRANSFER_DST,
                ..SwapchainCreateInfo::default()
            }).expect("Swapchain Creation Failed");
        self.swapchain = Some(swapchain_and_images.0);
        self.swap_images = swapchain_and_images.1;
    }
    pub fn should_close() -> bool{
        true
    }
}