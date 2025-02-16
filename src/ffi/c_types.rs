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

pub type VkBool32 = u32;

/// A 64-bit bitmask representing a collection of flags.
pub type VkFlags64 = u64;

/// Type alias for VkFlags.
pub type VkInstanceCreateFlags = VkFlags;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkDeviceQueueCreateFlags = VkFlags;

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
    pub struct VkStructureType {
        bits: i32,
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
    pub struct VkSystemAllocationScope {
        bits: i32,
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
    pub struct VkInternalAllocationType {
        bits: i32,
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
}

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

pub mod extents {
    /// Represents a 2D extent.
    #[repr(C)]
    pub struct VkExtent2D {
        width: u32,
        height: u32,
    }

    /// Represents a 3D extent.
    #[repr(C)]
    pub struct VkExtent3D {
        width: u32,
        height: u32,
        depth: u32,
    }
}

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
    ///
    /// This macro should only be used for generating function pointers and function definitions for
    /// function which need a `vkrs`-scoped return type. These return types are prefixed with
    /// `PFN_vkrs`, and are used for producing strictly-typed function pointers based on
    /// Vulkan API functions.
    ///
    /// Assume that where a `PFN_vkVoidFunction` is returned, the function pointer type
    /// created by this will probably be the preferred return value.
    ///
    /// It is worth noting that both of these functions are callable, and each will work with the
    /// same functionality. The pointer function type is provided when library users want to use the
    /// function returned directly from the calling function. The function itself is used for
    /// library users who want to use the original Vulkan API names. In these instances, it is up to
    /// the library user to ensure the prerequisite calling code has been invoked.
    macro_rules! vkrs_create_function_with_typedef {
            (
                $func_name:ident,
                $ptr_name:ident,
                ($($arg_name:ident : $arg_type:ty),*)
                -> $ret_type:ty
            ) => {
                #[allow(non_camel_case_types, non_snake_case)]
                pub type $ptr_name = unsafe extern "system" fn($($arg_name: $arg_type),*) -> $ret_type;

                unsafe extern "C" {
                    #[allow(non_camel_case_types, non_snake_case)]
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
    #[allow(non_camel_case_types)]
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
    #[allow(non_camel_case_types, non_snake_case)]
    pub type PFN_vkAllocationFunction = unsafe extern "system" fn(
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
    #[allow(non_camel_case_types, non_snake_case)]
    pub type PFN_vkReallocationFunction = unsafe extern "system" fn(
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
    #[allow(non_camel_case_types, non_snake_case)]
    pub type PFN_vkFreeFunction = unsafe extern "system" fn(
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
    #[allow(non_camel_case_types, non_snake_case)]
    pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(
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
    #[allow(non_camel_case_types, non_snake_case)]
    pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(
        pUserData: *mut c_void,
        size: size_t,
        allocationType: VkInternalAllocationType,
        allocationScope: VkSystemAllocationScope,
    );

    // Create the definitions for vkEnumerateInstanceVersion.
    vkrs_create_function_with_typedef!(
        vkEnumerateInstanceVersion,
        PFN_vkrsEnumerateInstanceVersion,
        (
            p_api_version: *mut u32
        ) -> VkResult
    );
}

#[allow(non_camel_case_types)]
pub mod objects {
    use std::ffi::c_void;

    use libc::{c_char};
    use paste;

    use crate::ffi::c_types::enums::VkStructureType;
    use crate::ffi::c_types::fn_ptrs::*;

    use super::*;

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
    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct VkApplicationInfo {
        sType: VkStructureType,
        pNext: *const c_void,
        pApplicationName: *const c_char,
        applicationVersion: u32,
        pEngineName: *const c_char,
        engineVersion: u32,
        apiVersion: u32,
    }

    /// Object which specifies the parameters of a newly created instance.
    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct VkInstanceCreateInfo {
        sType: VkStructureType,
        pNext: *const c_void,
        flags: VkInstanceCreateFlags,
        pApplicationInfo: *const VkApplicationInfo,
        enabledLayerCount: u32,
        ppEnabledLayerNames: *const *const c_char,
        enabledExtensionCount: u32,
        ppEnabledExtensionNames: *const *const c_char,
    }

    /// Structure specifying parameters of a newly created device queue
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
    #[allow(non_snake_case)]
    pub struct VkDeviceQueueCreateInfo {
        sType: VkStructureType,
        pNext: *const c_void,
        flags: VkDeviceQueueCreateFlags,
        queueFamilyIndex: u32,
        queueCount: u32,
        pQueueProperties: *const f32,
    }

    /// Structure describing the fine-grained features that can be supported by an implementation.
    ///
    /// *see each struct member for details*
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#VkPhysicalDeviceFeatures
    #[repr(C)]
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
    pub struct VkDeviceCreateInfo {
        sType: VkStructureType,
        pNext: *const c_void,
        flags: VkDeviceCreateFlags,
        queueCreateInfoCount: u32,
        pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
        enabledLayerCount: u32,
        ppEnabledLayerNames: *const *const c_char,
        enabledExtensionCount: u32,
        ppEnabledExtensionNames: *const *const c_char,
        pEnabledFeatures: *const VkPhysicalDeviceFeatures,
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
    #[allow(non_snake_case)]
    pub struct VkAllocationCallbacks {
        pUserDate: *mut c_void,
        pfnAllocation: PFN_vkAllocationFunction,
        pfnReallocation: PFN_vkReallocationFunction,
        pfnFree: PFN_vkFreeFunction,
        pfnInternalAllocation: PFN_vkInternalAllocationNotification,
        pfnInternalFree: PFN_vkInternalFreeNotification,
    }
}