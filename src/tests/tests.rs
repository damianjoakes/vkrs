use crate::ffi::c_types::enums::VkStructureType;
use crate::ffi::c_types::fn_ptrs::PFN_vkrsEnumerateInstanceVersion;
use crate::ffi::c_types::objects::{VkApplicationInfo, VkInstance, VkInstanceCreateInfo, VkPhysicalDevice};
use crate::ffi::functions::{vkCreateInstance, vkEnumeratePhysicalDevices, vkGetInstanceProcAddr};
use crate::vkrs_get_instance_proc_addr_ext;
use libc::c_char;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::ptr::null;

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

#[test]
fn vk_enumerate_devices_test() {
    let application_name = CString::new("test-application").unwrap();
    let application_name_ptr = application_name.as_ptr();

    let engine_name = CString::new("test-engine").unwrap();
    let engine_name_ptr = engine_name.as_ptr();

    let vk_application_info = VkApplicationInfo {
        sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: null(),
        pApplicationName: application_name_ptr,
        applicationVersion: 0,
        pEngineName: engine_name_ptr,
        engineVersion: 0,
        apiVersion: 0,
    };

    let enable_extension_names: [*const c_char; 0] = unsafe { MaybeUninit::zeroed().assume_init() };
    let enable_layer_names: [*const c_char; 0] = unsafe { MaybeUninit::zeroed().assume_init() };

    let vk_create_instance_info = VkInstanceCreateInfo {
        flags: 0,
        enabledExtensionCount: 0,
        enabledLayerCount: 0,
        pNext: null(),
        pApplicationInfo: &vk_application_info,
        sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        ppEnabledExtensionNames: enable_extension_names.as_ptr(),
        ppEnabledLayerNames: enable_layer_names.as_ptr(),
    };

    // The code below is used when we need to implement custom memory allocators.
    //
    // let user_data = null_mut();
    // let vk_allocation_callbacks = VkAllocationCallbacks {
    //     pUserData: user_data,
    //     pfnAllocation: | p_user_data, size, alignment, allocation_scope | { },
    //     pfnReallocation: | p_user_data, p_original_data, size, alignment, allocation_scope | { },
    //     pfnFree: | p_user_data, p_memory| {  },
    //     pfnInternalAllocation: | p_user_date, size, allocation_type, allocation_scope | { },
    //     pfnInternalFree: | p_user_data, size, allocation_type, allocation_scope | { },
    // };

    // Create an empty, uninitialized slice of memory for the Vulkan instance.
    let mut vk_instance: MaybeUninit<VkInstance> = MaybeUninit::uninit();

    unsafe {
        // SAFETY: since the uninitialized memory is being used safely, this function will initialize
        // the slice of memory at which the VkInstance will sit.
        let result = vkCreateInstance(
            &vk_create_instance_info,
            null(),
            vk_instance.as_mut_ptr(),
        );

        println!("{:?}", result);
    }

    let vk_instance = unsafe {
        // SAFETY: This code is unreachable if the VkInstance wasn't previously initialized.
        vk_instance.assume_init()
    };

    // This will now hold a dispatchable handle to the VkInstance on the host.
    dbg!(vk_instance);

    let mut physical_device_count = 0u32;

    // Create an empty, uninitialized slice of memory for the physical devices.
    let mut vk_physical_devices: MaybeUninit<VkPhysicalDevice> = MaybeUninit::uninit();

    unsafe {
        let enumerate_result = vkEnumeratePhysicalDevices(
            vk_instance,
            &mut physical_device_count,
            vk_physical_devices.as_mut_ptr(),
        );

        dbg!(enumerate_result);
    }

    // This will now hold a dispatchable handle to a list of physical devices on the host.
    let vk_physical_devices = unsafe {
        vk_physical_devices.assume_init()
    };

    dbg!(vk_physical_devices);
    dbg!(physical_device_count);
}
