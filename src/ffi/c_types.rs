/// Represents device memory size, and device memory offset values.
///
/// Corresponding C code:
/// ```C
/// typedef uint64_t VkDeviceSize;
/// ```
pub type VkDeviceSize = u64;

/// Represents device buffer address values.
///
/// Corresponding C code:
/// ```C
/// typedef uint64_t VkDeviceAddress;
/// ```
pub type VkDeviceAddress = u64;

/// A bitmask representing a collection of flags.
pub type VkFlags = u32;
pub type VkInstanceCreateFlags = VkFlags;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkDeviceQueueCreateFlags = VkFlags;
pub type VkSampleCountFlags = VkFlags;
pub type VkShaderStageFlags = VkFlags;
pub type VkSubgroupFeatureFlags = VkFlags;
pub type VkResolveModeFlags = VkFlags;
pub type VkQueueFlags = VkFlags;


pub type VkBool32 = u32;

/// A 64-bit bitmask representing a collection of flags.
pub type VkFlags64 = u64;
pub type VkPhysicalDeviceSchedulingControlsFlagBitsARM = VkFlags64;
pub type VkPhysicalDeviceSchedulingControlsFlagsARM = VkFlags64;


#[allow(non_snake_case, non_camel_case_types)]
pub mod enums {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct VkResult {
        bits: i32,
    }

    impl VkResult {
        pub const VK_SUCCESS: VkResult = VkResult { bits: 0 };
        pub const VK_NOT_READY: VkResult = VkResult { bits: 1 };
        pub const VK_TIMEOUT: VkResult = VkResult { bits: 2 };
        pub const VK_EVENT_SET: VkResult = VkResult { bits: 3 };
        pub const VK_EVENT_RESET: VkResult = VkResult { bits: 4 };
        pub const VK_INCOMPLETE: VkResult = VkResult { bits: 5 };
        pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = VkResult { bits: -1 };
        pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = VkResult { bits: -2 };
        pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = VkResult { bits: -3 };
        pub const VK_ERROR_DEVICE_LOST: VkResult = VkResult { bits: -4 };
        pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = VkResult { bits: -5 };
        pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = VkResult { bits: -6 };
        pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = VkResult { bits: -7 };
        pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = VkResult { bits: -8 };
        pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = VkResult { bits: -9 };
        pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = VkResult { bits: -10 };
        pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = VkResult { bits: -11 };
        pub const VK_ERROR_FRAGMENTED_POOL: VkResult = VkResult { bits: -12 };
        pub const VK_ERROR_UNKNOWN: VkResult = VkResult { bits: -13 };
        pub const VK_ERROR_OUT_OF_POOL_MEMORY: VkResult = VkResult { bits: -1000069000 };
        pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: VkResult = VkResult { bits: -1000072003 };
        pub const VK_ERROR_FRAGMENTATION: VkResult = VkResult { bits: -1000161000 };
        pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: VkResult = VkResult { bits: -1000257000 };
        pub const VK_PIPELINE_COMPILE_REQUIRED: VkResult = VkResult { bits: 1000297000 };
        pub const VK_ERROR_NOT_PERMITTED: VkResult = VkResult { bits: -1000174001 };
        pub const VK_ERROR_SURFACE_LOST_KHR: VkResult = VkResult { bits: -1000000000 };
        pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = VkResult { bits: -1000000001 };
        pub const VK_SUBOPTIMAL_KHR: VkResult = VkResult { bits: 1000001003 };
        pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult = VkResult { bits: -1000001004 };
        pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: VkResult = VkResult { bits: -1000003001 };
        pub const VK_ERROR_VALIDATION_FAILED_EXT: VkResult = VkResult { bits: -1000011001 };
        pub const VK_ERROR_INVALID_SHADER_NV: VkResult = VkResult { bits: -1000012000 };
        pub const VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: VkResult = VkResult { bits: -1000023000 };
        pub const VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: VkResult = VkResult { bits: -1000023001 };
        pub const VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: VkResult = VkResult { bits: -1000023002 };
        pub const VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: VkResult = VkResult { bits: -1000023003 };
        pub const VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: VkResult = VkResult { bits: -1000023004 };
        pub const VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: VkResult = VkResult { bits: -1000023005 };
        pub const VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: VkResult = VkResult { bits: -1000158000 };
        pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult = VkResult { bits: -1000255000 };
        pub const VK_THREAD_IDLE_KHR: VkResult = VkResult { bits: 1000268000 };
        pub const VK_THREAD_DONE_KHR: VkResult = VkResult { bits: 1000268001 };
        pub const VK_OPERATION_DEFERRED_KHR: VkResult = VkResult { bits: 1000268002 };
        pub const VK_OPERATION_NOT_DEFERRED_KHR: VkResult = VkResult { bits: 1000268003 };
        pub const VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: VkResult = VkResult { bits: -1000299000 };
        pub const VK_ERROR_COMPRESSION_EXHAUSTED_EXT: VkResult = VkResult { bits: -1000338000 };
        pub const VK_INCOMPATIBLE_SHADER_BINARY_EXT: VkResult = VkResult { bits: 1000482000 };
        pub const VK_PIPELINE_BINARY_MISSING_KHR: VkResult = VkResult { bits: 1000483000 };
        pub const VK_ERROR_NOT_ENOUGH_SPACE_KHR: VkResult = VkResult { bits: -1000483000 };
        pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR: VkResult = VkResult { bits: -1000069000 };
        pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR: VkResult = VkResult { bits: -1000072003 };
        pub const VK_ERROR_FRAGMENTATION_EXT: VkResult = VkResult { bits: -1000161000 };
        pub const VK_ERROR_NOT_PERMITTED_EXT: VkResult = VkResult { bits: -1000174001 };
        pub const VK_ERROR_NOT_PERMITTED_KHR: VkResult = VkResult { bits: -1000174001 };
        pub const VK_ERROR_INVALID_DEVICE_ADDRESS_EXT: VkResult = VkResult { bits: -1000257000 };
        pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: VkResult = VkResult { bits: -1000257000 };
        pub const VK_PIPELINE_COMPILE_REQUIRED_EXT: VkResult = VkResult { bits: 1000297000 };
        pub const VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT: VkResult = VkResult { bits: 1000297000 };
        pub const VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT: VkResult = VkResult { bits: 1000482000 };
    }

    impl From<i32> for VkResult {
        fn from(bits: i32) -> Self {
            Self { bits }
        }
    }

    impl Into<i32> for VkResult {
        fn into(self) -> i32 {
            self.bits
        }
    }

    /// The VkStructureType enum. Contains structure types within Vulkan.
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct VkStructureType {
        pub bits: i32,
    }

    impl VkStructureType {
        pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = VkStructureType { bits: 0 };
        pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1 };
        pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = VkStructureType { bits: 2 };
        pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = VkStructureType { bits: 3 };
        pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = VkStructureType { bits: 4 };
        pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = VkStructureType { bits: 5 };
        pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: VkStructureType = VkStructureType { bits: 6 };
        pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: VkStructureType = VkStructureType { bits: 7 };
        pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = VkStructureType { bits: 8 };
        pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = VkStructureType { bits: 9 };
        pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: VkStructureType = VkStructureType { bits: 10 };
        pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: VkStructureType = VkStructureType { bits: 11 };
        pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = VkStructureType { bits: 12 };
        pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: VkStructureType = VkStructureType { bits: 13 };
        pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: VkStructureType = VkStructureType { bits: 14 };
        pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = VkStructureType { bits: 15 };
        pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = VkStructureType { bits: 16 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: VkStructureType = VkStructureType { bits: 17 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType = VkStructureType { bits: 18 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 19 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 20 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 21 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 22 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 23 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 24 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 25 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 26 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 27 };
        pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType = VkStructureType { bits: 28 };
        pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = VkStructureType { bits: 29 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = VkStructureType { bits: 30 };
        pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: VkStructureType = VkStructureType { bits: 31 };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType = VkStructureType { bits: 32 };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = VkStructureType { bits: 33 };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = VkStructureType { bits: 34 };
        pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = VkStructureType { bits: 35 };
        pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: VkStructureType = VkStructureType { bits: 36 };
        pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: VkStructureType = VkStructureType { bits: 37 };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: VkStructureType = VkStructureType { bits: 38 };
        pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = VkStructureType { bits: 39 };
        pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = VkStructureType { bits: 40 };
        pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: VkStructureType = VkStructureType { bits: 41 };
        pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = VkStructureType { bits: 42 };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: VkStructureType = VkStructureType { bits: 43 };
        pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: VkStructureType = VkStructureType { bits: 44 };
        pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: VkStructureType = VkStructureType { bits: 45 };
        pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: VkStructureType = VkStructureType { bits: 46 };
        pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: VkStructureType = VkStructureType { bits: 47 };
        pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: VkStructureType = VkStructureType { bits: 48 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: VkStructureType = VkStructureType { bits: 1000094000 };
        pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO: VkStructureType = VkStructureType { bits: 1000157000 };
        pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO: VkStructureType = VkStructureType { bits: 1000157001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: VkStructureType = VkStructureType { bits: 1000083000 };
        pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS: VkStructureType = VkStructureType { bits: 1000127000 };
        pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO: VkStructureType = VkStructureType { bits: 1000127001 };
        pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO: VkStructureType = VkStructureType { bits: 1000060000 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: VkStructureType = VkStructureType { bits: 1000060003 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = VkStructureType { bits: 1000060004 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO: VkStructureType = VkStructureType { bits: 1000060005 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO: VkStructureType = VkStructureType { bits: 1000060006 };
        pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: VkStructureType = VkStructureType { bits: 1000060013 };
        pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: VkStructureType = VkStructureType { bits: 1000060014 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES: VkStructureType = VkStructureType { bits: 1000070000 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000070001 };
        pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = VkStructureType { bits: 1000146000 };
        pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = VkStructureType { bits: 1000146001 };
        pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = VkStructureType { bits: 1000146002 };
        pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2: VkStructureType = VkStructureType { bits: 1000146003 };
        pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: VkStructureType = VkStructureType { bits: 1000146004 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2: VkStructureType = VkStructureType { bits: 1000059000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2: VkStructureType = VkStructureType { bits: 1000059001 };
        pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2: VkStructureType = VkStructureType { bits: 1000059002 };
        pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = VkStructureType { bits: 1000059003 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: VkStructureType = VkStructureType { bits: 1000059004 };
        pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2: VkStructureType = VkStructureType { bits: 1000059005 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: VkStructureType = VkStructureType { bits: 1000059006 };
        pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = VkStructureType { bits: 1000059007 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: VkStructureType = VkStructureType { bits: 1000059008 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: VkStructureType = VkStructureType { bits: 1000117000 };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000117001 };
        pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000117002 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000117003 };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000053000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES: VkStructureType = VkStructureType { bits: 1000053001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: VkStructureType = VkStructureType { bits: 1000053002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: VkStructureType = VkStructureType { bits: 1000120000 };
        pub const VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO: VkStructureType = VkStructureType { bits: 1000145000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: VkStructureType = VkStructureType { bits: 1000145001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: VkStructureType = VkStructureType { bits: 1000145002 };
        pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2: VkStructureType = VkStructureType { bits: 1000145003 };
        pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000156000 };
        pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO: VkStructureType = VkStructureType { bits: 1000156001 };
        pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO: VkStructureType = VkStructureType { bits: 1000156002 };
        pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: VkStructureType = VkStructureType { bits: 1000156003 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: VkStructureType = VkStructureType { bits: 1000156004 };
        pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: VkStructureType = VkStructureType { bits: 1000156005 };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000085000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: VkStructureType = VkStructureType { bits: 1000071000 };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES: VkStructureType = VkStructureType { bits: 1000071001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: VkStructureType = VkStructureType { bits: 1000071002 };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES: VkStructureType = VkStructureType { bits: 1000071003 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES: VkStructureType = VkStructureType { bits: 1000071004 };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000072000 };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000072001 };
        pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO: VkStructureType = VkStructureType { bits: 1000072002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: VkStructureType = VkStructureType { bits: 1000112000 };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES: VkStructureType = VkStructureType { bits: 1000112001 };
        pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000113000 };
        pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000077000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: VkStructureType = VkStructureType { bits: 1000076000 };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES: VkStructureType = VkStructureType { bits: 1000076001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: VkStructureType = VkStructureType { bits: 1000168000 };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT: VkStructureType = VkStructureType { bits: 1000168001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: VkStructureType = VkStructureType { bits: 1000063000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: VkStructureType = VkStructureType { bits: 49 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: VkStructureType = VkStructureType { bits: 50 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: VkStructureType = VkStructureType { bits: 51 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: VkStructureType = VkStructureType { bits: 52 };
        pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000147000 };
        pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2: VkStructureType = VkStructureType { bits: 1000109000 };
        pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2: VkStructureType = VkStructureType { bits: 1000109001 };
        pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2: VkStructureType = VkStructureType { bits: 1000109002 };
        pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2: VkStructureType = VkStructureType { bits: 1000109003 };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2: VkStructureType = VkStructureType { bits: 1000109004 };
        pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO: VkStructureType = VkStructureType { bits: 1000109005 };
        pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO: VkStructureType = VkStructureType { bits: 1000109006 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: VkStructureType = VkStructureType { bits: 1000177000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES: VkStructureType = VkStructureType { bits: 1000196000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: VkStructureType = VkStructureType { bits: 1000180000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: VkStructureType = VkStructureType { bits: 1000082000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: VkStructureType = VkStructureType { bits: 1000197000 };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000161000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: VkStructureType = VkStructureType { bits: 1000161001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: VkStructureType = VkStructureType { bits: 1000161002 };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: VkStructureType = VkStructureType { bits: 1000161003 };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: VkStructureType = VkStructureType { bits: 1000161004 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: VkStructureType = VkStructureType { bits: 1000199000 };
        pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: VkStructureType = VkStructureType { bits: 1000199001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: VkStructureType = VkStructureType { bits: 1000221000 };
        pub const VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000246000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: VkStructureType = VkStructureType { bits: 1000130000 };
        pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000130001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: VkStructureType = VkStructureType { bits: 1000211000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: VkStructureType = VkStructureType { bits: 1000108000 };
        pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000108001 };
        pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: VkStructureType = VkStructureType { bits: 1000108002 };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO: VkStructureType = VkStructureType { bits: 1000108003 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: VkStructureType = VkStructureType { bits: 1000253000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: VkStructureType = VkStructureType { bits: 1000175000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: VkStructureType = VkStructureType { bits: 1000241000 };
        pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT: VkStructureType = VkStructureType { bits: 1000241001 };
        pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: VkStructureType = VkStructureType { bits: 1000241002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: VkStructureType = VkStructureType { bits: 1000261000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: VkStructureType = VkStructureType { bits: 1000207000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: VkStructureType = VkStructureType { bits: 1000207001 };
        pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000207002 };
        pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO: VkStructureType = VkStructureType { bits: 1000207003 };
        pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO: VkStructureType = VkStructureType { bits: 1000207004 };
        pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO: VkStructureType = VkStructureType { bits: 1000207005 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: VkStructureType = VkStructureType { bits: 1000257000 };
        pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO: VkStructureType = VkStructureType { bits: 1000244001 };
        pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: VkStructureType = VkStructureType { bits: 1000257002 };
        pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: VkStructureType = VkStructureType { bits: 1000257003 };
        pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: VkStructureType = VkStructureType { bits: 1000257004 };
        pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000001000 };
        pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR: VkStructureType = VkStructureType { bits: 1000001001 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: VkStructureType = VkStructureType { bits: 1000060007 };
        pub const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000060008 };
        pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: VkStructureType = VkStructureType { bits: 1000060009 };
        pub const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000060010 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR: VkStructureType = VkStructureType { bits: 1000060011 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000060012 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000002000 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000002001 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR: VkStructureType = VkStructureType { bits: 1000003000 };
        pub const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000004000 };
        pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000005000 };
        pub const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000006000 };
        pub const VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000008000 };
        pub const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000009000 };
        pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000011000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: VkStructureType = VkStructureType { bits: 1000018000 };
        pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT: VkStructureType = VkStructureType { bits: 1000022000 };
        pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT: VkStructureType = VkStructureType { bits: 1000022001 };
        pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT: VkStructureType = VkStructureType { bits: 1000022002 };
        pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000026000 };
        pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000026001 };
        pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000026002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000028000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000028001 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000028002 };
        pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX: VkStructureType = VkStructureType { bits: 1000030000 };
        pub const VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: VkStructureType = VkStructureType { bits: 1000041000 };
        pub const VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: VkStructureType = VkStructureType { bits: 1000049000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000050000 };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000056000 };
        pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000056001 };
        pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType = VkStructureType { bits: 1000057000 };
        pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType = VkStructureType { bits: 1000057001 };
        pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: VkStructureType = VkStructureType { bits: 1000058000 };
        pub const VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT: VkStructureType = VkStructureType { bits: 1000061000 };
        pub const VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN: VkStructureType = VkStructureType { bits: 1000062000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000066000 };
        pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT: VkStructureType = VkStructureType { bits: 1000067000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000067001 };
        pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000073000 };
        pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000073001 };
        pub const VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: 1000073002 };
        pub const VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000073003 };
        pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR: VkStructureType = VkStructureType { bits: 1000074000 };
        pub const VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: 1000074001 };
        pub const VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR: VkStructureType = VkStructureType { bits: 1000074002 };
        pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000075000 };
        pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000078000 };
        pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000078001 };
        pub const VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR: VkStructureType = VkStructureType { bits: 1000078002 };
        pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000078003 };
        pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR: VkStructureType = VkStructureType { bits: 1000079000 };
        pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR: VkStructureType = VkStructureType { bits: 1000079001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: 1000080000 };
        pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: VkStructureType = VkStructureType { bits: 1000081000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000081001 };
        pub const VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT: VkStructureType = VkStructureType { bits: 1000081002 };
        pub const VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR: VkStructureType = VkStructureType { bits: 1000084000 };
        pub const VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX: VkStructureType = VkStructureType { bits: 1000086000 };
        pub const VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX: VkStructureType = VkStructureType { bits: 1000086001 };
        pub const VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX: VkStructureType = VkStructureType { bits: 1000086002 };
        pub const VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX: VkStructureType = VkStructureType { bits: 1000086003 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX: VkStructureType = VkStructureType { bits: 1000086004 };
        pub const VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX: VkStructureType = VkStructureType { bits: 1000086005 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000087000 };
        pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT: VkStructureType = VkStructureType { bits: 1000090000 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT: VkStructureType = VkStructureType { bits: 1000091000 };
        pub const VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT: VkStructureType = VkStructureType { bits: 1000091001 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT: VkStructureType = VkStructureType { bits: 1000091002 };
        pub const VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000091003 };
        pub const VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE: VkStructureType = VkStructureType { bits: 1000092000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: VkStructureType = VkStructureType { bits: 1000097000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000098000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000099000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000099001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000101000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000101001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000102000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000102001 };
        pub const VK_STRUCTURE_TYPE_HDR_METADATA_EXT: VkStructureType = VkStructureType { bits: 1000105000 };
        pub const VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: VkStructureType = VkStructureType { bits: 1000111000 };
        pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000114000 };
        pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000114001 };
        pub const VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000114002 };
        pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR: VkStructureType = VkStructureType { bits: 1000115000 };
        pub const VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR: VkStructureType = VkStructureType { bits: 1000115001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: VkStructureType = VkStructureType { bits: 1000116000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: 1000116001 };
        pub const VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000116002 };
        pub const VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR: VkStructureType = VkStructureType { bits: 1000116003 };
        pub const VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR: VkStructureType = VkStructureType { bits: 1000116004 };
        pub const VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR: VkStructureType = VkStructureType { bits: 1000116005 };
        pub const VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR: VkStructureType = VkStructureType { bits: 1000116006 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: VkStructureType = VkStructureType { bits: 1000119000 };
        pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR: VkStructureType = VkStructureType { bits: 1000119001 };
        pub const VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR: VkStructureType = VkStructureType { bits: 1000119002 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR: VkStructureType = VkStructureType { bits: 1000121000 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR: VkStructureType = VkStructureType { bits: 1000121001 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR: VkStructureType = VkStructureType { bits: 1000121002 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR: VkStructureType = VkStructureType { bits: 1000121003 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR: VkStructureType = VkStructureType { bits: 1000121004 };
        pub const VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK: VkStructureType = VkStructureType { bits: 1000122000 };
        pub const VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK: VkStructureType = VkStructureType { bits: 1000123000 };
        pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT: VkStructureType = VkStructureType { bits: 1000128000 };
        pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT: VkStructureType = VkStructureType { bits: 1000128001 };
        pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT: VkStructureType = VkStructureType { bits: 1000128002 };
        pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: VkStructureType = VkStructureType { bits: 1000128003 };
        pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000128004 };
        pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: VkStructureType = VkStructureType { bits: 1000129000 };
        pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: VkStructureType = VkStructureType { bits: 1000129001 };
        pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: VkStructureType = VkStructureType { bits: 1000129002 };
        pub const VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: VkStructureType = VkStructureType { bits: 1000129003 };
        pub const VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: VkStructureType = VkStructureType { bits: 1000129004 };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID: VkStructureType = VkStructureType { bits: 1000129005 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000138000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000138001 };
        pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: VkStructureType = VkStructureType { bits: 1000138002 };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000138003 };
        pub const VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT: VkStructureType = VkStructureType { bits: 1000143000 };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: VkStructureType = VkStructureType { bits: 1000143001 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000143002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000143003 };
        pub const VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000143004 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000148000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000148001 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000148002 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000149000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000152000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000154000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: VkStructureType = VkStructureType { bits: 1000154001 };
        pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: VkStructureType = VkStructureType { bits: 1000158000 };
        pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000158001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: VkStructureType = VkStructureType { bits: 1000158002 };
        pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000158003 };
        pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000158004 };
        pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000158005 };
        pub const VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000160000 };
        pub const VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000160001 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000164000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000164001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: VkStructureType = VkStructureType { bits: 1000164002 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000164005 };
        pub const VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000165000 };
        pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000165001 };
        pub const VK_STRUCTURE_TYPE_GEOMETRY_NV: VkStructureType = VkStructureType { bits: 1000165003 };
        pub const VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV: VkStructureType = VkStructureType { bits: 1000165004 };
        pub const VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV: VkStructureType = VkStructureType { bits: 1000165005 };
        pub const VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: VkStructureType = VkStructureType { bits: 1000165006 };
        pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: VkStructureType = VkStructureType { bits: 1000165007 };
        pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: VkStructureType = VkStructureType { bits: 1000165008 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: VkStructureType = VkStructureType { bits: 1000165009 };
        pub const VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000165011 };
        pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV: VkStructureType = VkStructureType { bits: 1000165012 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000166000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000166001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: VkStructureType = VkStructureType { bits: 1000170000 };
        pub const VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000170001 };
        pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000174000 };
        pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT: VkStructureType = VkStructureType { bits: 1000178000 };
        pub const VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000178001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000178002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: VkStructureType = VkStructureType { bits: 1000181000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: VkStructureType = VkStructureType { bits: 1000183000 };
        pub const VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT: VkStructureType = VkStructureType { bits: 1000184000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: VkStructureType = VkStructureType { bits: 1000185000 };
        pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: VkStructureType = VkStructureType { bits: 1000189000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000190000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000190001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000190002 };
        pub const VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP: VkStructureType = VkStructureType { bits: 1000191000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000192000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000201000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000202000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: VkStructureType = VkStructureType { bits: 1000202001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000203000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000204000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000205000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000205002 };
        pub const VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV: VkStructureType = VkStructureType { bits: 1000206000 };
        pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: VkStructureType = VkStructureType { bits: 1000206001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: VkStructureType = VkStructureType { bits: 1000209000 };
        pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO_INTEL: VkStructureType = VkStructureType { bits: 1000210000 };
        pub const VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL: VkStructureType = VkStructureType { bits: 1000210001 };
        pub const VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL: VkStructureType = VkStructureType { bits: 1000210002 };
        pub const VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL: VkStructureType = VkStructureType { bits: 1000210003 };
        pub const VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL: VkStructureType = VkStructureType { bits: 1000210004 };
        pub const VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: VkStructureType = VkStructureType { bits: 1000210005 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000212000 };
        pub const VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: VkStructureType = VkStructureType { bits: 1000213000 };
        pub const VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: VkStructureType = VkStructureType { bits: 1000213001 };
        pub const VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: VkStructureType = VkStructureType { bits: 1000214000 };
        pub const VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000217000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000218000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000218001 };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000218002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000225000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000225001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000225002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: VkStructureType = VkStructureType { bits: 1000227000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: VkStructureType = VkStructureType { bits: 1000229000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000237000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000238000 };
        pub const VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000238001 };
        pub const VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR: VkStructureType = VkStructureType { bits: 1000239000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000240000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000244000 };
        pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000244002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000245000 };
        pub const VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000247000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000249000 };
        pub const VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV: VkStructureType = VkStructureType { bits: 1000249001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: VkStructureType = VkStructureType { bits: 1000249002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: VkStructureType = VkStructureType { bits: 1000250000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: VkStructureType = VkStructureType { bits: 1000250001 };
        pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: VkStructureType = VkStructureType { bits: 1000250002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000251000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000252000 };
        pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000255000 };
        pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: VkStructureType = VkStructureType { bits: 1000255002 };
        pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: VkStructureType = VkStructureType { bits: 1000255001 };
        pub const VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000256000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000259000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: 1000259001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000259002 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000265000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: VkStructureType = VkStructureType { bits: 1000269000 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000269001 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: 1000269002 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR: VkStructureType = VkStructureType { bits: 1000269003 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR: VkStructureType = VkStructureType { bits: 1000269004 };
        pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: VkStructureType = VkStructureType { bits: 1000269005 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000276000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: VkStructureType = VkStructureType { bits: 1000281000 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: 1000281001 };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT.bits };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2.bits };
        pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2.bits };
        pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2.bits };
        pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2.bits };
        pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2.bits };
        pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO.bits };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO.bits };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO.bits };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO.bits };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO.bits };
        pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO.bits };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO.bits };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO.bits };
        pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2.bits };
        pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2.bits };
        pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2.bits };
        pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2.bits };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2.bits };
        pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO.bits };
        pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SUBPASS_END_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS.bits };
        pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2.bits };
        pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2.bits };
        pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2.bits };
        pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2.bits };
        pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2.bits };
        pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO.bits };
        pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO.bits };
        pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO.bits };
        pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO.bits };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES.bits };
        pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO.bits };
        pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO.bits };
        pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT.bits };
        pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT.bits };
        pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO.bits };
        pub const VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO.bits };
        pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO.bits };
        pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO.bits };
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: VkStructureType = VkStructureType { bits: Self::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES.bits };
        pub const VK_STRUCTURE_TYPE_MAX_ENUM: VkStructureType = VkStructureType { bits: 0x7FFFFFFF };
    }

    impl From<i32> for VkStructureType {
        fn from(bits: i32) -> Self {
            Self { bits }
        }
    }

    impl Into<i32> for VkStructureType {
        fn into(self) -> i32 {
            self.bits
        }
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct VkSystemAllocationScope {
        pub bits: i32,
    }

    impl VkSystemAllocationScope {
        pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: VkSystemAllocationScope = VkSystemAllocationScope { bits: 0 };
        pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: VkSystemAllocationScope = VkSystemAllocationScope { bits: 1 };
        pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: VkSystemAllocationScope = VkSystemAllocationScope { bits: 2 };
        pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: VkSystemAllocationScope = VkSystemAllocationScope { bits: 3 };
        pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: VkSystemAllocationScope = VkSystemAllocationScope { bits: 4 };
        pub const VK_SYSTEM_ALLOCATION_SCOPE_MAX_ENUM: VkSystemAllocationScope = VkSystemAllocationScope { bits: 0x7FFFFFFF };
    }

    impl From<i32> for VkSystemAllocationScope {
        fn from(bits: i32) -> Self {
            Self { bits }
        }
    }

    impl Into<i32> for VkSystemAllocationScope {
        fn into(self) -> i32 {
            self.bits
        }
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct VkInternalAllocationType {
        pub bits: i32,
    }

    impl VkInternalAllocationType {
        pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: VkInternalAllocationType = VkInternalAllocationType { bits: 0 };
        pub const VK_INTERNAL_ALLOCATION_TYPE_MAX_ENUM: VkInternalAllocationType = VkInternalAllocationType { bits: 0x7FFFFFFF };
    }

    impl From<i32> for VkInternalAllocationType {
        fn from(bits: i32) -> Self {
            Self { bits }
        }
    }

    impl Into<i32> for VkInternalAllocationType {
        fn into(self) -> i32 {
            self.bits
        }
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct VkSampleCountFlagBits {
        pub bits: i32,
    }

    impl VkSampleCountFlagBits {
        pub const VK_SAMPLE_COUNT_1_BIT: i32 = 0x00000001;
        pub const VK_SAMPLE_COUNT_2_BIT: i32 = 0x00000002;
        pub const VK_SAMPLE_COUNT_4_BIT: i32 = 0x00000004;
        pub const VK_SAMPLE_COUNT_8_BIT: i32 = 0x00000008;
        pub const VK_SAMPLE_COUNT_16_BIT: i32 = 0x00000010;
        pub const VK_SAMPLE_COUNT_32_BIT: i32 = 0x00000020;
        pub const VK_SAMPLE_COUNT_64_BIT: i32 = 0x00000040;
        pub const VK_SAMPLE_COUNT_FLAG_BITS_MAX_ENUM: i32 = 0x7FFFFFFF;
    }

    impl From<i32> for VkSampleCountFlagBits {
        fn from(bits: i32) -> Self {
            Self { bits }
        }
    }

    impl Into<i32> for VkSampleCountFlagBits {
        fn into(self) -> i32 {
            self.bits
        }
    }

    impl From<i32> for VkVendorId {
        fn from(bits: i32) -> Self {
            match bits {
                0x10000 => Self::VK_VENDOR_ID_KHRONOS,
                0x10001 => Self::VK_VENDOR_ID_VIV,
                0x10002 => Self::VK_VENDOR_ID_VSI,
                0x10003 => Self::VK_VENDOR_ID_KAZAN,
                0x10004 => Self::VK_VENDOR_ID_CODEPLAY,
                0x10005 => Self::VK_VENDOR_ID_MESA,
                0x10006 => Self::VK_VENDOR_ID_POCL,
                0x10007 => Self::VK_VENDOR_ID_MOBILEYE,
                _ => Self::UNKNOWN
            }
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub enum VkVendorId {
        VK_VENDOR_ID_KHRONOS = 0x10000,
        VK_VENDOR_ID_VIV = 0x10001,
        VK_VENDOR_ID_VSI = 0x10002,
        VK_VENDOR_ID_KAZAN = 0x10003,
        VK_VENDOR_ID_CODEPLAY = 0x10004,
        VK_VENDOR_ID_MESA = 0x10005,
        VK_VENDOR_ID_POCL = 0x10006,
        VK_VENDOR_ID_MOBILEYE = 0x10007,
        UNKNOWN = 0x00000,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub enum VkPhysicalDeviceType {
        VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,
        VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
        VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
        VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
        VK_PHYSICAL_DEVICE_TYPE_CPU = 4,
    }

    impl Into<i32> for VkVendorId {
        fn into(self) -> i32 {
            match self {
                VkVendorId::VK_VENDOR_ID_KHRONOS => 0x10000,
                VkVendorId::VK_VENDOR_ID_VIV => 0x10001,
                VkVendorId::VK_VENDOR_ID_VSI => 0x10002,
                VkVendorId::VK_VENDOR_ID_KAZAN => 0x10003,
                VkVendorId::VK_VENDOR_ID_CODEPLAY => 0x10004,
                VkVendorId::VK_VENDOR_ID_MESA => 0x10005,
                VkVendorId::VK_VENDOR_ID_POCL => 0x10006,
                VkVendorId::VK_VENDOR_ID_MOBILEYE => 0x10007,
                VkVendorId::UNKNOWN => 0x00000
            }
        }
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct VkPointClippingBehavior {
        pub bits: i32,
    }

    impl VkPointClippingBehavior {
        pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: i32 = 0;
        pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: i32 = 1;
        pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR: i32 =
            Self::VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES;
        pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR: i32 =
            Self::VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY;
    }

    impl From<i32> for VkPointClippingBehavior {
        fn from(bits: i32) -> Self {
            Self { bits }
        }
    }

    impl Into<i32> for VkPointClippingBehavior {
        fn into(self) -> i32 {
            self.bits
        }
    }


    /// Khronos driver IDs
    ///
    /// # todo
    /// Provide documentation
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkDriverId {
        pub bits: i32
    }

    impl VkDriverId {
        pub const VK_DRIVER_ID_AMD_PROPRIETARY: VkDriverId = VkDriverId { bits: 1 };
        pub const VK_DRIVER_ID_AMD_OPEN_SOURCE: VkDriverId = VkDriverId { bits: 2 };
        pub const VK_DRIVER_ID_MESA_RADV: VkDriverId = VkDriverId { bits: 3 };
        pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY: VkDriverId = VkDriverId { bits: 4 };
        pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: VkDriverId = VkDriverId { bits: 5 };
        pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA: VkDriverId = VkDriverId { bits: 6 };
        pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY: VkDriverId = VkDriverId { bits: 7 };
        pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY: VkDriverId = VkDriverId { bits: 8 };
        pub const VK_DRIVER_ID_ARM_PROPRIETARY: VkDriverId = VkDriverId { bits: 9 };
        pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER: VkDriverId = VkDriverId { bits: 10 };
        pub const VK_DRIVER_ID_GGP_PROPRIETARY: VkDriverId = VkDriverId { bits: 11 };
        pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY: VkDriverId = VkDriverId { bits: 12 };
        pub const VK_DRIVER_ID_MESA_LLVMPIPE: VkDriverId = VkDriverId { bits: 13 };
        pub const VK_DRIVER_ID_MOLTENVK: VkDriverId = VkDriverId { bits: 14 };
        pub const VK_DRIVER_ID_COREAVI_PROPRIETARY: VkDriverId = VkDriverId { bits: 15 };
        pub const VK_DRIVER_ID_JUICE_PROPRIETARY: VkDriverId = VkDriverId { bits: 16 };
        pub const VK_DRIVER_ID_VERISILICON_PROPRIETARY: VkDriverId = VkDriverId { bits: 17 };
        pub const VK_DRIVER_ID_MESA_TURNIP: VkDriverId = VkDriverId { bits: 18 };
        pub const VK_DRIVER_ID_MESA_V3DV: VkDriverId = VkDriverId { bits: 19 };
        pub const VK_DRIVER_ID_MESA_PANVK: VkDriverId = VkDriverId { bits: 20 };
        pub const VK_DRIVER_ID_SAMSUNG_PROPRIETARY: VkDriverId = VkDriverId { bits: 21 };
        pub const VK_DRIVER_ID_MESA_VENUS: VkDriverId = VkDriverId { bits: 22 };
        pub const VK_DRIVER_ID_MESA_DOZEN: VkDriverId = VkDriverId { bits: 23 };
        pub const VK_DRIVER_ID_MESA_NVK: VkDriverId = VkDriverId { bits: 24 };
        pub const VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA: VkDriverId = VkDriverId { bits: 25 };
        pub const VK_DRIVER_ID_MESA_HONEYKRISP: VkDriverId = VkDriverId { bits: 26 };
        pub const VK_DRIVER_ID_VULKAN_SC_EMULATION_ON_VULKAN: VkDriverId = VkDriverId { bits: 27 };
        pub const VK_DRIVER_ID_AMD_PROPRIETARY_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_AMD_PROPRIETARY.bits };
        pub const VK_DRIVER_ID_AMD_OPEN_SOURCE_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_AMD_OPEN_SOURCE.bits };
        pub const VK_DRIVER_ID_MESA_RADV_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_MESA_RADV.bits };
        pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_NVIDIA_PROPRIETARY.bits };
        pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS.bits };
        pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA.bits };
        pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_IMAGINATION_PROPRIETARY.bits };
        pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_QUALCOMM_PROPRIETARY.bits };
        pub const VK_DRIVER_ID_ARM_PROPRIETARY_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_ARM_PROPRIETARY.bits };
        pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_GOOGLE_SWIFTSHADER.bits };
        pub const VK_DRIVER_ID_GGP_PROPRIETARY_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_GGP_PROPRIETARY.bits };
        pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY_KHR: VkDriverId = VkDriverId { bits: Self::VK_DRIVER_ID_BROADCOM_PROPRIETARY.bits };
    }

    impl From<i32> for VkDriverId {
        fn from(bits: i32) -> Self {
            Self { bits }
        }
    }

    impl Into<i32> for VkDriverId {
        fn into(self) -> i32 {
            self.bits
        }
    }


    #[repr(C)]
    #[derive(Debug)]
    pub enum VkLayeredDriverUnderlyingApiMSFT {
        VK_LAYERED_DRIVER_UNDERLYING_API_NONE_MSFT = 0,
        VK_LAYERED_DRIVER_UNDERLYING_API_D3D12_MSFT = 1,

    }

    #[repr(C)]
    #[derive(Debug)]
    pub enum VkQueueFlagBits {
        K_QUEUE_GRAPHICS_BIT = 0x00000001,
        VK_QUEUE_COMPUTE_BIT = 0x00000002,
        VK_QUEUE_TRANSFER_BIT = 0x00000004,
        VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008,
        /// Provided by VK_VERSION_1_1
        VK_QUEUE_PROTECTED_BIT = 0x00000010,
        /// Provided by VK_KHR_video_decode_queue
        VK_QUEUE_VIDEO_DECODE_BIT_KHR = 0x00000020,
        /// Provided by VK_KHR_video_encode_queue
        VK_QUEUE_VIDEO_ENCODE_BIT_KHR = 0x00000040,
        /// Provided by VK_NV_optical_flow
        VK_QUEUE_OPTICAL_FLOW_BIT_NV = 0x00000100,
    }
}

#[allow(non_snake_case, non_camel_case_types)]
pub mod offsets {
    /// Represents a 2D offset.
    #[repr(C)]
    pub struct VkOffset2D {
        x: i32,
        y: i32,
    }

    /// Represents a 3D offset.
    #[repr(C)]
    pub struct VkOffset3D {
        x: i32,
        y: i32,
        z: i32,
    }
}
#[allow(non_snake_case, non_camel_case_types)]

pub mod extents {
    /// Represents a 2D extent.
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkExtent2D {
        width: u32,
        height: u32,
    }

    /// Represents a 3D extent.
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkExtent3D {
        width: u32,
        height: u32,
        depth: u32,
    }
}

#[allow(non_snake_case, non_camel_case_types)]
pub mod rect {
    //! Contains type definitions for rectangle objects.
    use crate::ffi::c_types::{
        extents::VkExtent2D,
        offsets::VkOffset2D,
    };

    /// Represents a 2D rectangle.
    #[repr(C)]
    pub struct VkRect2D {
        offset: VkOffset2D,
        extent: VkExtent2D,
    }
}

#[allow(non_snake_case, non_camel_case_types)]
pub mod fn_ptrs {
    //! Contains function pointers that may or may not conform with Vulkan's API, but are
    //! The function pointers not defined in the Vulkan API are necessary for FFI
    //! compatability between C and Rust.
    //!
    //! Due to the strict typing within Rust, creating a function pointer that supports an
    //! arbitrary amount of arguments in the function's signature is not possible. Instead, we
    //! need to create function pointer types for all different possible pointers.
    //!
    //! For simplicity, despite this being non-C code, these type definitions will try to follow
    //! the Vulkan naming conventions as closely as possible. They will also be instead prefixed
    //! with `PFN_vkrs`, instead of `PFN_vk`.
    use std::ffi::c_void;

    use libc::size_t;

    use crate::ffi::c_types::enums::{VkInternalAllocationType, VkResult, VkSystemAllocationScope};

    /// Macro to simplify creation of function pointers and extern functions which call the
    /// Vulkan API.
    macro_rules! vkrs_create_function_with_typedef {
            (
                $func_name:ident,
                $ptr_name:ident,
                ($($arg_name:ident : $arg_type:ty),*)
                -> $ret_type:ty
            ) => {
                pub type $ptr_name = unsafe extern "system" fn($($arg_name: $arg_type),*) -> $ret_type;

                unsafe extern "C" {
                    pub fn $func_name($($arg_name: $arg_type),*) -> $ret_type;
                }
            };
        }

    /// A standard function pointer for use with the `vkGetProcAddrInfo` function.
    ///
    /// This type represents a generic function pointer within Vulkan.
    ///
    /// This type is mostly a base type used as a placeholder for casting to other function pointer
    /// types.
    pub type PFN_vkVoidFunction = unsafe extern "system" fn();

    /// Application-defined memory allocation function
    ///
    /// # Parameters
    /// - `pUserData` is the value specified for `VkAllocationCallbacks::pUserData` in the
    ///   allocator specified by the application.
    ///
    /// - `size` is the size in bytes of the requested allocation.
    ///
    /// - `alignment` is the requested alignment of the allocation in bytes and
    ///   must be a power of two.
    ///
    /// - `allocationScope` is a `VkSystemAllocationScope` value specifying the allocation scope of the
    ///   lifetime of the allocation, as described here.
    ///
    /// # Description
    /// If pfnAllocation is unable to allocate the requested memory, it must return NULL. If the
    /// allocation was successful, it must return a valid pointer to memory allocation containing at
    /// least size bytes, and with the pointer value being a multiple of alignment.
    ///
    /// > *Note*
    /// > Correct Vulkan operation cannot be assumed if the application does not follow these rules.
    /// >
    /// > For example, pfnAllocation (or pfnReallocation) could cause termination of running Vulkan
    /// > instance(s) on a failed allocation for debugging purposes, either directly or indirectly.
    /// > In these circumstances, it cannot be assumed that any part of any affected VkInstance
    /// > objects are going to operate correctly (even vkDestroyInstance), and the application must
    /// > ensure it cleans up properly via other means (e.g. process termination).
    ///
    /// If pfnAllocation returns NULL, and if the implementation is unable to continue correct
    /// processing of the current command without the requested allocation, it must treat this as a
    /// run-time error, and generate VK_ERROR_OUT_OF_HOST_MEMORY at the appropriate time for the
    /// command in which the condition was detected, as described in Return Codes.
    ///
    /// If the implementation is able to continue correct processing of the current command without
    /// the requested allocation, then it may do so, and must not generate
    /// VK_ERROR_OUT_OF_HOST_MEMORY as a result of this failed allocation.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#PFN_vkAllocationFunction
    pub type PFN_vkAllocationFunction = unsafe fn(
        pUserData: *mut c_void,
        size: size_t,
        alignment: size_t,
        allocationScope: VkSystemAllocationScope,
    );

    /// Application-defined memory reallocation function
    ///
    /// # Parameters
    /// - pUserData is the value specified for VkAllocationCallbacks::pUserData in the allocator
    /// specified by the application.
    ///
    /// - pOriginal must be either NULL or a pointer previously returned by pfnReallocation or
    /// pfnAllocation of a compatible allocator.
    ///
    /// - size is the size in bytes of the requested allocation.
    ///
    /// - alignment is the requested alignment of the allocation in bytes and must be a power of two.
    ///
    /// - allocationScope is a VkSystemAllocationScope value specifying the allocation scope of the
    /// lifetime of the allocation, as described here.
    ///
    /// # Description
    /// pfnReallocation must return an allocation with enough space for size bytes, and the contents
    /// of the original allocation from bytes zero to min(original size, new size) - 1 must be
    /// preserved in the returned allocation. If size is larger than the old size, the contents of
    /// the additional space are undefined. If satisfying these requirements involves creating a new
    /// allocation, then the old allocation should be freed.
    ///
    /// If pOriginal is NULL, then pfnReallocation must behave equivalently to a call to
    /// PFN_vkAllocationFunction with the same parameter values (without pOriginal).
    ///
    /// If size is zero, then pfnReallocation must behave equivalently to a call to
    /// PFN_vkFreeFunction with the same pUserData parameter value, and pMemory equal to pOriginal.
    ///
    /// If pOriginal is non-NULL, the implementation must ensure that alignment is equal to the
    /// alignment used to originally allocate pOriginal.
    ///
    /// If this function fails and pOriginal is non-NULL the application must not free the old
    /// allocation.
    ///
    /// pfnReallocation must follow the same rules for return values as PFN_vkAllocationFunction.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#PFN_vkReallocationFunction
    pub type PFN_vkReallocationFunction = unsafe fn(
        pUserData: *mut c_void,
        pOriginal: *mut c_void,
        size: size_t,
        alignment: size_t,
        allocationScope: VkSystemAllocationScope,
    );

    /// Application-defined memory free function
    ///
    /// # Parameters
    /// - pUserData is the value specified for `VkAllocationCallbacks::pUserData` in the allocator
    ///   specified by the application.
    ///
    /// - pMemory is the allocation to be freed.
    ///
    /// # Description
    /// pMemory may be NULL, which the callback must handle safely. If pMemory is non-NULL, it must
    /// be a pointer previously allocated by pfnAllocation or pfnReallocation. The application
    /// should free this memory.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#PFN_vkFreeFunction
    pub type PFN_vkFreeFunction = unsafe fn(
        pUserData: *mut c_void,
        pMemory: *mut c_void,
    );

    /// Application-defined memory allocation notification function
    ///
    /// # Parameters
    /// - pUserData is the value specified for VkAllocationCallbacks::pUserData in the allocator
    ///   specified by the application.
    ///
    /// - size is the requested size of an allocation.
    ///
    /// - allocationType is a VkInternalAllocationType value specifying the requested type of an
    ///   allocation.
    ///
    /// - allocationScope is a VkSystemAllocationScope value specifying the allocation scope of the
    ///   lifetime of the allocation, as described here.
    ///
    /// # Description
    /// This is a purely informational callback.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#PFN_vkInternalAllocationNotification
    pub type PFN_vkInternalAllocationNotification = unsafe fn(
        pUserData: *mut c_void,
        size: size_t,
        allocationType: VkInternalAllocationType,
        allocationScope: VkSystemAllocationScope,
    );

    /// Application-defined memory free notification function
    ///
    /// # Parameters
    /// - pUserData is the value specified for VkAllocationCallbacks::pUserData in the allocator
    ///   specified by the application.
    ///
    /// - size is the requested size of an allocation.
    ///
    /// - allocationType is a VkInternalAllocationType value specifying the requested type of an
    ///   allocation.
    ///
    /// - allocationScope is a VkSystemAllocationScope value specifying the allocation scope of the
    ///   lifetime of the allocation, as described here.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#PFN_vkInternalFreeNotification
    pub type PFN_vkInternalFreeNotification = fn(
        pUserData: *mut c_void,
        size: size_t,
        allocationType: VkInternalAllocationType,
        allocationScope: VkSystemAllocationScope,
    );

    // Create the definitions for vkEnumerateInstanceVersion.
    vkrs_create_function_with_typedef!(
        vkEnumerateInstanceVersion,
        PFN_vkEnumerateInstanceVersion,
        (
            p_api_version: *mut u32
        ) -> VkResult
    );
}

#[allow(non_snake_case, non_camel_case_types)]
pub mod objects {
    use std::ffi::c_void;

    use libc::c_char;
    use paste;

    use super::*;
    use crate::ffi::c_types::enums::*;
    use crate::ffi::c_types::extents::{VkExtent2D, VkExtent3D};
    use crate::ffi::c_types::fn_ptrs::*;
    use crate::ffi::constants::*;

    /// Equivalent to the C-Style `VK_DEFINE_HANDLE`. This creates a struct with the provided
    /// name, and a type which is a mutable pointer to the underlying struct.
    ///
    /// Each struct is assigned one member, `_nul`. This is because empty structs are not safe to
    /// pass through FFI, since the size of them cannot be determined. Giving each struct one member
    /// which takes up zero allows the compiler to understand what size the struct needs to be.
    macro_rules! vk_define_handle {
        ($name:ident) => {
            paste::paste! {
                #[repr(C)]
                #[derive(Debug)]
                pub struct [<$name _T>] {
                    _nul: [u8; 0]
                }

                pub type $name = *mut [<$name _T>];
            }
        };
    }

    // Define handles.
    vk_define_handle!(VkInstance);
    vk_define_handle!(VkDevice);
    vk_define_handle!(VkPhysicalDevice);
    vk_define_handle!(VkEvent);

    /// Object which specifies the current application info.
    ///
    /// sType must be VK_STRUCTURE_TYPE_APPLICATION_INFO
    #[repr(C)]
    pub struct VkApplicationInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub pApplicationName: *const c_char,
        pub applicationVersion: u32,
        pub pEngineName: *const c_char,
        pub engineVersion: u32,
        pub apiVersion: u32,
    }

    /// Object which specifies the parameters of a newly created instance.
    ///
    /// sType must be VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkInstanceCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkInstanceCreateFlags,
        pub pApplicationInfo: *const VkApplicationInfo,
        pub enabledLayerCount: u32,
        pub ppEnabledLayerNames: *const *const core::ffi::c_char,
        pub enabledExtensionCount: u32,
        pub ppEnabledExtensionNames: *const *const c_char,
    }

    /// Structure specifying parameters of a newly created device queue
    ///
    /// sType must be VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO
    ///
    /// # Members
    /// - sType is the type of this structure.
    ///
    /// - pNext is NULL or a pointer to an extension-specific structure.
    ///
    /// - flags is a bitmask indicating behavior of the queue.
    ///
    /// - queueFamilyIndex is an unsigned integer indicating the index of the queue family to create
    ///   on this device. This index corresponds to the index of an element of the
    ///   pQueueFamilyProperties array that was returned by vkGetPhysicalDeviceQueueFamilyProperties.
    ///
    /// - queueCount is an unsigned integer specifying the number of queues to create in the queue
    ///   family indicated by queueFamilyIndex.
    ///
    /// - pQueuePriorities is a pointer to an array of queueCount normalized floating point values,
    ///   specifying priorities of work that will be submitted to each created queue. See
    ///   Queue Priority for more information.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#_vkdevicequeuecreateinfo3
    #[repr(C)]
    pub struct VkDeviceQueueCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkDeviceQueueCreateFlags,
        pub queueFamilyIndex: u32,
        pub queueCount: u32,
        pub pQueueProperties: *const f32,
    }

    /// Structure describing the fine-grained features that can be supported by an implementation.
    ///
    /// *see each struct member for details*
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#VkPhysicalDeviceFeatures
    #[repr(C)]
    pub struct VkPhysicalDeviceFeatures {
        /// `robustBufferAccess` specifies that accesses to buffers are bounds-checked against the
        /// range of the buffer descriptor (as determined by `VkDescriptorBufferInfo::range`,
        /// `VkBufferViewCreateInfo::range`, or the size of the buffer). Out of bounds accesses must
        /// not cause application termination, and the effects of shader loads, stores, and atomics
        /// must conform to an implementation-dependent behavior as described below
        ///
        /// A buffer access is considered to be out of bounds if any of the following are true:
        ///
        /// - The pointer was formed by OpImageTexelPointer and the coordinate is less than zero or
        /// greater than or equal to the number of whole elements in the bound range.
        ///
        /// - The pointer was not formed by OpImageTexelPointer and the object pointed to is not
        ///   wholly contained within the bound range. This includes accesses performed via variable
        ///   pointers where the buffer descriptor being accessed cannot be statically determined.
        ///   Uninitialized pointers and pointers equal to OpConstantNull are treated as pointing to
        ///   a zero-sized object, so all accesses through such pointers are considered to be out of
        ///   bounds. Buffer accesses through buffer device addresses are not bounds-checked. If the
        ///   cooperativeMatrixRobustBufferAccess feature is not enabled, then accesses using
        ///   OpCooperativeMatrixLoadNV and OpCooperativeMatrixStoreNV may not be bounds-checked.
        ///
        /// > *Note*
        /// > If a SPIR-V OpLoad instruction loads a structure and the tail end of the structure is
        ///   out of bounds, then all members of the structure are considered out of bounds even if
        ///   the members at the end are not statically used.
        ///
        /// - If any buffer access is determined to be out of bounds, then any other access of the
        ///   same type (load, store, or atomic) to the same buffer that accesses an address less
        ///   than 16 bytes away from the out of bounds address may also be considered out of bounds.
        robustBufferAccess: VkBool32,

        /// `fullDrawIndexUint32` specifies the full 32-bit range of indices is supported for indexed
        /// draw calls when using a `VkIndexType` of `VK_INDEX_TYPE_UINT32`. `maxDrawIndexedIndexValue`
        /// is the maximum index value that may be used (aside from the primitive restart index,
        /// which is always 232-1 when the `VkIndexType` is `VK_INDEX_TYPE_UINT32`). If this feature
        /// is supported, `maxDrawIndexedIndexValue` must be 232-1; otherwise it must be no smaller
        /// than 224-1. See `maxDrawIndexedIndexValue`.
        fullDrawIndexUint32: VkBool32,

        /// `imageCubeArray` specifies whether image views with a `VkImageViewType` of
        /// `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY` can be created, and that the corresponding
        /// SampledCubeArray and ImageCubeArray SPIR-V capabilities can be used in shader code.
        imageCubeArray: VkBool32,

        /// `independentBlend` specifies whether the `VkPipelineColorBlendAttachmentState` settings are
        /// controlled independently per-attachment. If this feature is not enabled, the
        /// `VkPipelineColorBlendAttachmentState` settings for all color attachments must be
        /// identical. Otherwise, a different `VkPipelineColorBlendAttachmentState` can be provided
        /// for each bound color attachment.
        independentBlend: VkBool32,

        /// `geometryShader` specifies whether geometry shaders are supported. If this feature is not
        /// enabled, the `VK_SHADER_STAGE_GEOMETRY_BIT` and `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
        /// enum values must not be used. This also specifies whether shader modules can declare
        /// the Geometry capability.
        geometryShader: VkBool32,

        /// `tessellationShader` specifies whether tessellation control and evaluation shaders are
        /// supported. If this feature is not enabled, the `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`,
        /// `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`, `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT`,
        /// `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`, and
        /// `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO` enum values must not be used.
        /// This also specifies whether shader modules can declare the Tessellation capability.
        tessellationShader: VkBool32,

        /// `sampleRateShading` specifies whether Sample Shading and multisample interpolation are
        /// supported. If this feature is not enabled, the `sampleShadingEnable` member of the
        /// `VkPipelineMultisampleStateCreateInfo` structure must be set to `VK_FALSE` and the
        /// `minSampleShading` member is ignored. This also specifies whether shader modules can
        /// declare the SampleRateShading capability.
        sampleRateShading: VkBool32,

        /// `dualSrcBlend` specifies whether blend operations which take two sources are supported.
        /// If this feature is not enabled, the `VK_BLEND_FACTOR_SRC1_COLOR`,
        /// `VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`, `VK_BLEND_FACTOR_SRC1_ALPHA`, and
        /// `VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA` enum values must not be used as source or
        /// destination blending factors. See
        /// https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#framebuffer-dsb.
        dualSrcBlend: VkBool32,

        /// `logicOp` specifies whether logic operations are supported. If this feature is not
        /// enabled, the `logicOpEnable` member of the `VkPipelineColorBlendStateCreateInfo` structure
        /// must be set to `VK_FALSE`, and the `logicOp` member is ignored.
        logicOp: VkBool32,

        /// `multiDrawIndirect` specifies whether multiple draw indirect is supported. If this feature
        /// is not enabled, the drawCount parameter to the `vkCmdDrawIndirect` and
        /// `vkCmdDrawIndexedIndirect` commands must be 0 or 1. The `maxDrawIndirectCount` member of the
        /// `VkPhysicalDeviceLimits` structure must also be 1 if this feature is not supported. See
        /// `maxDrawIndirectCount`.
        multiDrawIndirect: VkBool32,

        /// `drawIndirectFirstInstance` specifies whether indirect draw calls support the
        /// `firstInstance` parameter. If this feature is not enabled, the `firstInstance` member of
        /// all `VkDrawIndirectCommand` and `VkDrawIndexedIndirectCommand` structures that are
        /// provided to the `vkCmdDrawIndirect` and `vkCmdDrawIndexedIndirect` commands must be 0.
        drawIndirectFirstInstance: VkBool32,

        /// `depthClamp` specifies whether depth clamping is supported. If this feature is not
        /// enabled, the `depthClampEnable` member of the `VkPipelineRasterizationStateCreateInfo`
        /// structure must be set to `VK_FALSE`. Otherwise, setting `depthClampEnable` to `VK_TRUE`
        /// will enable depth clamping.
        depthClamp: VkBool32,

        /// `depthBiasClamp` specifies whether depth bias clamping is supported. If this feature is
        /// not enabled, the `depthBiasClamp` member of the `VkPipelineRasterizationStateCreateInfo`
        /// structure must be set to 0.0 unless the `VK_DYNAMIC_STATE_DEPTH_BIAS` dynamic state is
        /// enabled, and the `depthBiasClamp` parameter to `vkCmdSetDepthBias` must be set to 0.0.
        depthBiasClamp: VkBool32,

        /// `fillModeNonSolid` specifies whether point and wireframe fill modes are supported.
        /// If this feature is not enabled, the `VK_POLYGON_MODE_POINT` and `VK_POLYGON_MODE_LINE`
        /// enum values must not be used.
        fillModeNonSolid: VkBool32,

        /// `depthBounds` specifies whether depth bounds tests are supported. If this feature is not
        /// enabled, the `depthBoundsTestEnable` member of the `VkPipelineDepthStencilStateCreateInfo`
        /// structure must be set to `VK_FALSE`. When `depthBoundsTestEnable` is set to `VK_FALSE`, the
        /// `minDepthBounds` and `maxDepthBounds` members of the `VkPipelineDepthStencilStateCreateInfo`
        /// structure are ignored.
        depthBounds: VkBool32,

        /// `wideLines` specifies whether lines with width other than 1.0 are supported. If this
        /// feature is not enabled, the `lineWidth` member of the
        /// `VkPipelineRasterizationStateCreateInfo` structure must be set to 1.0 unless the
        /// `VK_DYNAMIC_STATE_LINE_WIDTH` dynamic state is enabled, and the `lineWidth` parameter to
        /// `vkCmdSetLineWidth` must be set to 1.0. When this feature is supported, the range and
        /// granularity of supported line widths are indicated by the `lineWidthRange` and
        /// `lineWidthGranularity` members of the `VkPhysicalDeviceLimits` structure, respectively.
        wideLines: VkBool32,

        /// `largePoints` specifies whether points with size greater than 1.0 are supported. If this
        /// feature is not enabled, only a point size of 1.0 written by a shader is supported. The
        /// range and granularity of supported point sizes are indicated by the `pointSizeRange` and
        /// `pointSizeGranularity` members of the `VkPhysicalDeviceLimits` structure, respectively.
        largePoints: VkBool32,

        /// `alphaToOne` specifies whether the implementation is able to replace the alpha value of
        /// the color fragment output from the fragment shader with the maximum representable alpha
        /// value for fixed-point colors or 1.0 for floating-point colors. If this feature is not
        /// enabled, then the `alphaToOneEnable` member of the `VkPipelineMultisampleStateCreateInfo`
        /// structure must be set to `VK_FALSE`. Otherwise, setting `alphaToOneEnable` to VK_TRUE will
        /// enable alpha-to-one behavior.
        alphaToOne: VkBool32,

        /// multiViewport specifies whether more than one viewport is supported. If this feature is not enabled:
        ///
        /// - The viewportCount and scissorCount members of the VkPipelineViewportStateCreateInfo
        ///   structure must be set to 1.
        ///
        /// - The firstViewport and viewportCount parameters to the vkCmdSetViewport command must be
        ///   set to 0 and 1, respectively.
        ///
        /// - The firstScissor and scissorCount parameters to the vkCmdSetScissor command must be
        ///   set to 0 and 1, respectively.
        ///
        /// - The exclusiveScissorCount member of the VkPipelineViewportExclusiveScissorStateCreateInfoNV
        ///   structure must be set to 0 or 1.
        ///
        /// - The firstExclusiveScissor and exclusiveScissorCount parameters to the
        ///   vkCmdSetExclusiveScissorNV command must be set to 0 and 1, respectively.
        multiViewport: VkBool32,

        /// `samplerAnisotropy` specifies whether anisotropic filtering is supported. If this feature
        /// is not enabled, the `anisotropyEnable` member of the `VkSamplerCreateInfo` structure must be
        /// `VK_FALSE`.
        samplerAnisotropy: VkBool32,

        /// `textureCompressionETC2` specifies whether all of the ETC2 and EAC compressed texture formats
        /// are supported. If this feature is enabled, then the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`,
        /// `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
        /// features must be supported in `optimalTilingFeatures` for the following formats:
        ///
        /// - `VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_EAC_R11_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_EAC_R11_SNORM_BLOCK`
        ///
        /// - `VK_FORMAT_EAC_R11G11_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_EAC_R11G11_SNORM_BLOCK`
        ///
        /// To query for additional properties, or if the feature is not enabled,
        /// `vkGetPhysicalDeviceFormatProperties` and `vkGetPhysicalDeviceImageFormatProperties`
        /// can be used to check for supported properties of individual formats as normal.
        textureCompressionETC2: VkBool32,

        /// `textureCompressionASTC_LDR` specifies whether all of the ASTC LDR compressed texture
        /// formats are supported. If this feature is enabled, then the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`,
        /// `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
        /// features must be supported in `optimalTilingFeatures` for the following formats:
        ///
        /// - `VK_FORMAT_ASTC_4x4_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_4x4_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_5x4_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_5x4_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_5x5_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_5x5_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_6x5_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_6x5_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_6x6_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_6x6_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_8x5_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_8x5_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_8x6_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_8x6_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_8x8_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_8x8_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_10x5_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_10x5_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_10x6_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_10x6_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_10x8_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_10x8_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_10x10_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_10x10_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_12x10_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_12x10_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_12x12_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_ASTC_12x12_SRGB_BLOCK`
        ///
        /// To query for additional properties, or if the feature is not enabled,
        /// `vkGetPhysicalDeviceFormatProperties` and `vkGetPhysicalDeviceImageFormatProperties` can be
        /// used to check for supported properties of individual formats as normal.
        textureCompressionASTC_LDR: VkBool32,

        /// `textureCompressionBC` specifies whether all of the BC compressed texture formats are
        /// supported. If this feature is enabled, then the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`,
        /// `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
        /// features must be supported in `optimalTilingFeatures` for the following formats:
        ///
        /// - `VK_FORMAT_BC1_RGB_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_BC1_RGB_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_BC1_RGBA_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_BC1_RGBA_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_BC2_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_BC2_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_BC3_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_BC3_SRGB_BLOCK`
        ///
        /// - `VK_FORMAT_BC4_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_BC4_SNORM_BLOCK`
        ///
        /// - `VK_FORMAT_BC5_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_BC5_SNORM_BLOCK`
        ///
        /// - `VK_FORMAT_BC6H_UFLOAT_BLOCK`
        ///
        /// - `VK_FORMAT_BC6H_SFLOAT_BLOCK`
        ///
        /// - `VK_FORMAT_BC7_UNORM_BLOCK`
        ///
        /// - `VK_FORMAT_BC7_SRGB_BLOCK`
        ///
        /// To query for additional properties, or if the feature is not enabled,
        /// `vkGetPhysicalDeviceFormatProperties` and `vkGetPhysicalDeviceImageFormatProperties` can be
        /// used to check for supported properties of individual formats as normal.
        textureCompressionBC: VkBool32,

        /// `occlusionQueryPrecise` specifies whether occlusion queries returning actual sample counts
        /// are supported. Occlusion queries are created in a `VkQueryPool` by specifying the
        /// `queryType` of `VK_QUERY_TYPE_OCCLUSION` in the `VkQueryPoolCreateInfo` structure which is
        /// passed to `vkCreateQueryPool`. If this feature is enabled, queries of this type can enable
        /// `VK_QUERY_CONTROL_PRECISE_BIT` in the flags parameter to `vkCmdBeginQuery`. If this feature
        /// is not supported, the implementation supports only boolean occlusion queries. When any
        /// samples are passed, boolean queries will return a non-zero result value, otherwise a
        /// result value of zero is returned. When this feature is enabled and
        /// `VK_QUERY_CONTROL_PRECISE_BIT` is set, occlusion queries will report the actual
        /// number of samples passed.
        occlusionQueryPrecise: VkBool32,

        /// `pipelineStatisticsQuery` specifies whether the pipeline statistics queries are supported.
        /// If this feature is not enabled, queries of type `VK_QUERY_TYPE_PIPELINE_STATISTICS` cannot
        /// be created, and none of the `VkQueryPipelineStatisticFlagBits` bits can be set in the
        /// `pipelineStatistics` member of the `VkQueryPoolCreateInfo` structure.
        pipelineStatisticsQuery: VkBool32,

        /// `vertexPipelineStoresAndAtomics` specifies whether storage buffers and images support
        /// stores and atomic operations in the vertex, tessellation, and geometry shader stages.
        /// If this feature is not enabled, all storage image, storage texel buffers, and storage
        /// buffer variables used by these stages in shader modules must be decorated with the
        /// NonWritable decoration (or the readonly memory qualifier in GLSL).
        vertexPipelineStoresAndAtomics: VkBool32,

        /// `fragmentStoresAndAtomics` specifies whether storage buffers and images support stores
        /// and atomic operations in the fragment shader stage. If this feature is not enabled,
        /// all storage image, storage texel buffers, and storage buffer variables used by the
        /// fragment stage in shader modules must be decorated with the NonWritable decoration
        /// (or the readonly memory qualifier in GLSL).
        fragmentStoresAndAtomics: VkBool32,

        /// `shaderTessellationAndGeometryPointSize` specifies whether the PointSize built-in
        /// decoration is available in the tessellation control, tessellation evaluation, and
        /// geometry shader stages. If this feature is not enabled, members decorated with the
        /// PointSize built-in decoration must not be read from or written to and all points
        /// written from a tessellation or geometry shader will have a size of 1.0. This also
        /// specifies whether shader modules can declare the TessellationPointSize capability
        /// for tessellation control and evaluation shaders, or if the shader modules can declare
        /// the GeometryPointSize capability for geometry shaders. An implementation supporting
        /// this feature must also support one or both of the `tessellationShader` or `geometryShader`
        /// features.
        shaderTessellationAndGeometryPointSize: VkBool32,

        /// `shaderImageGatherExtended` specifies whether the extended set of image gather instructions
        /// are available in shader code. If this feature is not enabled, the `OpImage*Gather`
        /// instructions do not support the Offset and ConstOffsets operands. This also specifies
        /// whether shader modules can declare the ImageGatherExtended capability.
        shaderImageGatherExtended: VkBool32,

        /// `shaderStorageImageExtendedFormats` specifies whether all the “storage image extended
        /// formats” below are supported; if this feature is supported, then the
        /// `VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT` must be supported in `optimalTilingFeatures` for the
        /// following formats:
        ///
        /// - `VK_FORMAT_R16G16_SFLOAT`
        ///
        /// - `VK_FORMAT_B10G11R11_UFLOAT_PACK32`
        ///
        /// - `VK_FORMAT_R16_SFLOAT`
        ///
        /// - `VK_FORMAT_R16G16B16A16_UNORM`
        ///
        /// - `VK_FORMAT_A2B10G10R10_UNORM_PACK32`
        ///
        /// - `VK_FORMAT_R16G16_UNORM`
        ///
        /// - `VK_FORMAT_R8G8_UNORM`
        ///
        /// - `VK_FORMAT_R16_UNORM`
        ///
        /// - `VK_FORMAT_R8_UNORM`
        ///
        /// - `VK_FORMAT_R16G16B16A16_SNORM`
        ///
        /// - `VK_FORMAT_R16G16_SNORM`
        ///
        /// - `VK_FORMAT_R8G8_SNORM`
        ///
        /// - `VK_FORMAT_R16_SNORM`
        ///
        /// - `VK_FORMAT_R8_SNORM`
        ///
        /// - `VK_FORMAT_R16G16_SINT`
        ///
        /// - `VK_FORMAT_R8G8_SINT`
        ///
        /// - `VK_FORMAT_R16_SINT`
        ///
        /// - `VK_FORMAT_R8_SINT`
        ///
        /// - `VK_FORMAT_A2B10G10R10_UINT_PACK32`
        ///
        /// - `VK_FORMAT_R16G16_UINT`
        ///
        /// - `VK_FORMAT_R8G8_UINT`
        ///
        /// - `VK_FORMAT_R16_UINT`
        ///
        /// - `VK_FORMAT_R8_UINT`
        ///
        /// > *Note*
        /// > `shaderStorageImageExtendedFormats` feature only adds a guarantee of format support,
        /// > which is specified for the whole physical device. Therefore enabling or disabling the
        /// > feature via vkCreateDevice has no practical effect.
        ///
        /// To query for additional properties, or if the feature is not supported,
        /// `vkGetPhysicalDeviceFormatProperties` and `vkGetPhysicalDeviceImageFormatProperties` can be
        /// used to check for supported properties of individual formats, as usual rules allow.
        ///
        /// `VK_FORMAT_R32G32_UINT`, `VK_FORMAT_R32G32_SINT`, and `VK_FORMAT_R32G32_SFLOAT` from
        /// StorageImageExtendedFormats SPIR-V capability, are already covered by core Vulkan
        /// mandatory format support.
        shaderStorageImageExtendedFormats: VkBool32,

        /// `shaderStorageImageMultisample` specifies whether multisampled storage images are supported.
        /// If this feature is not enabled, images that are created with a usage that includes
        /// `VK_IMAGE_USAGE_STORAGE_BIT` must be created with samples equal to `VK_SAMPLE_COUNT_1_BIT`.
        /// This also specifies whether shader modules can declare the StorageImageMultisample
        /// capability.
        shaderStorageImageMultisample: VkBool32,

        /// shaderStorageImageReadWithoutFormat specifies whether storage images require a format
        /// qualifier to be specified when reading from storage images. If this feature is not enabled,
        /// the OpImageRead instruction must not have an OpTypeImage of Unknown. This also specifies
        /// whether shader modules can declare the StorageImageReadWithoutFormat capability.
        shaderStorageImageReadWithoutFormat: VkBool32,

        /// shaderStorageImageWriteWithoutFormat specifies whether storage images require a format
        /// qualifier to be specified when writing to storage images. If this feature is not enabled,
        /// the OpImageWrite instruction must not have an OpTypeImage of Unknown. This also specifies
        /// whether shader modules can declare the StorageImageWriteWithoutFormat capability.
        shaderStorageImageWriteWithoutFormat: VkBool32,

        /// `shaderUniformBufferArrayDynamicIndexing` specifies whether arrays of uniform buffers can
        /// be indexed by dynamically uniform integer expressions in shader code. If this feature
        /// is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
        /// `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` must be indexed only by constant integral
        /// expressions when aggregated into arrays in shader code. This also specifies whether
        /// shader modules can declare the UniformBufferArrayDynamicIndexing capability.
        shaderUniformBufferArrayDynamicIndexing: VkBool32,

        /// `shaderSampledImageArrayDynamicIndexing` specifies whether arrays of samplers or sampled
        /// images can be indexed by dynamically uniform integer expressions in shader code. If this
        /// feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_SAMPLER`,
        /// `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` must be
        /// indexed only by constant integral expressions when aggregated into arrays in shader code.
        /// This also specifies whether shader modules can declare the SampledImageArrayDynamicIndexing
        /// capability.
        shaderSampledImageArrayDynamicIndexing: VkBool32,

        /// `shaderStorageBufferArrayDynamicIndexing` specifies whether arrays of storage buffers can
        /// be indexed by dynamically uniform integer expressions in shader code. If this feature
        /// is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`
        /// or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` must be indexed only by constant integral
        /// expressions when aggregated into arrays in shader code. This also specifies whether
        /// shader modules can declare the StorageBufferArrayDynamicIndexing capability.
        shaderStorageBufferArrayDynamicIndexing: VkBool32,

        /// shaderStorageImageArrayDynamicIndexing specifies whether arrays of storage images can be
        /// indexed by dynamically uniform integer expressions in shader code. If this feature is not
        /// enabled, resources with a descriptor type of VK_DESCRIPTOR_TYPE_STORAGE_IMAGE must be
        /// indexed only by constant integral expressions when aggregated into arrays in shader code.
        /// This also specifies whether shader modules can declare the StorageImageArrayDynamicIndexing
        /// capability.
        shaderStorageImageArrayDynamicIndexing: VkBool32,

        /// shaderClipDistance specifies whether clip distances are supported in shader code. If this
        /// feature is not enabled, any members decorated with the ClipDistance built-in decoration
        /// must not be read from or written to in shader modules. This also specifies whether shader
        /// modules can declare the ClipDistance capability.
        shaderClipDistance: VkBool32,

        /// shaderCullDistance specifies whether cull distances are supported in shader code. If this
        /// feature is not enabled, any members decorated with the CullDistance built-in decoration
        /// must not be read from or written to in shader modules. This also specifies whether shader
        /// modules can declare the CullDistance capability.
        shaderCullDistance: VkBool32,

        /// shaderFloat64 specifies whether 64-bit floats (doubles) are supported in shader code. If
        /// this feature is not enabled, 64-bit floating-point types must not be used in shader code.
        /// This also specifies whether shader modules can declare the Float64 capability. Declaring
        /// and using 64-bit floats is enabled for all storage classes that SPIR-V allows with the
        /// Float64 capability.
        shaderFloat64: VkBool32,

        /// shaderInt64 specifies whether 64-bit integers (signed and unsigned) are supported in
        /// shader code. If this feature is not enabled, 64-bit integer types must not be used in
        /// shader code. This also specifies whether shader modules can declare the Int64 capability.
        /// Declaring and using 64-bit integers is enabled for all storage classes that SPIR-V
        /// allows with the Int64 capability.
        shaderInt64: VkBool32,

        /// shaderInt16 specifies whether 16-bit integers (signed and unsigned) are supported in
        /// shader code. If this feature is not enabled, 16-bit integer types must not be used in
        /// shader code. This also specifies whether shader modules can declare the Int16 capability.
        /// However, this only enables a subset of the storage classes that SPIR-V allows for the
        /// Int16 SPIR-V capability: Declaring and using 16-bit integers in the Private, Workgroup,
        /// and Function storage classes is enabled, while declaring them in the interface storage
        /// classes (e.g., UniformConstant, Uniform, StorageBuffer, Input, Output, and PushConstant)
        /// is not enabled.
        shaderInt16: VkBool32,

        /// shaderResourceResidency specifies whether image operations that return resource residency
        /// information are supported in shader code. If this feature is not enabled, the
        /// OpImageSparse* instructions must not be used in shader code. This also specifies
        /// whether shader modules can declare the SparseResidency capability. The feature requires
        /// at least one of the sparseResidency* features to be supported.
        shaderResourceResidency: VkBool32,

        /// shaderResourceMinLod specifies whether image operations specifying the minimum resource
        /// LOD are supported in shader code. If this feature is not enabled, the MinLod image
        /// operand must not be used in shader code. This also specifies whether shader modules can
        /// declare the MinLod capability.
        shaderResourceMinLod: VkBool32,

        /// sparseBinding specifies whether resource memory can be managed at opaque sparse block
        /// level instead of at the object level. If this feature is not enabled, resource memory
        /// must be bound only on a per-object basis using the vkBindBufferMemory and
        /// vkBindImageMemory commands. In this case, buffers and images must not be created with
        /// VK_BUFFER_CREATE_SPARSE_BINDING_BIT and VK_IMAGE_CREATE_SPARSE_BINDING_BIT set in the
        /// flags member of the VkBufferCreateInfo and VkImageCreateInfo structures, respectively.
        /// Otherwise, resource memory can be managed as described in Sparse Resource Features.
        sparseBinding: VkBool32,

        /// sparseResidencyBuffer specifies whether the device can access partially resident buffers.
        /// If this feature is not enabled, buffers must not be created with
        /// VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the VkBufferCreateInfo
        /// structure.
        sparseResidencyBuffer: VkBool32,

        /// sparseResidencyImage2D specifies whether the device can access partially resident 2D
        /// images with 1 sample per pixel. If this feature is not enabled, images with an imageType
        /// of VK_IMAGE_TYPE_2D and samples set to VK_SAMPLE_COUNT_1_BIT must not be created with
        /// VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the VkImageCreateInfo
        /// structure.
        sparseResidencyImage2D: VkBool32,

        /// sparseResidencyImage3D specifies whether the device can access partially resident 3D
        /// images. If this feature is not enabled, images with an imageType of VK_IMAGE_TYPE_3D
        /// must not be created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member
        /// of the VkImageCreateInfo structure.
        sparseResidencyImage3D: VkBool32,

        /// sparseResidency2Samples specifies whether the physical device can access partially
        /// resident 2D images with 2 samples per pixel. If this feature is not enabled, images
        /// with an imageType of VK_IMAGE_TYPE_2D and samples set to VK_SAMPLE_COUNT_2_BIT must not
        /// be created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the
        /// VkImageCreateInfo structure.
        sparseResidency2Samples: VkBool32,

        /// sparseResidency4Samples specifies whether the physical device can access partially
        /// resident 2D images with 4 samples per pixel. If this feature is not enabled, images with
        /// an imageType of VK_IMAGE_TYPE_2D and samples set to VK_SAMPLE_COUNT_4_BIT must not be
        /// created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the
        /// VkImageCreateInfo structure.
        sparseResidency4Samples: VkBool32,

        /// sparseResidency8Samples specifies whether the physical device can access partially
        /// resident 2D images with 8 samples per pixel. If this feature is not enabled, images with
        /// an imageType of VK_IMAGE_TYPE_2D and samples set to VK_SAMPLE_COUNT_8_BIT must not be
        /// created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the
        /// VkImageCreateInfo structure.
        sparseResidency8Samples: VkBool32,

        /// sparseResidency16Samples specifies whether the physical device can access partially
        /// resident 2D images with 16 samples per pixel. If this feature is not enabled, images
        /// with an imageType of VK_IMAGE_TYPE_2D and samples set to VK_SAMPLE_COUNT_16_BIT must
        /// not be created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the
        /// VkImageCreateInfo structure.
        sparseResidency16Samples: VkBool32,

        /// sparseResidencyAliased specifies whether the physical device can correctly access data
        /// aliased into multiple locations. If this feature is not enabled, the
        /// VK_BUFFER_CREATE_SPARSE_ALIASED_BIT and VK_IMAGE_CREATE_SPARSE_ALIASED_BIT enum values
        /// must not be used in flags members of the VkBufferCreateInfo and VkImageCreateInfo
        /// structures, respectively.
        sparseResidencyAliased: VkBool32,

        /// variableMultisampleRate specifies whether all pipelines that will be bound to a command
        /// buffer during a subpass with no attachments must have the same value for
        /// VkPipelineMultisampleStateCreateInfo::rasterizationSamples. If set to VK_TRUE, the
        /// implementation supports variable multisample rates in a subpass with no attachments. If
        /// set to VK_FALSE, then all pipelines bound in such a subpass must have the same
        /// multisample rate. This has no effect in situations where a subpass uses any attachments.
        variableMultisampleRate: VkBool32,

        /// inheritedQueries specifies whether a secondary command buffer may be executed while a
        /// query is active.
        inheritedQueries: VkBool32,

    }

    /// Structure specifying parameters of a newly created device
    ///
    /// sType must be VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO
    ///
    /// # Members
    /// - sType is the type of this structure.
    ///
    /// - pNext is NULL or a pointer to an extension-specific structure.
    ///
    /// - flags is reserved for future use.
    ///
    /// - queueCreateInfoCount is the unsigned integer size of the pQueueCreateInfos array. Refer to
    ///   the Queue Creation section below for further details.
    ///
    /// - pQueueCreateInfos is a pointer to an array of VkDeviceQueueCreateInfo structures
    ///   describing the queues that are requested to be created along with the logical device.
    ///   Refer to the Queue Creation section below for further details.
    ///
    /// - enabledLayerCount is deprecated and ignored.
    ///
    /// - ppEnabledLayerNames is deprecated and ignored. See
    ///   https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#extendingvulkan-layers-devicelayerdeprecation.
    ///
    /// - enabledExtensionCount is the number of device extensions to enable.
    ///
    /// - ppEnabledExtensionNames is a pointer to an array of enabledExtensionCount null-terminated
    ///   UTF-8 strings containing the names of extensions to enable for the created device. See the
    ///   https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#extendingvulkan-extensions
    ///   section for further details.
    ///
    /// - pEnabledFeatures is NULL or a pointer to a VkPhysicalDeviceFeatures structure containing
    ///   boolean indicators of all the features to be enabled. Refer to the Features section for
    ///   further details.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#_vkdevicecreateinfo3
    #[repr(C)]
    pub struct VkDeviceCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkDeviceCreateFlags,
        pub queueCreateInfoCount: u32,
        pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
        pub enabledLayerCount: u32,
        pub ppEnabledLayerNames: *const *const c_char,
        pub enabledExtensionCount: u32,
        pub ppEnabledExtensionNames: *const *const c_char,
        pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
    }

    /// Structure containing callback function pointers for memory allocation.
    ///
    /// Allocators are provided by the application as a pointer to a VkAllocationCallbacks structure.
    ///
    /// # Members
    ///
    /// - `pUserData` is a value to be interpreted by the implementation of the callbacks. When any of
    /// the callbacks in `VkAllocationCallbacks` are called, the Vulkan implementation will pass this
    /// value as the first parameter to the callback. This value can vary each time an allocator
    /// is passed into a command, even when the same object takes an allocator in multiple commands.
    ///
    /// - `pfnAllocation` is a `PFN_vkAllocationFunction` pointer to an application-defined memory
    /// allocation function.
    ///
    /// - `pfnReallocation` is a `PFN_vkReallocationFunction` pointer to an application-defined memory
    /// reallocation function.
    ///
    /// - `pfnFree` is a `PFN_vkFreeFunction` pointer to an application-defined memory free function.
    ///
    /// - `pfnInternalAllocation` is a `PFN_vkInternalAllocationNotification` pointer to an
    /// application-defined function that is called by the implementation when the implementation
    /// makes internal allocations.
    ///
    /// - `pfnInternalFree` is a `PFN_vkInternalFreeNotification` pointer to an application-defined
    /// function that is called by the implementation when the implementation frees internal
    /// allocations.
    #[repr(C)]
    pub struct VkAllocationCallbacks {
        pub pUserData: *mut c_void,
        pub pfnAllocation: PFN_vkAllocationFunction,
        pub pfnReallocation: PFN_vkReallocationFunction,
        pub pfnFree: PFN_vkFreeFunction,
        pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
        pub pfnInternalFree: PFN_vkInternalFreeNotification,
    }

    /// Structure specifying physical device sparse memory properties
    ///
    /// # todo
    /// Provide proper documentation for this struct.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#VkPhysicalDeviceSparseProperties
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceSparseProperties {
        pub residencyStandard2DBlockShape: VkBool32,
        pub residencyStandard2DMultisampleBlockShape: VkBool32,
        pub residencyStandard3DBlockShape: VkBool32,
        pub residencyAlignedMipSize: VkBool32,
        pub residencyNonResidentStrict: VkBool32,
    }

    /// Structure reporting implementation-dependent physical device limits
    ///
    /// # todo
    /// Insert proper documentation for this struct.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#VkPhysicalDeviceLimits
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceLimits {
        pub maxImageDimension1D: u32,
        pub maxImageDimension2D: u32,
        pub maxImageDimension3D: u32,
        pub maxImageDimensionCube: u32,
        pub maxImageArrayLayers: u32,
        pub maxTexelBufferElements: u32,
        pub maxUniformBufferRange: u32,
        pub maxStorageBufferRange: u32,
        pub maxPushConstantsSize: u32,
        pub maxMemoryAllocationCount: u32,
        pub maxSamplerAllocationCount: u32,
        pub bufferImageGranularity: VkDeviceSize,
        pub sparseAddressSpaceSize: VkDeviceSize,
        pub maxBoundDescriptorSets: u32,
        pub maxPerStageDescriptorSamplers: u32,
        pub maxPerStageDescriptorUniformBuffers: u32,
        pub maxPerStageDescriptorStorageBuffers: u32,
        pub maxPerStageDescriptorSampledImages: u32,
        pub maxPerStageDescriptorStorageImages: u32,
        pub maxPerStageDescriptorInputAttachments: u32,
        pub maxPerStageResources: u32,
        pub maxDescriptorSetSamplers: u32,
        pub maxDescriptorSetUniformBuffers: u32,
        pub maxDescriptorSetUniformBuffersDynamic: u32,
        pub maxDescriptorSetStorageBuffers: u32,
        pub maxDescriptorSetStorageBuffersDynamic: u32,
        pub maxDescriptorSetSampledImages: u32,
        pub maxDescriptorSetStorageImages: u32,
        pub maxDescriptorSetInputAttachments: u32,
        pub maxVertexInputAttributes: u32,
        pub maxVertexInputBindings: u32,
        pub maxVertexInputAttributeOffset: u32,
        pub maxVertexInputBindingStride: u32,
        pub maxVertexOutputComponents: u32,
        pub maxTessellationGenerationLevel: u32,
        pub maxTessellationPatchSize: u32,
        pub maxTessellationControlPerVertexInputComponents: u32,
        pub maxTessellationControlPerVertexOutputComponents: u32,
        pub maxTessellationControlPerPatchOutputComponents: u32,
        pub maxTessellationControlTotalOutputComponents: u32,
        pub maxTessellationEvaluationInputComponents: u32,
        pub maxTessellationEvaluationOutputComponents: u32,
        pub maxGeometryShaderInvocations: u32,
        pub maxGeometryInputComponents: u32,
        pub maxGeometryOutputComponents: u32,
        pub maxGeometryOutputVertices: u32,
        pub maxGeometryTotalOutputComponents: u32,
        pub maxFragmentInputComponents: u32,
        pub maxFragmentOutputAttachments: u32,
        pub maxFragmentDualSrcAttachments: u32,
        pub maxFragmentCombinedOutputResources: u32,
        pub maxComputeSharedMemorySize: u32,
        pub maxComputeWorkGroupCount: [u32; 3],
        pub maxComputeWorkGroupInvocations: u32,
        pub maxComputeWorkGroupSize: [u32; 3],
        pub subPixelPrecisionBits: u32,
        pub subTexelPrecisionBits: u32,
        pub mipmapPrecisionBits: u32,
        pub maxDrawIndexedIndexValue: u32,
        pub maxDrawIndirectCount: u32,
        pub maxSamplerLodBias: f32,
        pub maxSamplerAnisotropy: f32,
        pub maxViewports: u32,
        pub maxViewportDimensions: [u32; 2],
        pub viewportBoundsRange: [f32; 2],
        pub viewportSubPixelBits: u32,
        pub minMemoryMapAlignment: usize,
        pub minTexelBufferOffsetAlignment: VkDeviceSize,
        pub minUniformBufferOffsetAlignment: VkDeviceSize,
        pub minStorageBufferOffsetAlignment: VkDeviceSize,
        pub minTexelOffset: i32,
        pub maxTexelOffset: u32,
        pub minTexelGatherOffset: i32,
        pub maxTexelGatherOffset: u32,
        pub minInterpolationOffset: f32,
        pub maxInterpolationOffset: f32,
        pub subPixelInterpolationOffsetBits: u32,
        pub maxFramebufferWidth: u32,
        pub maxFramebufferHeight: u32,
        pub maxFramebufferLayers: u32,
        pub framebufferColorSampleCounts: VkSampleCountFlags,
        pub framebufferDepthSampleCounts: VkSampleCountFlags,
        pub framebufferStencilSampleCounts: VkSampleCountFlags,
        pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
        pub maxColorAttachments: u32,
        pub sampledImageColorSampleCounts: VkSampleCountFlags,
        pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
        pub sampledImageDepthSampleCounts: VkSampleCountFlags,
        pub sampledImageStencilSampleCounts: VkSampleCountFlags,
        pub storageImageSampleCounts: VkSampleCountFlags,
        pub maxSampleMaskWords: u32,
        pub timestampComputeAndGraphics: VkBool32,
        pub timestampPeriod: f32,
        pub maxClipDistances: u32,
        pub maxCullDistances: u32,
        pub maxCombinedClipAndCullDistances: u32,
        pub discreteQueuePriorities: u32,
        pub pointSizeRange: [f32; 2],
        pub lineWidthRange: [f32; 2],
        pub pointSizeGranularity: f32,
        pub lineWidthGranularity: f32,
        pub strictLines: VkBool32,
        pub standardSampleLocations: VkBool32,
        pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
        pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
        pub nonCoherentAtomSize: VkDeviceSize,
    }

    /// Structure specifying physical device properties
    ///
    /// # Members
    /// - apiVersion is the version of Vulkan supported by the device, encoded as described in
    ///   https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers.
    ///
    /// - driverVersion is the vendor-specified version of the driver.
    ///
    /// - vendorID is a unique identifier for the vendor (see below) of the physical device.
    ///
    /// - deviceID is a unique identifier for the physical device among devices available from the vendor.
    ///
    /// - deviceType is a VkPhysicalDeviceType specifying the type of device.
    ///
    /// - deviceName is an array of VK_MAX_PHYSICAL_DEVICE_NAME_SIZE char containing a null-terminated
    ///   UTF-8 string which is the name of the device.
    ///
    /// - pipelineCacheUUID is an array of VK_UUID_SIZE uint8_t values representing a universally
    ///   unique identifier for the device.
    ///
    /// - limits is the VkPhysicalDeviceLimits structure specifying device-specific limits of the
    ///   physical device. See Limits for details.
    ///
    /// - sparseProperties is the VkPhysicalDeviceSparseProperties structure specifying various sparse
    ///   related properties of the physical device. See Sparse Properties for details.
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceProperties {
        pub apiVersion: u32,
        pub driverVersion: u32,
        pub vendorID: u32,
        pub deviceID: u32,
        pub deviceType: VkPhysicalDeviceType,
        pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
        pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
        pub limits: VkPhysicalDeviceLimits,
        pub sparseProperties: VkPhysicalDeviceSparseProperties,
    }

    /// Structure specifying physical device properties
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2
    ///
    /// # Members
    /// `sType` is the type of this structure.
    ///
    /// `pNext` is NULL or a pointer to an extension-specific structure.
    ///
    /// `properties` is a VkPhysicalDeviceProperties structure describing properties of the physical
    /// device. This structure is written with the same values as if it were written by
    /// vkGetPhysicalDeviceProperties.
    ///
    /// # Description
    /// The `pNext` chain of this structure is used to extend the structure with properties defined
    /// by extensions.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#VkPhysicalDeviceProperties2
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceProperties2 {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub properties: VkPhysicalDeviceProperties,
    }

    pub type VkPhysicalDeviceProperties2KHR = VkPhysicalDeviceProperties2;

    /// Structure specifying physical device properties for functionality promoted to Vulkan 1.1
    ///
    /// To query the properties of the driver corresponding to Vulkan 1.1 functionality, add
    /// `VkPhysicalDeviceVulkan11Properties` to the `pNext` chain of the `VkPhysicalDeviceProperties2`
    /// structure.
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES
    ///
    /// # Members
    /// `deviceUUID` is an array of VK_UUID_SIZE uint8_t values representing a universally unique
    /// identifier for the device.
    ///
    /// `driverUUID` is an array of VK_UUID_SIZE uint8_t values representing a universally unique
    /// identifier for the driver build in use by the device.
    ///
    /// `deviceLUID` is an array of VK_LUID_SIZE uint8_t values representing a locally unique
    /// identifier for the device.
    ///
    /// `deviceNodeMask` is a uint32_t bitfield identifying the node within a linked device adapter
    /// corresponding to the device.
    ///
    /// `deviceLUIDValid` is a boolean value that will be VK_TRUE if deviceLUID contains a valid LUID
    /// and deviceNodeMask contains a valid node mask, and VK_FALSE if they do not.
    ///
    /// `subgroupSize` is the default number of invocations in each subgroup. subgroupSize is at
    /// least 1 if any of the physical device’s queues support VK_QUEUE_GRAPHICS_BIT or
    /// VK_QUEUE_COMPUTE_BIT. subgroupSize is a power-of-two.
    ///
    /// `subgroupSupportedStages` is a bitfield of VkShaderStageFlagBits describing the shader stages
    /// that subgroup operations are supported in. subgroupSupportedStages will have the
    /// VK_SHADER_STAGE_COMPUTE_BIT bit set if any of the physical device’s queues support
    /// VK_QUEUE_COMPUTE_BIT.
    ///
    /// `subgroupSupportedOperations` is a bitmask of VkSubgroupFeatureFlagBits specifying the sets
    /// of subgroup operations supported on this device. subgroupSupportedOperations will have the
    /// VK_SUBGROUP_FEATURE_BASIC_BIT bit set if any of the physical device’s queues support
    /// VK_QUEUE_GRAPHICS_BIT or VK_QUEUE_COMPUTE_BIT.
    ///
    /// `subgroupQuadOperationsInAllStages` is a boolean specifying whether quad subgroup operations
    /// are available in all stages, or are restricted to fragment and compute stages.
    ///
    /// `pointClippingBehavior` is a VkPointClippingBehavior value specifying the point clipping
    /// behavior supported by the implementation.
    ///
    /// `maxMultiviewViewCount` is one greater than the maximum view index that can be used in a
    /// subpass.
    ///
    /// `maxMultiviewInstanceIndex` is the maximum valid value of instance index allowed to be
    /// generated by a drawing command recorded within a subpass of a multiview render pass
    /// instance.
    ///
    /// `protectedNoFault` specifies the behavior of the implementation when protected memory access
    /// rules are broken. If protectedNoFault is VK_TRUE, breaking those rules will not result in
    /// process termination or device loss.
    ///
    /// `maxPerSetDescriptors` is a maximum number of descriptors (summed over all descriptor types)
    /// in a single descriptor set that is guaranteed to satisfy any implementation-dependent
    /// constraints on the size of a descriptor set itself. Applications can query whether a
    /// descriptor set that goes beyond this limit is supported using
    /// vkGetDescriptorSetLayoutSupport.
    ///
    /// `maxMemoryAllocationSize` is the maximum size of a memory allocation that can be created,
    /// even if there is more space available in the heap.
    ///
    /// # Description
    /// The members of VkPhysicalDeviceVulkan11Properties must have the same values as the
    /// corresponding members of VkPhysicalDeviceIDProperties, VkPhysicalDeviceSubgroupProperties,
    /// VkPhysicalDevicePointClippingProperties, VkPhysicalDeviceMultiviewProperties,
    /// VkPhysicalDeviceProtectedMemoryProperties, and VkPhysicalDeviceMaintenance3Properties.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#VkPhysicalDeviceVulkan11Properties
    #[repr(C)]
    pub struct VkPhysicalDeviceVulkan11Properties {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub deviceUUID: [u8; VK_UUID_SIZE],
        pub driverUUID: [u8; VK_UUID_SIZE],
        pub deviceLUID: [u8; VK_LUID_SIZE],
        pub deviceNodeMask: u32,
        pub deviceLUIDValid: VkBool32,
        pub subgroupSize: u32,
        pub subgroupSupportedStages: VkShaderStageFlags,
        pub subgroupSupportedOperations: VkSubgroupFeatureFlags,
        pub subgroupQuadOperationsInAllStages: VkBool32,
        pub pointClippingBehavior: VkPointClippingBehavior,
        pub maxMultiviewViewCount: u32,
        pub maxMultiviewInstanceIndex: u32,
        pub protectedNoFault: VkBool32,
        pub maxPerSetDescriptors: u32,
        pub maxMemoryAllocationSize: VkDeviceSize,
    }

    /// Structure containing the conformance test suite version the implementation is compliant with
    ///
    /// # todo
    /// Finish documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkConformanceVersion.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkConformanceVersion {
        major: u8,
        minor: u8,
        subminor: u8,
        patch: u8
    }

    pub type VkConformanceVersionKHR = VkConformanceVersion;

    #[repr(C)]
    #[derive(Debug)]
    pub struct VkShaderFloatControlsIndependence {}

    /// Structure specifying physical device properties for functionality promoted to Vulkan 1.2
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES
    ///
    /// To query the properties of the driver corresponding to Vulkan 1.2 functionality, add
    /// VkPhysicalDeviceVulkan12Properties to the pNext chain of the VkPhysicalDeviceProperties2
    /// structure.
    ///
    /// # Members
    /// `driverID` is a unique identifier for the driver of the physical device.
    ///
    /// `driverName` is an array of VK_MAX_DRIVER_NAME_SIZE_KHR char containing a null-terminated
    /// UTF-8 string which is the name of the driver.
    ///
    /// `driverInfo` is an array of VK_MAX_DRIVER_INFO_SIZE_KHR char containing a null-terminated
    /// UTF-8 string with additional information about the driver.
    ///
    /// `conformanceVersion` is the version of the Vulkan conformance test this driver is conformant
    /// against (see VkConformanceVersion).
    ///
    /// `denormBehaviorIndependence` is a VkShaderFloatControlsIndependence value indicating whether,
    /// and how, denorm behavior can be set independently for different bit widths.
    ///
    /// `roundingModeIndependence` is a VkShaderFloatControlsIndependence value indicating whether,
    /// and how, rounding modes can be set independently for different bit widths.
    ///
    /// `shaderSignedZeroInfNanPreserveFloat16` is a boolean value indicating whether sign of a zero,
    /// Nans and ±∞ can be preserved in 16-bit floating-point computations. It also indicates
    /// whether the SignedZeroInfNanPreserve execution mode can be used for 16-bit floating-point
    /// types.
    ///
    /// `shaderSignedZeroInfNanPreserveFloat32` is a boolean value indicating whether sign of a zero,
    /// Nans and ±∞ can be preserved in 32-bit floating-point computations. It also indicates whether
    /// the SignedZeroInfNanPreserve execution mode can be used for 32-bit floating-point types.
    ///
    /// `shaderSignedZeroInfNanPreserveFloat64` is a boolean value indicating whether sign of a zero,
    /// Nans and ±∞ can be preserved in 64-bit floating-point computations. It also indicates whether
    /// yhe SignedZeroInfNanPreserve execution mode can be used for 64-bit floating-point types.
    ///
    /// `shaderDenormPreserveFloat16` is a boolean value indicating whether denormals can be preserved
    /// in 16-bit floating-point computations. It also indicates whether the DenormPreserve execution
    /// mode can be used for 16-bit floating-point types.
    ///
    /// `shaderDenormPreserveFloat32` is a boolean value indicating whether denormals can be preserved
    /// in 32-bit floating-point computations. It also indicates whether the DenormPreserve execution
    /// mode can be used for 32-bit floating-point types.
    ///
    /// `shaderDenormPreserveFloat64` is a boolean value indicating whether denormals can be preserved
    /// in 64-bit floating-point computations. It also indicates whether the DenormPreserve execution
    /// mode can be used for 64-bit floating-point types.
    ///
    /// `shaderDenormFlushToZeroFloat16` is a boolean value indicating whether denormals can be
    /// flushed to zero in 16-bit floating-point computations. It also indicates whether the
    /// DenormFlushToZero execution mode can be used for 16-bit floating-point types.
    ///
    /// `shaderDenormFlushToZeroFloat32` is a boolean value indicating whether denormals can be
    /// flushed to zero in 32-bit floating-point computations. It also indicates whether the
    /// DenormFlushToZero execution mode can be used for 32-bit floating-point types.
    ///
    /// `shaderDenormFlushToZeroFloat64` is a boolean value indicating whether denormals can be
    /// flushed to zero in 64-bit floating-point computations. It also indicates whether the
    /// DenormFlushToZero execution mode can be used for 64-bit floating-point types.
    ///
    /// `shaderRoundingModeRTEFloat16` is a boolean value indicating whether an implementation
    /// supports the round-to-nearest-even rounding mode for 16-bit floating-point arithmetic
    /// nd conversion instructions. It also indicates whether the RoundingModeRTE execution mode
    /// can be used for 16-bit floating-point types.
    ///
    /// `shaderRoundingModeRTEFloat32` is a boolean value indicating whether an implementation
    /// supports the round-to-nearest-even rounding mode for 32-bit floating-point arithmetic
    /// and conversion instructions. It also indicates whether the RoundingModeRTE execution mode
    /// can be used for 32-bit floating-point types.
    ///
    /// `shaderRoundingModeRTEFloat64` is a boolean value indicating whether an implementation
    /// supports the round-to-nearest-even rounding mode for 64-bit floating-point arithmetic and
    /// conversion instructions. It also indicates whether the RoundingModeRTE execution mode can
    /// be used for 64-bit floating-point types.
    ///
    /// `shaderRoundingModeRTZFloat16` is a boolean value indicating whether an implementation
    /// supports the round-towards-zero rounding mode for 16-bit floating-point arithmetic and
    /// conversion instructions. It also indicates whether the RoundingModeRTZ execution mode can
    /// be used for 16-bit floating-point types.
    ///
    /// `shaderRoundingModeRTZFloat32` is a boolean value indicating whether an implementation
    /// supports the round-towards-zero rounding mode for 32-bit floating-point arithmetic and
    /// conversion instructions. It also indicates whether the RoundingModeRTZ execution mode can
    /// be used for 32-bit floating-point types.
    ///
    /// `shaderRoundingModeRTZFloat64` is a boolean value indicating whether an implementation
    /// supports the round-towards-zero rounding mode for 64-bit floating-point arithmetic and
    /// conversion instructions. It also indicates whether the RoundingModeRTZ execution mode can
    /// be used for 64-bit floating-point types.
    ///
    /// `maxUpdateAfterBindDescriptorsInAllPools` is the maximum number of descriptors (summed over
    /// all descriptor types) that can be created across all pools that are created with the
    /// VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT bit set. Pool creation may fail when this
    /// limit is exceeded, or when the space this limit represents is unable to satisfy a pool
    /// creation due to fragmentation.
    ///
    /// `shaderUniformBufferArrayNonUniformIndexingNative` is a boolean value indicating whether
    /// uniform buffer descriptors natively support nonuniform indexing. If this is VK_FALSE, then
    /// a single dynamic instance of an instruction that nonuniformly indexes an array of uniform
    /// buffers may execute multiple times in order to access all the descriptors.
    ///
    /// `shaderSampledImageArrayNonUniformIndexingNative` is a boolean value indicating whether
    /// sampler and image descriptors natively support nonuniform indexing. If this is VK_FALSE,
    /// then a single dynamic instance of an instruction that nonuniformly indexes an array of
    /// samplers or images may execute multiple times in order to access all the descriptors.
    ///
    /// `shaderStorageBufferArrayNonUniformIndexingNative` is a boolean value indicating whether
    /// storage buffer descriptors natively support nonuniform indexing. If this is VK_FALSE, then a
    /// single dynamic instance of an instruction that nonuniformly indexes an array of storage
    /// buffers may execute multiple times in order to access all the descriptors.
    ///
    /// `shaderStorageImageArrayNonUniformIndexingNative` is a boolean value indicating whether
    /// storage image descriptors natively support nonuniform indexing. If this is VK_FALSE, then a
    /// single dynamic instance of an instruction that nonuniformly indexes an array of storage
    /// images may execute multiple times in order to access all the descriptors.
    ///
    /// `shaderInputAttachmentArrayNonUniformIndexingNative` is a boolean value indicating whether
    /// input attachment descriptors natively support nonuniform indexing. If this is VK_FALSE,
    /// then a single dynamic instance of an instruction that nonuniformly indexes an array of input
    /// attachments may execute multiple times in order to access all the descriptors.
    ///
    /// `robustBufferAccessUpdateAfterBind` is a boolean value indicating whether robustBufferAccess
    /// can be enabled in a device simultaneously with descriptorBindingUniformBufferUpdateAfterBind,
    /// descriptorBindingStorageBufferUpdateAfterBind, descriptorBindingUniformTexelBufferUpdateAfterBind,
    /// and/or descriptorBindingStorageTexelBufferUpdateAfterBind. If this is VK_FALSE, then either
    /// robustBufferAccess must be disabled or all of these update-after-bind features must be
    /// disabled.
    ///
    /// `quadDivergentImplicitLod` is a boolean value indicating whether implicit level of detail
    /// calculations for image operations have well-defined results when the image and/or sampler
    /// objects used for the instruction are not uniform within a quad. See Derivative Image
    /// Operations.
    ///
    /// `maxPerStageDescriptorUpdateAfterBindSamplers` is similar to maxPerStageDescriptorSamplers
    /// but counts descriptors from descriptor sets created with or without the
    /// VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxPerStageDescriptorUpdateAfterBindUniformBuffers` is similar to
    /// maxPerStageDescriptorUniformBuffers but counts descriptors from descriptor sets created
    /// with or without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxPerStageDescriptorUpdateAfterBindStorageBuffers` is similar to
    /// maxPerStageDescriptorStorageBuffers but counts descriptors from descriptor sets created
    /// with or without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxPerStageDescriptorUpdateAfterBindSampledImages` is similar to
    /// maxPerStageDescriptorSampledImages but counts descriptors from descriptor sets created
    /// with or without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxPerStageDescriptorUpdateAfterBindStorageImages` is similar to
    /// maxPerStageDescriptorStorageImages but counts descriptors from descriptor sets created
    /// with or without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxPerStageDescriptorUpdateAfterBindInputAttachments` is similar to
    /// maxPerStageDescriptorInputAttachments but counts descriptors from descriptor sets created
    /// with or without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxPerStageUpdateAfterBindResources` is similar to maxPerStageResources but counts descriptors
    /// from descriptor sets created with or without the
    /// VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxDescriptorSetUpdateAfterBindSamplers` is similar to maxDescriptorSetSamplers but counts
    /// descriptors from descriptor sets created with or without the
    /// VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxDescriptorSetUpdateAfterBindUniformBuffers` is similar to maxDescriptorSetUniformBuffers
    /// but counts descriptors from descriptor sets created with or without the
    /// VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxDescriptorSetUpdateAfterBindUniformBuffersDynamic` is similar to
    /// maxDescriptorSetUniformBuffersDynamic but counts descriptors from descriptor sets created
    /// with or without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxDescriptorSetUpdateAfterBindStorageBuffers` is similar to maxDescriptorSetStorageBuffers
    /// but counts descriptors from descriptor sets created with or without the
    /// VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxDescriptorSetUpdateAfterBindStorageBuffersDynamic` is similar to
    /// maxDescriptorSetStorageBuffersDynamic but counts descriptors from descriptor sets created
    /// with or without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxDescriptorSetUpdateAfterBindSampledImages` is similar to maxDescriptorSetSampledImages
    /// but counts descriptors from descriptor sets created with or without the
    /// VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxDescriptorSetUpdateAfterBindStorageImages` is similar to maxDescriptorSetStorageImages
    /// but counts descriptors from descriptor sets created with or without the
    /// VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `maxDescriptorSetUpdateAfterBindInputAttachments` is similar to
    /// maxDescriptorSetInputAttachments but counts descriptors from descriptor sets created with or
    /// without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set.
    ///
    /// `supportedDepthResolveModes` is a bitmask of VkResolveModeFlagBits indicating the set of
    /// supported depth resolve modes. VK_RESOLVE_MODE_SAMPLE_ZERO_BIT must be included in the set
    /// but implementations may support additional modes.
    ///
    /// `supportedStencilResolveModes` is a bitmask of VkResolveModeFlagBits indicating the set of
    /// supported stencil resolve modes. VK_RESOLVE_MODE_SAMPLE_ZERO_BIT must be included in the
    /// set but implementations may support additional modes. VK_RESOLVE_MODE_AVERAGE_BIT must not
    /// be included in the set.
    ///
    /// `independentResolveNone` is VK_TRUE if the implementation supports setting the depth and
    /// stencil resolve modes to different values when one of those modes is VK_RESOLVE_MODE_NONE.
    /// Otherwise the implementation only supports setting both modes to the same value.
    ///
    /// `independentResolve` is VK_TRUE if the implementation supports all combinations of the
    /// supported depth and stencil resolve modes, including setting either depth or stencil resolve
    /// mode to VK_RESOLVE_MODE_NONE. An implementation that supports independentResolve must also
    /// support independentResolveNone.
    ///
    /// `filterMinmaxSingleComponentFormats` is a boolean value indicating whether a minimum set of
    /// required formats support min/max filtering.
    ///
    /// `filterMinmaxImageComponentMapping` is a boolean value indicating whether the implementation
    /// supports non-identity component mapping of the image when doing min/max filtering.
    ///
    /// `maxTimelineSemaphoreValueDifference` indicates the maximum difference allowed by the
    /// implementation between the current value of a timeline semaphore and any pending signal or
    /// wait operations.
    ///
    /// `framebufferIntegerColorSampleCounts` is a bitmask of VkSampleCountFlagBits indicating the
    /// color sample counts that are supported for all framebuffer color attachments with integer
    /// formats.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#_vkphysicaldevicevulkan12properties3
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceVulkan12Properties {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub driverID: VkDriverId,
        pub driverName: [c_char; VK_MAX_DRIVER_NAME_SIZE],
        pub driverInfo: [c_char; VK_MAX_DRIVER_INFO_SIZE],
        pub conformanceVersion: VkConformanceVersion,
        pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
        pub roundingModeIndependence: VkShaderFloatControlsIndependence,
        pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
        pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
        pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
        pub shaderDenormPreserveFloat16: VkBool32,
        pub shaderDenormPreserveFloat32: VkBool32,
        pub shaderDenormPreserveFloat64: VkBool32,
        pub shaderDenormFlushToZeroFloat16: VkBool32,
        pub shaderDenormFlushToZeroFloat32: VkBool32,
        pub shaderDenormFlushToZeroFloat64: VkBool32,
        pub shaderRoundingModeRTEFloat16: VkBool32,
        pub shaderRoundingModeRTEFloat32: VkBool32,
        pub shaderRoundingModeRTEFloat64: VkBool32,
        pub shaderRoundingModeRTZFloat16: VkBool32,
        pub shaderRoundingModeRTZFloat32: VkBool32,
        pub shaderRoundingModeRTZFloat64: VkBool32,
        pub maxUpdateAfterBindDescriptorsInAllPools: u32,
        pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
        pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
        pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
        pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
        pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
        pub robustBufferAccessUpdateAfterBind: VkBool32,
        pub quadDivergentImplicitLod: VkBool32,
        pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
        pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
        pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
        pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
        pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
        pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
        pub maxPerStageUpdateAfterBindResources: u32,
        pub maxDescriptorSetUpdateAfterBindSamplers: u32,
        pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
        pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
        pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
        pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
        pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
        pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
        pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
        pub supportedDepthResolveModes: VkResolveModeFlags,
        pub supportedStencilResolveModes: VkResolveModeFlags,
        pub independentResolveNone: VkBool32,
        pub independentResolve: VkBool32,
        pub filterMinmaxSingleComponentFormats: VkBool32,
        pub filterMinmaxImageComponentMapping: VkBool32,
        pub maxTimelineSemaphoreValueDifference: u64,
        pub framebufferIntegerColorSampleCounts: VkSampleCountFlags,
    }

    /// VkPhysicalDeviceVulkan13Properties
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES
    /// # Members
    /// minSubgroupSize is the minimum subgroup size supported by this device. minSubgroupSize is at
    /// least one if any of the physical device’s queues support VK_QUEUE_GRAPHICS_BIT or
    /// VK_QUEUE_COMPUTE_BIT. minSubgroupSize is a power-of-two. minSubgroupSize is less than or
    /// equal to maxSubgroupSize. minSubgroupSize is less than or equal to subgroupSize.
    ///
    /// maxSubgroupSize is the maximum subgroup size supported by this device. maxSubgroupSize is at
    /// least one if any of the physical device’s queues support VK_QUEUE_GRAPHICS_BIT or
    /// VK_QUEUE_COMPUTE_BIT. maxSubgroupSize is a power-of-two. maxSubgroupSize is greater than or
    /// equal to minSubgroupSize. maxSubgroupSize is greater than or equal to subgroupSize.
    ///
    /// maxComputeWorkgroupSubgroups is the maximum number of subgroups supported by the
    /// implementation within a workgroup.
    ///
    /// requiredSubgroupSizeStages is a bitfield of what shader stages support having a required
    /// subgroup size specified.
    ///
    /// maxInlineUniformBlockSize is the maximum size in bytes of an inline uniform block binding.
    ///
    /// maxPerStageDescriptorInlineUniformBlocks is the maximum number of inline uniform block
    /// bindings that can be accessible to a single shader stage in a pipeline layout. Descriptor
    /// bindings with a descriptor type of VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK count against
    /// this limit. Only descriptor bindings in descriptor set layouts created without the
    /// VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit set count against this limit.
    ///
    /// maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks is similar to
    /// maxPerStageDescriptorInlineUniformBlocks but counts descriptor bindings from descriptor sets
    /// created with or without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT bit
    /// set.
    ///
    /// maxDescriptorSetInlineUniformBlocks is the maximum number of inline uniform block bindings
    /// that can be included in descriptor bindings in a pipeline layout across all pipeline shader
    /// stages and descriptor set numbers. Descriptor bindings with a descriptor type of
    /// VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK count against this limit. Only descriptor bindings
    /// in descriptor set layouts created without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT
    /// bit set count against this limit.
    ///
    /// maxDescriptorSetUpdateAfterBindInlineUniformBlocks is similar to
    /// maxDescriptorSetInlineUniformBlocks but counts descriptor bindings from descriptor sets
    /// created with or without the VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT
    /// bit set.
    ///
    /// maxInlineUniformTotalSize is the maximum total size in bytes of all inline uniform block
    /// bindings, across all pipeline shader stages and descriptor set numbers, that can be
    /// included in a pipeline layout. Descriptor bindings with a descriptor type of
    /// VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK count against this limit.
    ///
    /// integerDotProduct8BitUnsignedAccelerated is a boolean that will be VK_TRUE if the support
    /// for 8-bit unsigned dot product operations using the OpUDotKHR SPIR-V instruction is
    /// accelerated as defined below.
    ///
    /// integerDotProduct8BitSignedAccelerated is a boolean that will be VK_TRUE if the support for
    /// 8-bit signed dot product operations using the OpSDotKHR SPIR-V instruction is accelerated
    /// as defined below.
    ///
    /// integerDotProduct8BitMixedSignednessAccelerated is a boolean that will be VK_TRUE if the
    /// support for 8-bit mixed signedness dot product operations using the OpSUDotKHR SPIR-V
    /// instruction is accelerated as defined below.
    ///
    /// integerDotProduct4x8BitPackedUnsignedAccelerated is a boolean that will be VK_TRUE if the
    /// support for 8-bit unsigned dot product operations from operands packed into 32-bit integers
    /// using the OpUDotKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProduct4x8BitPackedSignedAccelerated is a boolean that will be VK_TRUE if the
    /// support for 8-bit signed dot product operations from operands packed into 32-bit integers
    /// using the OpSDotKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProduct4x8BitPackedMixedSignednessAccelerated is a boolean that will be VK_TRUE
    /// if the support for 8-bit mixed signedness dot product operations from operands packed into
    /// 32-bit integers using the OpSUDotKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProduct16BitUnsignedAccelerated is a boolean that will be VK_TRUE if the support for
    /// 16-bit unsigned dot product operations using the OpUDotKHR SPIR-V instruction is accelerated
    /// as defined below.
    ///
    /// integerDotProduct16BitSignedAccelerated is a boolean that will be VK_TRUE if the support for
    /// 16-bit signed dot product operations using the OpSDotKHR SPIR-V instruction is accelerated as
    /// defined below.
    ///
    /// integerDotProduct16BitMixedSignednessAccelerated is a boolean that will be VK_TRUE if the
    /// support for 16-bit mixed signedness dot product operations using the OpSUDotKHR SPIR-V
    /// instruction is accelerated as defined below.
    ///
    /// integerDotProduct32BitUnsignedAccelerated is a boolean that will be VK_TRUE if the support
    /// for 32-bit unsigned dot product operations using the OpUDotKHR SPIR-V instruction is
    /// accelerated as defined below.
    ///
    /// integerDotProduct32BitSignedAccelerated is a boolean that will be VK_TRUE if the support
    /// for 32-bit signed dot product operations using the OpSDotKHR SPIR-V instruction is
    /// accelerated as defined below.
    ///
    /// integerDotProduct32BitMixedSignednessAccelerated is a boolean that will be VK_TRUE if the
    /// support for 32-bit mixed signedness dot product operations using the OpSUDotKHR SPIR-V
    /// instruction is accelerated as defined below.
    ///
    /// integerDotProduct64BitUnsignedAccelerated is a boolean that will be VK_TRUE if the support
    /// for 64-bit unsigned dot product operations using the OpUDotKHR SPIR-V instruction is
    /// accelerated as defined below.
    ///
    /// integerDotProduct64BitSignedAccelerated is a boolean that will be VK_TRUE if the support
    /// for 64-bit signed dot product operations using the OpSDotKHR SPIR-V instruction is
    /// accelerated as defined below.
    ///
    /// integerDotProduct64BitMixedSignednessAccelerated is a boolean that will be VK_TRUE if the
    /// support for 64-bit mixed signedness dot product operations using the OpSUDotKHR SPIR-V
    /// instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating8BitUnsignedAccelerated is a boolean that will be
    /// VK_TRUE if the support for 8-bit unsigned accumulating saturating dot product operations
    /// using the OpUDotAccSatKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating8BitSignedAccelerated is a boolean that will be
    /// VK_TRUE if the support for 8-bit signed accumulating saturating dot product operations
    /// using the OpSDotAccSatKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated is a boolean that
    /// will be VK_TRUE if the support for 8-bit mixed signedness accumulating saturating dot
    /// product operations using the OpSUDotAccSatKHR SPIR-V instruction is accelerated as
    /// defined below.
    ///
    /// integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated is a boolean that
    /// will be VK_TRUE if the support for 8-bit unsigned accumulating saturating dot product
    /// operations from operands packed into 32-bit integers using the OpUDotAccSatKHR SPIR-V
    /// instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated is a boolean that will
    /// be VK_TRUE if the support for 8-bit signed accumulating saturating dot product operations
    /// from operands packed into 32-bit integers using the OpSDotAccSatKHR SPIR-V instruction is
    /// accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated is a boolean that
    /// will be VK_TRUE if the support for 8-bit mixed signedness accumulating saturating dot product
    /// operations from operands packed into 32-bit integers using the OpSUDotAccSatKHR SPIR-V
    /// instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating16BitUnsignedAccelerated is a boolean that will be
    /// VK_TRUE if the support for 16-bit unsigned accumulating saturating dot product operations
    /// using the OpUDotAccSatKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating16BitSignedAccelerated is a boolean that will be
    /// VK_TRUE if the support for 16-bit signed accumulating saturating dot product operations
    /// using the OpSDotAccSatKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated is a boolean that
    /// will be VK_TRUE if the support for 16-bit mixed signedness accumulating saturating dot
    /// product operations using the OpSUDotAccSatKHR SPIR-V instruction is accelerated as
    /// defined below.
    ///
    /// integerDotProductAccumulatingSaturating32BitUnsignedAccelerated is a boolean that will be
    /// VK_TRUE if the support for 32-bit unsigned accumulating saturating dot product operations
    /// using the OpUDotAccSatKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating32BitSignedAccelerated is a boolean that will be
    /// VK_TRUE if the support for 32-bit signed accumulating saturating dot product operations
    /// using the OpSDotAccSatKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated is a boolean that
    /// will be VK_TRUE if the support for 32-bit mixed signedness accumulating saturating dot
    /// product operations using the OpSUDotAccSatKHR SPIR-V instruction is accelerated as
    /// defined below.
    ///
    /// integerDotProductAccumulatingSaturating64BitUnsignedAccelerated is a boolean that will be
    /// VK_TRUE if the support for 64-bit unsigned accumulating saturating dot product operations
    /// using the OpUDotAccSatKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating64BitSignedAccelerated is a boolean that will be
    /// VK_TRUE if the support for 64-bit signed accumulating saturating dot product operations
    /// using the OpSDotAccSatKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated is a boolean that will
    /// be VK_TRUE if the support for 64-bit mixed signedness accumulating saturating dot product
    /// operations using the OpSUDotAccSatKHR SPIR-V instruction is accelerated as defined below.
    ///
    /// storageTexelBufferOffsetAlignmentBytes is a byte alignment that is sufficient for a storage
    /// texel buffer of any format. The value must be a power of two.
    ///
    /// storageTexelBufferOffsetSingleTexelAlignment indicates whether single texel alignment is
    /// sufficient for a storage texel buffer of any format.
    ///
    /// uniformTexelBufferOffsetAlignmentBytes is a byte alignment that is sufficient for a uniform
    /// texel buffer of any format. The value must be a power of two.
    ///
    /// uniformTexelBufferOffsetSingleTexelAlignment indicates whether single texel alignment is
    /// sufficient for a uniform texel buffer of any format.
    ///
    /// maxBufferSize is the maximum size VkBuffer that can be created.
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceVulkan13Properties {
        pub sType: VkStructureType,
        pub pNext: *mut std::ffi::c_void,
        pub minSubgroupSize: u32,
        pub maxSubgroupSize: u32,
        pub maxComputeWorkgroupSubgroups: u32,
        pub requiredSubgroupSizeStages: VkShaderStageFlags,
        pub maxInlineUniformBlockSize: u32,
        pub maxPerStageDescriptorInlineUniformBlocks: u32,
        pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
        pub maxDescriptorSetInlineUniformBlocks: u32,
        pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
        pub maxInlineUniformTotalSize: u32,
        pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
        pub integerDotProduct8BitSignedAccelerated: VkBool32,
        pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
        pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
        pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
        pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
        pub integerDotProduct16BitSignedAccelerated: VkBool32,
        pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
        pub integerDotProduct32BitSignedAccelerated: VkBool32,
        pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
        pub integerDotProduct64BitSignedAccelerated: VkBool32,
        pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
        pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
        pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
        pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
        pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
        pub maxBufferSize: VkDeviceSize,
    }

    /// Structure specifying physical device properties for functionality promoted to Vulkan 1.4
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES
    ///
    /// # Members
    /// sType is a VkStructureType value identifying this structure.
    ///
    /// pNext is NULL or a pointer to a structure extending this structure.
    ///
    /// lineSubPixelPrecisionBits is the number of bits of subpixel precision in framebuffer
    /// coordinates xf and yf when rasterizing line segments.
    ///
    /// maxVertexAttribDivisor is the maximum value of the number of instances that will repeat the
    /// value of vertex attribute data when instanced rendering is enabled.
    ///
    /// supportsNonZeroFirstInstance specifies whether a non-zero value for the firstInstance
    /// parameter of drawing commands is supported when VkVertexInputBindingDivisorDescription::divisor
    /// is not 1.
    ///
    /// maxPushDescriptors is the maximum number of descriptors that can be used in a descriptor set
    /// layout created with VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT set.
    ///
    /// dynamicRenderingLocalReadDepthStencilAttachments is VK_TRUE if the implementation supports
    /// local reads of depth/stencil attachments, VK_FALSE otherwise.
    ///
    /// dynamicRenderingLocalReadMultisampledAttachments is VK_TRUE if the implementation supports
    /// local reads of multisampled attachments, VK_FALSE otherwise.
    ///
    /// earlyFragmentMultisampleCoverageAfterSampleCounting is a boolean value indicating whether
    /// the fragment shading and multisample coverage operations are performed after sample
    /// counting for fragment shaders with EarlyFragmentTests execution mode.
    ///
    /// earlyFragmentSampleMaskTestBeforeSampleCounting is a boolean value indicating whether the
    /// sample mask test operation is performed before sample counting for fragment shaders using the
    /// EarlyFragmentTests execution mode.
    ///
    /// depthStencilSwizzleOneSupport is a boolean indicating that depth/stencil texturing operations
    /// with VK_COMPONENT_SWIZZLE_ONE have defined behavior.
    ///
    /// polygonModePointSize is a boolean value indicating whether the point size of the final
    /// rasterization of polygons with VK_POLYGON_MODE_POINT is controlled by PointSize.
    ///
    /// nonStrictSinglePixelWideLinesUseParallelogram is a boolean value indicating whether
    /// non-strict lines with a width of 1.0 are rasterized as parallelograms or using Bresenham’s
    /// algorithm.
    ///
    /// nonStrictWideLinesUseParallelogram is a boolean value indicating whether non-strict lines
    /// with a width greater than 1.0 are rasterized as parallelograms or using Bresenham’s algorithm.
    ///
    /// blockTexelViewCompatibleMultipleLayers is a boolean value indicating that an implementation
    /// supports creating image views with VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT where
    /// the layerCount member of subresourceRange is greater than 1.
    ///
    /// maxCombinedImageSamplerDescriptorCount is the maximum number of combined image sampler
    /// descriptors that the implementation uses to access any of the formats that require a
    /// sampler Y′CBCR conversion supported by the implementation.
    ///
    /// fragmentShadingRateClampCombinerInputs is a boolean value indicating that an implementation
    /// clamps the inputs to combiner operations.
    ///
    /// defaultRobustnessStorageBuffers describes the behavior of out of bounds accesses made to
    /// storage buffers when no robustness features are enabled
    ///
    /// defaultRobustnessUniformBuffers describes the behavior of out of bounds accesses made to
    /// uniform buffers when no robustness features are enabled
    ///
    /// defaultRobustnessVertexInputs describes the behavior of out of bounds accesses made to
    /// vertex input attributes when no robustness features are enabled
    ///
    /// defaultRobustnessImages describes the behavior of out of bounds accesses made to images
    /// when no robustness features are enabled
    ///
    /// copySrcLayoutCount is an integer related to the number of image layouts for host copies
    /// from images available or queried, as described below.
    ///
    /// pCopySrcLayouts is a pointer to an array of VkImageLayout in which supported image layouts
    /// for use with host copy operations from images are returned.
    ///
    /// copyDstLayoutCount is an integer related to the number of image layouts for host copies to
    /// images available or queried, as described below.
    ///
    /// pCopyDstLayouts is a pointer to an array of VkImageLayout in which supported image layouts
    /// for use with host copy operations to images are returned.
    ///
    /// optimalTilingLayoutUUID is an array of VK_UUID_SIZE uint8_t values representing a universally
    /// unique identifier for the implementation’s swizzling layout of images created with
    /// VK_IMAGE_TILING_OPTIMAL.
    ///
    /// identicalMemoryTypeRequirements indicates that specifying the VK_IMAGE_USAGE_HOST_TRANSFER_BIT
    /// flag in VkImageCreateInfo::usage does not affect the memory type requirements of the image.
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceVulkan14Properties {
        pub sType: VkStructureType,
        pub pNext: *mut std::ffi::c_void,
        pub lineSubPixelPrecisionBits: u32,
        pub maxVertexAttribDivisor: u32,
        pub supportsNonZeroFirstInstance: VkBool32,
        pub maxPushDescriptors: u32,
        pub dynamicRenderingLocalReadDepthStencilAttachments: VkBool32,
        pub dynamicRenderingLocalReadMultisampledAttachments: VkBool32,
        pub earlyFragmentMultisampleCoverageAfterSampleCounting: VkBool32,
        pub earlyFragmentSampleMaskTestBeforeSampleCounting: VkBool32,
        pub depthStencilSwizzleOneSupport: VkBool32,
        pub polygonModePointSize: VkBool32,
        pub identicalMemoryTypeRequirements: VkBool32,
    }

    /// Structure specifying IDs related to the physical device
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES
    ///
    /// # Members
    /// sType is a VkStructureType value identifying this structure.
    ///
    /// pNext is NULL or a pointer to a structure extending this structure.
    ///
    /// deviceUUID is an array of VK_UUID_SIZE uint8_t values representing a universally unique
    /// identifier for the device.
    ///
    /// driverUUID is an array of VK_UUID_SIZE uint8_t values representing a universally unique
    /// identifier for the driver build in use by the device.
    ///
    /// deviceLUID is an array of VK_LUID_SIZE uint8_t values representing a locally unique
    /// identifier for the device.
    ///
    /// deviceNodeMask is a uint32_t bitfield identifying the node within a linked device adapter
    /// corresponding to the device.
    ///
    /// deviceLUIDValid is a boolean value that will be VK_TRUE if deviceLUID contains a valid LUID
    /// and deviceNodeMask contains a valid node mask, and VK_FALSE if they do not.
    ///
    /// # Description
    /// If the VkPhysicalDeviceIDProperties structure is included in the pNext chain of the
    /// VkPhysicalDeviceProperties2 structure passed to vkGetPhysicalDeviceProperties2, it is
    /// filled in with each corresponding implementation-dependent property.
    ///
    /// deviceUUID must be immutable for a given device across instances, processes, driver APIs,
    /// driver versions, and system reboots.
    ///
    /// Applications can compare the driverUUID value across instance and process boundaries, and
    /// can make similar queries in external APIs to determine whether they are capable of sharing
    /// memory objects and resources using them with the device.
    ///
    /// deviceUUID and/or driverUUID must be used to determine whether a particular external object
    /// can be shared between driver components, where such a restriction exists as defined in the
    /// compatibility table for the particular object type:
    ///
    /// - External memory handle types compatibility
    ///
    /// - External semaphore handle types compatibility
    ///
    /// - External fence handle types compatibility
    ///
    /// If deviceLUIDValid is VK_FALSE, the values of deviceLUID and deviceNodeMask are undefined.
    /// If deviceLUIDValid is VK_TRUE and Vulkan is running on the Windows operating system, the
    /// contents of deviceLUID can be cast to an LUID object and must be equal to the locally unique
    /// identifier of a IDXGIAdapter1 object that corresponds to physicalDevice. If deviceLUIDValid
    /// is VK_TRUE, deviceNodeMask must contain exactly one bit. If Vulkan is running on an operating
    /// system that supports the Direct3D 12 API and physicalDevice corresponds to an individual
    /// device in a linked device adapter, deviceNodeMask identifies the Direct3D 12 node
    /// corresponding to physicalDevice. Otherwise, deviceNodeMask must be 1.
    ///
    /// > *Note*
    /// > Although they have identical descriptions, VkPhysicalDeviceIDProperties::deviceUUID may
    /// > differ from VkPhysicalDeviceProperties2::pipelineCacheUUID. The former is intended to
    /// > identify and correlate devices across API and driver boundaries, while the latter is used
    /// > to identify a compatible device and driver combination to use when serializing and
    /// > de-serializing pipeline state.
    /// >
    /// > Implementations should return deviceUUID values which are likely to be unique even in the
    /// > presence of multiple Vulkan implementations (such as a GPU driver and a software renderer;
    /// > two drivers for different GPUs; or the same Vulkan driver running on two logically
    /// > different devices).
    /// >
    /// > Khronos' conformance testing is unable to guarantee that deviceUUID values are actually
    /// > unique, so implementors should make their own best efforts to ensure this. In particular,
    /// > hard-coded deviceUUID values, especially all-0 bits, should never be used.
    /// >
    /// > A combination of values unique to the vendor, the driver, and the hardware environment can
    /// > be used to provide a deviceUUID which is unique to a high degree of certainty. Some possible
    /// > inputs to such a computation are:
    /// >
    /// > - Information reported by vkGetPhysicalDeviceProperties
    /// >
    /// > - PCI device ID (if defined)
    /// >
    /// > - PCI bus ID, or similar system configuration information.
    /// >
    /// > - Driver binary checksums.
    ///
    /// > Note
    /// > While VkPhysicalDeviceIDProperties::deviceUUID is specified to remain consistent across
    /// > driver versions and system reboots, it is not intended to be usable as a serializable
    /// > persistent identifier for a device. It may change when a device is physically added to,
    /// > removed from, or moved to a different connector in a system while that system is powered
    /// > down. Further, there is no reasonable way to verify with conformance testing that a given
    /// > device retains the same UUID in a given system across all driver versions supported in that
    /// > system. While implementations should make every effort to report consistent device UUIDs
    /// > across driver versions, applications should avoid relying on the persistence of this value
    /// > for uses other than identifying compatible devices for external object sharing purposes.
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceIDProperties.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceIDProperties {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub deviceUUID: u8,
        pub driverUUID: u8,
        pub deviceLUID: u8,
        pub deviceNodeMask: u32,
        pub deviceLUIDValid: VkBool32,
    }

    pub type VkPhysicalDeviceIDPropertiesKHR = VkPhysicalDeviceIDProperties;

    /// Structure containing driver identification information
    ///
    /// sType **must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`
    ///
    /// # todo
    /// Finish implementing documentation.
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDriverProperties.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceDriverProperties {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub driverID: VkDriverId,
        pub driverName: [c_char; VK_MAX_DRIVER_NAME_SIZE],
        pub driverInfo: [c_char; VK_MAX_DRIVER_INFO_SIZE],
        pub conformanceVersion: VkConformanceVersion
    }

    /// Structure containing PCI bus information of a physical device
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT
    ///
    /// # todo
    /// Finish documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub pciDomain: u32,
        pub pciBus: u32,
        pub pciDevice: u32,
        pub pciFunction: u32,
    }

    /// Structure containing DRM information of a physical device
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT
    ///
    /// Extension `VK_EXT_physical_device_drm` must be enabled.
    ///
    /// # todo
    /// Finish documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDrmPropertiesEXT.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceDrmPropertiesEXT {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub hasPrimary: VkBool32,
        pub hasRender: VkBool32,
        pub primaryMajor: i32,
        pub primaryMinor: i32,
        pub renderMajor: i32,
        pub renderMinor: i32,
    }

    /// Structure containing information about integer dot product support for a physical device
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES
    ///
    /// VK_KHR_shader_integer_dot_product extension must be enabled
    ///
    /// # todo Documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderIntegerDotProductProperties.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceShaderIntegerDotProductProperties {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
        pub integerDotProduct8BitSignedAccelerated: VkBool32,
        pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
        pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
        pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
        pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
        pub integerDotProduct16BitSignedAccelerated: VkBool32,
        pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
        pub integerDotProduct32BitSignedAccelerated: VkBool32,
        pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
        pub integerDotProduct64BitSignedAccelerated: VkBool32,
        pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
        pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,

    }

    pub type VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR =
        VkPhysicalDeviceShaderIntegerDotProductProperties;

    /// Structure containing image processing properties
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM
    ///
    /// Provided by `VK_QCOM_image_processing`
    ///
    /// # todo Documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageProcessingPropertiesQCOM.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceImageProcessingPropertiesQCOM {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub maxWeightFilterPhases: u32,
        pub maxWeightFilterDimension: VkExtent2D,
        pub maxBlockMatchRegion: VkExtent2D,
        pub maxBoxFilterBlockSize: VkExtent2D,
    }

    /// Structure containing information about tile image support for a physical device
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT
    ///
    /// Provided by `VK_EXT_shader_tile_image`
    ///
    /// # todo Documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderTileImagePropertiesEXT.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceShaderTileImagePropertiesEXT {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub shaderTileImageCoherentReadAccelerated: VkBool32,
        pub shaderTileImageReadSampleFromPixelRateInvocation: VkBool32,
        pub shaderTileImageReadFromHelperInvocation: VkBool32,
    }

    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM
    ///
    /// Structure containing image processing2 properties
    ///
    /// Provided by VK_QCOM_image_processing2
    ///
    /// # todo Documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageProcessing2PropertiesQCOM.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceImageProcessing2PropertiesQCOM {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub maxBlockMatchWindow: VkExtent2D,
    }

    /// Structure containing information about driver layering for a physical device
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT
    ///
    /// Provided by `VK_MSFT_layered_driver`
    ///
    /// # todo Documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLayeredDriverPropertiesMSFT.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceLayeredDriverPropertiesMSFT {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub underlyingAPI: VkLayeredDriverUnderlyingApiMSFT,
    }

    /// Structure containing scheduling control properties of a physical device
    ///
    /// sType must be VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM
    ///
    /// Provided by `VK_ARM_scheduling_controls`
    ///
    /// # todo Documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSchedulingControlsPropertiesARM.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkPhysicalDeviceSchedulingControlsPropertiesARM {
        pub sType: VkStructureType,
        pub pNext: *mut c_void,
        pub schedulingControlsFlags: VkPhysicalDeviceSchedulingControlsFlagsARM
    }

    /// Structure providing information about a queue family
    ///
    /// # todo Documentation
    ///
    /// https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyProperties.html
    #[repr(C)]
    #[derive(Debug)]
    pub struct VkQueueFamilyProperties {
        queueFlags: VkQueueFlags,
        queueCount: u32,
        timestampValidBits: u32,
        minImageTransferGranularity: VkExtent3D
    }
}