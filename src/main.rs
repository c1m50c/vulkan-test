use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::device::physical::PhysicalDevice;
use vulkano::Version;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit::event::{Event, WindowEvent};

use std::sync::Arc;

mod rendering;


fn run(event_loop: EventLoop<()>, window: Window, device: Arc<Device>, queue: Arc<Queue>) {
    use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
    
    let source_content = 0 .. 64;
    let source = CpuAccessibleBuffer::from_iter(
        device.clone(), BufferUsage::all(), false, source_content
    ).unwrap();

    let destination_content = (0 .. 64).map(|_| { 0 });
    let destination = CpuAccessibleBuffer::from_iter(
        device.clone(), BufferUsage::all(), false, destination_content
    ).unwrap();
    
    use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};

    let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    builder.copy_buffer(source.clone(), destination.clone()).unwrap();
    let command_buffer = builder.build().unwrap();
}


fn init_vulkan() -> (Arc<Device>, Arc<Queue>) {
    let instance = Instance::new(
        None,
        Version::V1_1,
        &InstanceExtensions::none(),
        None
    ).expect("Failed to create an Instance");
    
    let physical = PhysicalDevice::enumerate(&instance)
        .next().expect("No PhysicalDevice available.");
    
    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Cannot find a QueueFamily that supports graphics.");
    
    let (device, mut queues) = Device::new(
        physical, &Features::none(), &DeviceExtensions::none(),
        [(queue_family, 0.5)].iter().cloned()).expect("Failed to create a Device.");
    
    return (device, queues.next().unwrap());
}


fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    
    let (device, queue) = init_vulkan();

    run(event_loop, window, device, queue);
}
