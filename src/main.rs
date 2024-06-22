use std::{collections::HashMap, fmt::Debug, sync::Arc};
use renderer::{application::{run_app, Application}, context::{is_device_viable, Context}};
use vulkano::{device::{physical::{PhysicalDevice, PhysicalDeviceType}, Device, DeviceCreateInfo, DeviceExtensions, Features, QueueCreateFlags, QueueCreateInfo}, instance::{Instance, InstanceCreateInfo, InstanceExtensions}, memory::allocator::StandardMemoryAllocator, swapchain::{Surface, Swapchain}, Version, VulkanLibrary};
use winit::{application::ApplicationHandler, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::{ControlFlow, EventLoop, EventLoopBuilder}, raw_window_handle::{DisplayHandle, HasDisplayHandle, RawDisplayHandle}, window::{self, Window}};

fn device_score(device: &Arc<PhysicalDevice>) -> u32 {
    let mut score: u32 = 0;
    let props = device.properties();
    if props.device_type == PhysicalDeviceType::DiscreteGpu {
        score += 10;
    }
    score
}

fn select_device(devices: Vec<Arc<PhysicalDevice>>) -> Option<Arc<PhysicalDevice>> {
    let mut selected:Option<Arc<PhysicalDevice>> = Option::None;
    let mut highest_score: u32 = 0;
    for device in devices {
        let score = device_score(&device);
        if is_device_viable(&device) &&  score > highest_score {
            selected = Some(device);
            highest_score = score;
        }
    }
    selected
}

fn update(ctx: &Context)
{
    println!("Custom update function to do rendering");
}

fn main() {
    let mut app = Application::new();
    app.frame_fn = update;
    let mut ctx = Context::create(select_device, &app);
    ctx.init_for_application(&mut app, 1280, 720, "LMAO");
    run_app(app, ctx);
    print!("Damm");
}
