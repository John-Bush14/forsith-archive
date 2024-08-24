use crate::vulkan::{
    devices::device::VkDevice,
    swapchain::VkExtent2D,
    image::VkImageView,
    vertex::{
        VkVertexInputBindingDescription,
        VkVertexInputAttributeDescription
    },
    VkBool32,
    VkResult,
    VkStructureType
};

use std::ffi::{
    c_void,
    c_char
};

use super::{image::Texture, uniform::DescriptorBindings};


pub type VkShaderModule = u64;

pub type VkPipelineLayout = u64;

pub type VkRenderPass = u64;

pub type VkPipeline = u64;

pub type VkPipelineCache = u64;

pub type VkFramebuffer = u64;


#[derive(Clone)]
pub enum BuiltinUniform {
    Camera2d,
    Camera3d,
    Model2d,
    Model3d,
}

#[derive(Clone)]
pub enum ShaderType {
    Sampler2D
}

impl UniformType {
    pub fn to_shader_item(&self) -> ShaderItem {
        match self {
            Self::Builtin(_) => ShaderItem::Void,
            _ =>  {
                let shader_type = match self {Self::Local(x) => x, Self::Global(x) => x, _ => &ShaderType::Sampler2D};

                match shader_type {
                    ShaderType::Sampler2D => ShaderItem::Sampler2D(Default::default())
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum ShaderItem {
    Sampler2D(Texture),
    Void
}

#[derive(Clone)]
pub enum UniformType {
    Builtin(BuiltinUniform),
    Local(ShaderType),
    Global(ShaderType)
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ShaderStage {
    Fragment,
    Vertex
}

#[derive(Clone)]
pub struct GraphicsPipeline {
    pub pipeline: VkPipeline,
    pub vertex_shader: VkShaderModule,
    pub fragment_shader: VkShaderModule,
    pub uniform_layout: std::collections::HashMap<ShaderStage, Vec<UniformType>>,
    pub global_uniforms: std::collections::HashMap<ShaderStage, Vec<ShaderItem>>,
    pub descriptor_bindings: DescriptorBindings,
    pub uses: u32
}


#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub stage: u32,
    pub module: VkShaderModule,
    pub name: *const c_char,
    pub specialization_info: *const c_void
}

#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub depth_clamp_enable: VkBool32,
    pub rasterizer_discard_enable: VkBool32,
    pub polygon_mode: u32,
    pub cull_mode: u32,
    pub front_face: u32,
    pub depth_bias_enable: VkBool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32
}

#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub rasterization_samples: u32,
    pub sample_shading_enable: VkBool32,
    pub min_sample_shading: f32,
    pub sample_mask: *const c_void, // temp
    pub alpha_to_coverage_enable: VkBool32,
    pub alpha_to_one_enable: VkBool32
}

#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub vertex_binding_description_count: u32,
    pub vertex_binding_descriptions: *const VkVertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub vertex_attribute_descriptions: *const VkVertexInputAttributeDescription,
}

#[repr(C)]
pub struct VkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32
}

#[repr(C)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32
}

#[repr(C)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D
}

#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub viewport_count: u32,
    pub viewports: *const VkViewport,
    pub scissor_count: u32,
    pub scissors: *const VkRect2D
}

#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blend_enable: VkBool32,
    pub src_color_blend_factor: u32,
    pub dst_color_blend_factor: u32,
    pub color_blend_op: u32,
    pub src_alpha_blend_factor: u32,
    pub dst_alpha_blend_factor: u32,
    pub alpha_blend_op: u32,
    pub color_write_mask: u32
}

#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub logic_op_enable: VkBool32,
    pub logic_op: u32,
    pub attachment_count: u32,
    pub attachments: *const VkPipelineColorBlendAttachmentState,
    pub blend_constants: [f32;4]
}

#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub topology: u32,
    pub primitive_restart_enable: VkBool32
}

#[repr(C)]
pub struct VkShaderModuleCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub code_size: usize,
    pub code: *const u32
}

#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub set_layout_count: u32,
    pub set_layouts: *const u64,
    pub push_constant_range_count: u32,
    pub push_constant_ranges: *const c_void
}

#[repr(C)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: u32
}

#[repr(C)]
pub struct VkSubpassDescription {
    pub flags: u32,
    pub pipeline_bind_point: u32,
    pub input_attachment_count: u32,
    pub input_attachments: *const VkAttachmentReference,
    pub color_attachment_count: u32,
    pub color_attachments: *const VkAttachmentReference,
    pub resolve_attachments: *const VkAttachmentReference,
    pub depth_stencil_attachment: *const VkAttachmentReference,
    pub preserve_attachment_count: u32,
    pub preserve_attachments: *const u32
}

#[repr(C)]
pub struct VkAttachmentDescription {
    pub flags: u32,
    pub format: u32,
    pub samples: u32,
    pub load_op: u32,
    pub store_op: u32,
    pub stencil_load_op: u32,
    pub stencil_store_op: u32,
    pub initial_layout: u32,
    pub final_layout: u32
}

#[repr(C)]
pub struct VkSubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: u32,
    pub dst_stage_mask: u32,
    pub src_acces_mask: u32,
    pub dst_acces_mask: u32,
    pub dependency_flags: u32
}

#[repr(C)]
pub struct VkRenderPassCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub attachment_count: u32,
    pub attachments: *const VkAttachmentDescription,
    pub subpass_count: u32,
    pub subpasses: *const VkSubpassDescription,
    pub dependency_count: u32,
    pub dependencies: *const VkSubpassDependency
}

#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub patch_control_points: u32
}

#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub depth_test_enable: VkBool32,
    pub depth_write_enable: VkBool32,
    pub depth_compare_op: u32,
    pub depth_bounds_test_enable: VkBool32,
    pub stencil_test_enable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VkStencilOpState {
    fail_op: u32,
    pass_op: u32,
    depth_fail_op: u32,
    compare_op: u32,
    compare_mask: u32,
    write_mask: u32,
    reference: u32
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub stage_count: u32,
    pub stages: *const VkPipelineShaderStageCreateInfo,
    pub vertex_input_state: *const VkPipelineVertexInputStateCreateInfo,
    pub input_assembly_state: *const VkPipelineInputAssemblyStateCreateInfo,
    pub tesselation_state: *const VkPipelineTessellationStateCreateInfo,
    pub viewport_state: *const VkPipelineViewportStateCreateInfo,
    pub rasterization_state: *const VkPipelineRasterizationStateCreateInfo,
    pub multisample_state: *const VkPipelineMultisampleStateCreateInfo,
    pub depth_stencil_state: *const VkPipelineDepthStencilStateCreateInfo,
    pub color_blend_state: *const VkPipelineColorBlendStateCreateInfo,
    pub dynamic_state: *const c_void,
    pub layout: VkPipelineLayout,
    pub render_pass: VkRenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: VkPipeline,
    pub base_pipeline_handle_index: i32
}

#[repr(C)]
pub struct VkFramebufferCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: u32,
    pub render_pass: VkRenderPass,
    pub attachment_count: u32,
    pub attachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32
}


#[link(name = "vulkan")]
extern "C" {
    pub fn vkCreateShaderModule(
        device: VkDevice,
        create_info: *const VkShaderModuleCreateInfo,
        _: *const c_void,
        shader_module: *mut VkShaderModule
    ) -> VkResult;

    pub fn vkCreatePipelineLayout(
        device: VkDevice,
        create_info: *const VkPipelineLayoutCreateInfo,
        _: *const c_void,
        pipeline_layout: *mut VkPipelineLayout
    ) -> VkResult;

    pub fn vkCreateRenderPass(
        device: VkDevice,
        create_info: *const VkRenderPassCreateInfo,
        _: *const c_void,
        render_pass: *mut VkRenderPass
    ) -> VkResult;

    pub fn vkCreateGraphicsPipelines(
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        create_info_count: u32,
        create_infos: *const VkGraphicsPipelineCreateInfo,
        _: *const c_void,
        pipelines: *mut VkPipeline
    ) -> VkResult;

    pub fn vkDestroyShaderModule(
        device: VkDevice,
        shader_module: VkShaderModule,
        _: *const c_void
    );

    pub fn vkDestroyPipelineLayout(
        device: VkDevice,
        pipeline_layout: VkPipelineLayout,
        _: *const c_void
    );

    pub fn vkDestroyRenderPass(
        device: VkDevice,
        render_pass: VkRenderPass,
        _: *const c_void
    );

    pub fn vkDestroyPipeline(
        device: VkDevice,
        pipeline: VkPipeline,
        _: *const c_void
    );

    pub fn vkCreateFramebuffer(
        device: VkDevice,
        create_info: *const VkFramebufferCreateInfo,
        _: *const c_void,
        framebuffer: *mut VkFramebuffer
    ) -> VkResult;

    pub fn vkDestroyFramebuffer(
        device: VkDevice,
        framebuffer: VkFramebuffer,
        _: *const c_void
    );
}
