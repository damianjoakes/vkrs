use crate::ffi::c_types::fn_ptrs::PFN_vkrsEnumerateInstanceVersion;
use crate::ffi::functions::vkGetInstanceProcAddr;
use crate::vkrs_get_instance_proc_addr_ext;

#[test]
fn vk_get_instance_proc_test() {
    let vk_instance = core::ptr::null_mut();
    let mut api_version = 0u32;

    unsafe {
        let pfn = vkrs_get_instance_proc_addr_ext!(
            vk_instance,
            "vkEnumerateInstanceVersion",
            PFN_vkrsEnumerateInstanceVersion
        );

        let a: i32 = pfn(&mut api_version).into();

        println!("{}", a);
        println!("{}", api_version);
    }
}

// # todo
// This currently doesn't work. We need to create the vkCreateDevice function before we continue.
// #[test]
// fn vk_get_device_proc_test() {
//     let vk_device = 1u32;
//     let mut api_version = 0u32;
//
//     unsafe {
//         let pfn = vkrs_get_device_proc_addr_ext!(
//             vk_device,
//             "vkEnumerateInstanceVersion",
//             PFN_vkrsEnumerateInstanceVersion
//         );
//
//         let a: i32 = pfn(&mut api_version).into();
//
//         println!("{}", a);
//         println!("{}", api_version);
//     }
// }