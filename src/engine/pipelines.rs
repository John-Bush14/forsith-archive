use crate::vulkan::pipeline::{vkDestroyPipeline, GraphicsPipeline, VkPipeline};

use super::initialisation::pipelines::create_render_pass;

/// add's the pipelines to the engine, activating them
impl crate::engine::Engine {pub fn add_pipelines(&mut self, pipelines: Vec<GraphicsPipeline>) {
    for pipeline in pipelines {
        self.pipelines.push(pipeline);
    }

    self.create_pipeline_layouts();
}}

impl crate::engine::Engine { pub(crate) fn free_pipelines(&mut self) {
    self.pipelines.iter_mut().filter(|pipeline| pipeline.uses <= 0)
        .for_each(|pipeline| {
            unsafe {vkDestroyPipeline(self.device, pipeline.pipeline, std::ptr::null())};
            pipeline.pipeline = 0;
    });
}}

impl crate::engine::Engine { pub(crate) fn create_needed_pipelines(&mut self, recreate: bool) {
	 if recreate || self.render_pass == 0 {
        self.render_pass = create_render_pass(self.device, self.swapchain_image_format.format, self.depth_format, self.msaa_samples);
    };

    let mut changed_pipelines: Vec<(usize, VkPipeline)> = vec!();

    self.pipelines.iter().enumerate()
        .filter(|(_, pipeline)| {
            return  (pipeline.pipeline == 0 || recreate)
                && (pipeline.uses > 0)
        })
        .for_each(|(i, pipeline)| {
            let pipeline = self.create_pipeline(pipeline);

            changed_pipelines.push((i, pipeline));
        });

    for (i, pipeline) in changed_pipelines {
        self.pipelines[i].pipeline = pipeline;
    }

    if recreate || self.framebuffers.len() == 0 {
        self.create_framebuffers();
    }
}}
