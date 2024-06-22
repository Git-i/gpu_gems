use std::sync::Arc;

use vulkano::{device::{physical::PhysicalDevice, Device, DeviceCreateInfo, DeviceExtensions, Features, QueueCreateInfo}, format::Format, image::{Image, ImageUsage}, instance::{Instance, InstanceCreateInfo}, swapchain::{self, ColorSpace, Surface, SurfaceInfo, Swapchain, SwapchainCreateInfo}, Version, VulkanLibrary};
use winit::{dpi::{LogicalSize, PhysicalSize, Size}, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowAttributes}};

use crate::application::{self, Application};

pub struct Context {
    device: Arc<Device>,
    physical_device: Arc<PhysicalDevice>,
    instance: Arc<Instance>,
    
    window: Option<Arc<Window>>,
    surface: Option<Arc<Surface>>,
    swapchain: Option<Arc<Swapchain>>,
    swap_images: Vec<Arc<Image>>
}

/// call this function to see if a physical device is acceptable by the renderer
pub fn is_device_viable(device: &Arc<PhysicalDevice>) -> bool {
    //when we require more features this function will have effect
    true
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
    pub fn create(select_device_fn: impl Fn(Vec<Arc<PhysicalDevice>>) -> Option<Arc<PhysicalDevice>>, application: &Application) -> Context {
        let library = VulkanLibrary::new().expect("No Vulkan Library found");

        let extensions = Surface::required_extensions(&application.event_loop());
        let info = InstanceCreateInfo {
            max_api_version: Some(Version::V1_3),
            enabled_extensions: extensions,
            ..InstanceCreateInfo::default()
        };
        let instance = Instance::new(library.clone(), info).expect("Couldn't Create Instance");
    
        let devices: Vec<Arc<PhysicalDevice>> = instance.enumerate_physical_devices().expect("Couldn't enum devices").collect();
        let physical_device = select_device_fn(devices).expect("Couldn't find a suitable device");

        let device_info = DeviceCreateInfo {
            enabled_features: Features {
                dynamic_rendering: true,
                ..Features::default()
            },
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index: 0,
                ..QueueCreateInfo::default() 
            }],
            enabled_extensions: DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::empty()
            },
            ..DeviceCreateInfo::default()
        };
        let (device, mut queues) = Device::new(physical_device.clone(), device_info).expect("Couldn't Create Device");
        //retrieve the one queue we created
        let queue = queues.next().expect("Queue Creation Failed");
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
    pub fn window(&self) -> &Option<Arc<Window>> {
        &self.window
    }
    pub fn init_for_application(&mut self, application: &mut Application, width: u32, height: u32, title: &str) {
        self.window = Some(Arc::new(application.event_loop().create_window(WindowAttributes::default()
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

        application.add_window(self.window.clone().unwrap());
    }
    pub(crate) fn invalidate_application(&mut self) {
        self.window = None;
        self.surface = None;
        self.swap_images.clear();
        self.swapchain = None;
    }
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }
    pub fn should_close() -> bool{
        true
    }
}