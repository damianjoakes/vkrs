use libc::c_char;

use crate::ffi::c_types::enums::VkResult;
use crate::ffi::c_types::fn_ptrs::PFN_vkVoidFunction;
use crate::ffi::c_types::objects::VkDevice;
use crate::ffi::c_types::objects::VkEvent;
use crate::ffi::c_types::objects::VkInstance;

#[link(name = "C:\\VulkanSDK\\1.4.304.0\\Lib\\vulkan-1")]
unsafe extern "C" {
    /// Obtains a function pointer for any Vulkan command.
    ///
    /// `vk_instance` is the instance that the function pointer will be compatible with,
    /// or NULL for commands not dependent on any instance.
    ///
    /// `p_name` is the name of the command to obtain.
    pub fn vkGetInstanceProcAddr(
        vk_instance: VkInstance,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction;

    pub fn vkResetEvent(
        device: VkDevice,
        event: VkEvent,
    ) -> VkResult;
}