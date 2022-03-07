use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::device::physical::PhysicalDevice;
use vulkano::Version;

use winit::event_loop::{EventLoopWindowTarget, ControlFlow};
use winit::window::WindowId;
use winit::event::Event;


#[inline]
pub fn render(event: Event<()>, target: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow, window_id: WindowId) {
    
}