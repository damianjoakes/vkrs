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
    use crate::ffi::c_types::enums::VkResult;

    /// Macro to simplify creation of function pointers and extern functions which call the
    /// Vulkan API.
    ///
    /// This macro should only be used for generating function pointers and function definitions for
    /// function which need a `vkrs`-scoped return type. These return types are prefixed with
    /// `PFN_vkrs`, and are used for producing strictly-typed function pointers based on
    /// Vulkan API functions.
    ///
    /// This simplifies the use of `PFN_vkVoidFunction`, since all Vulkan functions should exist
    ///
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
    use paste;

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

    vk_define_handle!(VkInstance);
    vk_define_handle!(VkDevice);
    vk_define_handle!(VkEvent);
}