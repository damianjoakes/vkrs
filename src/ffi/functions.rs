use libc::c_char;

use crate::ffi::c_types::enums::VkResult;
use crate::ffi::c_types::fn_ptrs::PFN_vkVoidFunction;
use crate::ffi::c_types::objects::{VkAllocationCallbacks, VkDevice, VkInstanceCreateInfo};
use crate::ffi::c_types::objects::VkEvent;
use crate::ffi::c_types::objects::VkInstance;

#[link(name = "C:\\VulkanSDK\\1.4.304.0\\Lib\\vulkan-1")]
unsafe extern "C" {
    /// Obtains a function pointer for any Vulkan command, from a Vulkan *instance*.
    ///
    /// `vk_instance` is the instance that the function pointer will be compatible with,
    /// or NULL for commands not dependent on any instance.
    ///
    /// `p_name` is the name of the command to obtain.
    pub fn vkGetInstanceProcAddr(
        vk_instance: VkInstance,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction;

    /// Obtains a function pointer for any Vulkan command, from a Vulkan *device*.
    ///
    /// `vk_device` is the device that the function pointer will be compatible with.
    ///
    /// This function is necessary as the host may be an environment where multiple devices are in
    /// use, and have different implementations of the Vulkan API.
    pub fn vkGetDeviceProcAddr(
        vk_device: VkDevice,
        p_name: *const c_char
    ) -> PFN_vkVoidFunction;

    #[allow(non_snake_case)]
    pub fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    );

    pub fn vkResetEvent(
        device: VkDevice,
        event: VkEvent,
    ) -> VkResult;
}