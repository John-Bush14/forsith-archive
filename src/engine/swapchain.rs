use crate::vulkan::{
    swapchain::{
        vkDestroySwapchainKHR,
        image_view::vkDestroyImageView
    },
    pipeline::{
        vkDestroyRenderPass,
        vkDestroyPipeline,
        vkDestroyFramebuffer
    },
    commands::command_buffer::vkFreeCommandBuffers,
    devices::device::vkDeviceWaitIdle
};


impl super::Engine { pub fn cleanup_swapchain(&mut self) { unsafe {
    self.framebuffers.iter().for_each(|&framebuffer| vkDestroyFramebuffer(self.device, framebuffer, std::ptr::null()));

    vkFreeCommandBuffers(self.device, self.command_pool, self.command_buffers.len() as u32, self.command_buffers.as_ptr());

    vkDestroyRenderPass(self.device, self.render_pass, std::ptr::null());

    //vkDestroyPipelineLayout(self.device, self.pipeline_layout, std::ptr::null());
        
    self.pipelines.iter().for_each(|pipeline| {
        vkDestroyPipeline(self.device, pipeline.pipeline, std::ptr::null())
    });

    self.swapchain_image_views.iter().for_each(|&image_view| vkDestroyImageView(self.device, image_view, std::ptr::null()));

    vkDestroySwapchainKHR(self.device, self.swapchain, std::ptr::null());
}}}

impl super::Engine { pub fn recreate_swapchain(&mut self) {
    if self.new_dimensions.is_some() {
        println!("dimenions: {:?}", self.dimensions);
        while self.new_dimensions.unwrap()[0] <= 0 || self.new_dimensions.unwrap()[1] <= 0 {self.process_events();}
        self.dimensions = self.new_dimensions.unwrap(); self.new_dimensions = None
    }

    unsafe {vkDeviceWaitIdle(self.device)};

    self.cleanup_swapchain();

    self.create_swapchain();

    self.create_image_views();

    self.create_pipelines(true);

    self.create_command_buffers();

    self.record_and_enter_command_buffers();
}}
