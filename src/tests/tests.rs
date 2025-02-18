use crate::ffi::c_types::enums::VkStructureType;
use crate::ffi::c_types::fn_ptrs::PFN_vkEnumerateInstanceVersion;
use crate::ffi::c_types::objects::{VkApplicationInfo, VkInstance, VkInstanceCreateInfo, VkPhysicalDevice, VkPhysicalDevicePCIBusInfoPropertiesEXT, VkPhysicalDeviceProperties, VkPhysicalDeviceProperties2};
use crate::ffi::functions::{vkCreateInstance, vkDestroyInstance, vkEnumeratePhysicalDevices, vkGetInstanceProcAddr, vkGetPhysicalDeviceProperties, vkGetPhysicalDeviceProperties2};
use crate::vkrs_get_instance_proc_addr_ext;
use libc::c_char;
use std::ffi::{c_void, CStr, CString};
use std::mem;
use std::mem::MaybeUninit;
use std::ptr::{null, null_mut};


#[test]
fn vk_get_instance_proc_test() {
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
        flags: 0x00000001,
        enabledExtensionCount: 0,
        enabledLayerCount: 0,
        pNext: null(),
        pApplicationInfo: &vk_application_info,
        sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        ppEnabledExtensionNames: enable_extension_names.as_ptr(),
        ppEnabledLayerNames: enable_layer_names.as_ptr(),
    };

    // Create an empty, uninitialized slice of memory for the Vulkan instance.
    let mut vk_instance: MaybeUninit<VkInstance> = MaybeUninit::uninit();

    unsafe {
        // SAFETY: since the uninitialized memory is being used safely, this function will initialize
        // the slice of memory at which the VkInstance will sit.
        let _result = vkCreateInstance(
            &vk_create_instance_info,
            null(),
            vk_instance.as_mut_ptr(),
        );
    }

    let mut vk_instance = unsafe { vk_instance.assume_init() };

    let pfn = unsafe {
        vkrs_get_instance_proc_addr_ext!(
            vk_instance,
            "vkEnumerateInstanceVersion",
            PFN_vkEnumerateInstanceVersion
        )
    };

    let mut api_version = 0u32;
    let result = unsafe { pfn(&mut api_version) };

    dbg!(result);
    dbg!(api_version);
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
        flags: 0x00000001,
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

    // Need to assign
    let mut physical_device_count = 0u32;


    unsafe {
        let enumerate_result = vkEnumeratePhysicalDevices(
            vk_instance,
            &mut physical_device_count,
            null_mut(), // pass in a null value, so we can get the device count first before enumerating the devices.
        );
        dbg!(enumerate_result);
        dbg!(physical_device_count);
    }


    // Create an empty, uninitialized slice of memory for the physical devices.
    // Using mem::zeroed here, so that we can zero-fill enough space in memory for the devices.
    let mut vk_physical_devices: [MaybeUninit<VkPhysicalDevice>; 8] = unsafe { mem::zeroed() };

    unsafe {
        let enumerate_result = vkEnumeratePhysicalDevices(
            vk_instance,
            &mut physical_device_count,
            vk_physical_devices.as_mut_ptr() as *mut VkPhysicalDevice,
        );

        dbg!(enumerate_result);
    }

    unsafe {
        vkDestroyInstance(vk_instance, null());
    }
}

#[test]
fn vk_get_physical_device_properties_test() {
    let enable_extension_names: [*const c_char; 0] = unsafe { MaybeUninit::zeroed().assume_init() };
    let enable_layer_names: [*const c_char; 0] = unsafe { MaybeUninit::zeroed().assume_init() };

    let vk_create_instance_info = VkInstanceCreateInfo {
        flags: 0x00000001,
        enabledExtensionCount: 0,
        enabledLayerCount: 0,
        pNext: null(),
        pApplicationInfo: null(),
        sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        ppEnabledExtensionNames: enable_extension_names.as_ptr(),
        ppEnabledLayerNames: enable_layer_names.as_ptr(),
    };

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

    // Need to assign
    let mut physical_device_count = 0u32;


    unsafe {
        let enumerate_result = vkEnumeratePhysicalDevices(
            vk_instance,
            &mut physical_device_count,
            null_mut(), // pass in a null value, so we can get the device count first before enumerating the devices.
        );
        dbg!(enumerate_result);
        dbg!(physical_device_count);
    }


    // Create an empty, uninitialized slice of memory for the physical devices.
    // Using mem::zeroed here, so that we can zero-fill enough space in memory for the devices.
    let mut vk_physical_devices: [MaybeUninit<VkPhysicalDevice>; 8] = unsafe { mem::zeroed() };

    unsafe {
        let enumerate_result = vkEnumeratePhysicalDevices(
            vk_instance,
            &mut physical_device_count,
            vk_physical_devices.as_mut_ptr() as *mut VkPhysicalDevice,
        );

        dbg!(enumerate_result);
    }

    let mut physical_device_properties: MaybeUninit<VkPhysicalDeviceProperties> = MaybeUninit::uninit();

    unsafe {
        vkGetPhysicalDeviceProperties(vk_physical_devices[0].assume_init(), physical_device_properties.as_mut_ptr());

        let device_properties = physical_device_properties.assume_init();
        let device_name = CStr::from_ptr(device_properties.deviceName.as_ptr());
        dbg!(device_properties);
        dbg!(device_name);
    }

    unsafe {
        vkDestroyInstance(vk_instance, null());
    }
}

#[test]
fn vk_get_physical_device_properties_2_test() {

    let vk_create_instance_info = VkInstanceCreateInfo {
        flags: 0x00000001,
        enabledExtensionCount: 0,
        enabledLayerCount: 0,
        pNext: null(),
        pApplicationInfo: null(),
        sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        ppEnabledExtensionNames: null(),
        ppEnabledLayerNames: null(),
    };

    // Create an empty, uninitialized slice of memory for the Vulkan instance.
    let mut vk_instance: MaybeUninit<VkInstance> = MaybeUninit::uninit();

    unsafe {
        // SAFETY: since the uninitialized memory is being used safely, this function will initialize
        // the slice of memory at which the VkInstance will sit.
        let _result = vkCreateInstance(
            &vk_create_instance_info,
            null(),
            vk_instance.as_mut_ptr(),
        );
    }

    let vk_instance = unsafe {
        // SAFETY: This code is unreachable if the VkInstance wasn't previously initialized.
        vk_instance.assume_init()
    };

    let mut physical_device_count = 0u32;

    unsafe {
        let _enumerate_result = vkEnumeratePhysicalDevices(
            vk_instance,
            &mut physical_device_count,
            null_mut(), // pass in a null value, so we can get the device count first before enumerating the devices.
        );
    }

    // Create an empty, uninitialized slice of memory for the physical devices.
    // Using mem::zeroed here, so that we can zero-fill enough space in memory for the devices.
    let mut vk_physical_devices: [MaybeUninit<VkPhysicalDevice>; 8] = unsafe { mem::zeroed() };

    unsafe {
        let _enumerate_result = vkEnumeratePhysicalDevices(
            vk_instance,
            &mut physical_device_count,
            vk_physical_devices.as_mut_ptr() as *mut VkPhysicalDevice,
        );
    }


    let mut device_pci_properties = VkPhysicalDevicePCIBusInfoPropertiesEXT {
        sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT,
        pNext: null_mut(),
        pciDomain: 0,
        pciBus: 0,
        pciDevice: 0,
        pciFunction: 0,
    };

    let mut physical_device_properties: MaybeUninit<VkPhysicalDeviceProperties> = MaybeUninit::uninit();

    let device = unsafe { vk_physical_devices[0].assume_init() };

    let physical_device_properties = unsafe {
        vkGetPhysicalDeviceProperties(device, physical_device_properties.as_mut_ptr());

        let device_properties = physical_device_properties.assume_init();
        device_properties
    };

    // This isn't supported by my driver, so this test was useless. The implementation is working
    // now, though.
    let mut physical_device_properties_2 = VkPhysicalDeviceProperties2 {
        sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2,
        pNext: std::ptr::from_mut(&mut device_pci_properties) as *mut c_void,
        properties: physical_device_properties,
    };

    unsafe {
        vkGetPhysicalDeviceProperties2(device, &mut physical_device_properties_2);
    }

    dbg!(&physical_device_properties_2);
    dbg!(&device_pci_properties);

    unsafe {
        vkDestroyInstance(vk_instance, null());
    }
}