use crate::ffi::c_types::objects::VkInstance;
use libc::c_char;

use crate::ffi::c_types::fn_ptrs::PFN_vkVoidFunction;

#[allow(non_snake_case)]
unsafe extern "C" {
    pub fn vkGetInstanceProcAddr(
        vk_instance: VkInstance,
        pName: *const c_char,
    ) -> PFN_vkVoidFunction;
}
