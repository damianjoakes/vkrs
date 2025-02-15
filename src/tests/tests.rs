use std::ffi::CString;

use crate::ffi::c_types::fn_ptrs::PFN_vkrsEnumerateInstanceVersion;
use crate::ffi::functions::vkGetInstanceProcAddr;
use crate::vkrs_get_instance_proc_addr;

#[test]
fn vk_void_fn_test() {
    let vk_instance = core::ptr::null_mut();
    let mut api_version = 0u32;

    unsafe {
        let pfn = vkrs_get_instance_proc_addr!(
            vk_instance,
            "vkEnumerateInstanceVersion",
            PFN_vkrsEnumerateInstanceVersion
        );

        let a: i32 = pfn(&mut api_version).into();
        println!("{}", a);
    }
}