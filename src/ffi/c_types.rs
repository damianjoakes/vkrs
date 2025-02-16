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

/// A 64-bit bitmask representing a collection of flags.
pub type VkFlags64 = u64;

/// Type alias for VkFlags.
pub type VkInstanceCreateFlags = VkFlags;

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

    #[allow(non_camel_case_types, non_snake_case)]
    pub type PFN_vkAllocationFunction = unsafe extern "system" fn(
        pUserData: *mut c_void,
        size: size_t,
        alignment: size_t,
        allocationScope: VkSystemAllocationScope,
    );

    #[allow(non_camel_case_types, non_snake_case)]
    pub type PFN_vkReallocationFunction = unsafe extern "system" fn(
        pUserData: *mut c_void,
        pOriginal: *mut c_void,
        size: size_t,
        alignment: size_t,
        allocationScope: VkSystemAllocationScope,
    );

    #[allow(non_camel_case_types, non_snake_case)]
    pub type PFN_vkFreeFunction = unsafe extern "system" fn(
        pUserData: *mut c_void,
        pMemory: *mut c_void,
    );

    #[allow(non_camel_case_types, non_snake_case)]
    pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(
        pUserData: *mut c_void,
        size: size_t,
        allocationType: VkInternalAllocationType,
        allocationScope: VkSystemAllocationScope,
    );

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

    use libc::c_char;
    use paste;

    use crate::ffi::c_types::{VkInstanceCreateFlags};
    use crate::ffi::c_types::enums::VkStructureType;
    use crate::ffi::c_types::fn_ptrs::{PFN_vkAllocationFunction, PFN_vkFreeFunction, PFN_vkInternalAllocationNotification, PFN_vkInternalFreeNotification, PFN_vkReallocationFunction};

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