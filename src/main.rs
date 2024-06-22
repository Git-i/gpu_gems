use std::sync::Arc;
use renderer::Context;
use vulkano::{device::{physical::{PhysicalDevice, PhysicalDeviceType}, Device, DeviceCreateInfo, DeviceExtensions, Features, QueueCreateFlags, QueueCreateInfo}, instance::{Instance, InstanceCreateInfo, InstanceExtensions}, memory::allocator::StandardMemoryAllocator, swapchain::{Surface, Swapchain}, Version, VulkanLibrary};
use winit::{application::ApplicationHandler, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::{ControlFlow, EventLoop, EventLoopBuilder}, raw_window_handle::{DisplayHandle, HasDisplayHandle, RawDisplayHandle}, window::Window};
fn device_score(device: &Arc<PhysicalDevice>) -> u32 {
    let mut score: u32 = 0;
    let props = device.properties();
    if props.device_type == PhysicalDeviceType::DiscreteGpu {
        score += 10;
    }
    score
}
fn is_device_viable(device: &Arc<PhysicalDevice>) -> bool {
    //when we require more features this function will have effect
    true
}
fn select_device(devices: impl Iterator<Item = Arc<PhysicalDevice>>) -> Option<Arc<PhysicalDevice>> {
    let mut lol:Option<Arc<PhysicalDevice>> = Option::None;
    let mut highest_score: u32 = 0;
    for device in devices {
        let score = device_score(&device);
        if is_device_viable(&device) &&  score > highest_score {
            lol = Some(device); //is this a move?
            highest_score = score;
        }
    }
    lol
}
fn main() {
    //Do I clone the arcs or are they copy by default
    let library = VulkanLibrary::new().expect("No Vulkan Library found");
    let event_loop = EventLoop::new().expect("Couldn't Create Event Loop");

    let extensions = Surface::required_extensions(&event_loop);
    let info = InstanceCreateInfo {
        max_api_version: Some(Version::V1_3),
        enabled_extensions: extensions,
        ..InstanceCreateInfo::default()
    };
    let instance = Instance::new(library.clone(), info).expect("Couldn't Create Instance");
    
    let device_iter = instance.enumerate_physical_devices().expect("Couldn't enum devices");
    let selected_device = select_device(device_iter).expect("Couldn't find a suitable device");

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
    let (device, mut queues) = Device::new(selected_device.clone(), device_info).expect("Couldn't Create Device");
    //retrieve the one queue we created
    let queue = queues.next().expect("Queue Creation Failed");
    //ripped straight from vulkano docs
    let allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let mut ctx = Context::new(device, selected_device, instance);
    ctx.init_window_and_swapchain(&event_loop, 1280, 720, "LMAO");
    
    
    let res = event_loop.run(move |event: Event<()>, event_loop| {
        match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                },
                WindowEvent::CursorEntered { device_id: _ } => {
                    // On x11, println when the cursor entered in a window even if the child window
                    // is created by some key inputs.
                    // the child windows are always placed at (0, 0) with size (200, 200) in the
                    // parent window, so we also can see this log when we move
                    // the cursor around (200, 200) in parent window.
                    println!("cursor entered in the window {window_id:?}");
                },
                WindowEvent::KeyboardInput {
                    event: KeyEvent { state: ElementState::Pressed, ..}, ..
                } => {
                },
                WindowEvent::RedrawRequested => {
                },
                _ => (),
            },
            _ => (),
        }
    });
    print!("Damm");
}
