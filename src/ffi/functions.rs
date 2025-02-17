use libc::c_char;

use crate::ffi::c_types::enums::VkResult;
use crate::ffi::c_types::fn_ptrs::PFN_vkVoidFunction;
use crate::ffi::c_types::objects::*;

#[allow(non_snake_case, non_camel_case_types)]
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


    pub fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult;

    /// Destroy an instance of Vulkan
    ///
    /// # Parameters
    /// - instance is the handle of the instance to destroy.
    ///
    /// - pAllocator controls host memory allocation as described in the Memory Allocation chapter.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#_vkdestroyinstance3
    pub fn vkDestroyInstance(
        instance: VkInstance,
        pAllocator: *const VkAllocationCallbacks
    );

    /// Returns properties of a physical device
    ///
    /// # Parameters
    /// `physicalDevice` is the handle to the physical device whose properties will be queried.
    ///
    /// `pProperties` is a pointer to a VkPhysicalDeviceProperties structure in which properties are returned
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#vkGetPhysicalDeviceProperties
    pub fn vkGetPhysicalDeviceProperties(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties
    );

    /// Returns properties of a physical device
    ///
    /// # Parameters
    /// `physicalDevice` is the handle to the physical device whose properties will be queried.
    ///
    /// `pProperties` is a pointer to a VkPhysicalDeviceProperties structure in which properties are returned
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#vkGetPhysicalDeviceProperties2
    ///
    /// # todo
    /// This currently isn't working properly. Our tests are providing a access violation. We need
    /// to review this and come back to it.
    pub fn vkGetPhysicalDeviceProperties2(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties2
    );

    /// Returns properties of a physical device
    ///
    /// # Parameters
    /// `physicalDevice` is the handle to the physical device whose properties will be queried.
    ///
    /// `pProperties` is a pointer to a VkPhysicalDeviceProperties structure in which properties are returned
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#vkGetPhysicalDeviceProperties2
    pub fn vkGetPhysicalDeviceProperties2KHR(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties2
    );

    /// Create a new device instance
    ///
    /// # Parameters
    /// - `physicalDevice` must be one of the device handles returned from a call to
    ///   vkEnumeratePhysicalDevices (see Physical Device Enumeration).
    ///
    /// - `pCreateInfo` is a pointer to a VkDeviceCreateInfo structure containing information about
    ///   how to create the device.
    ///
    /// - `pAllocator` controls host memory allocation as described in the Memory Allocation chapter.
    ///
    /// - `pDevice` is a pointer to a handle in which the created VkDevice is returned.
    ///
    /// # Description
    /// vkCreateDevice verifies that extensions and features requested in the ppEnabledExtensionNames
    /// and pEnabledFeatures members of pCreateInfo, respectively, are supported by the implementation.
    /// If any requested extension is not supported, vkCreateDevice must return
    /// VK_ERROR_EXTENSION_NOT_PRESENT. If any requested feature is not supported, vkCreateDevice
    /// must return VK_ERROR_FEATURE_NOT_PRESENT. Support for extensions can be checked before
    /// creating a device by querying vkEnumerateDeviceExtensionProperties. Support for features can
    /// similarly be checked by querying vkGetPhysicalDeviceFeatures.
    ///
    /// After verifying and enabling the extensions the VkDevice object is created and returned to the
    /// application. If a requested extension is only supported by a layer, both the layer and the
    /// extension need to be specified at vkCreateInstance time for the creation to succeed.
    ///
    /// Multiple logical devices can be created from the same physical device. Logical device
    /// creation may fail due to lack of device-specific resources (in addition to the other errors).
    /// If that occurs, vkCreateDevice will return VK_ERROR_TOO_MANY_OBJECTS.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#vkCreateDevice
    pub fn vkCreateDevice(
        phsyicalDevice: VkPhysicalDevice,
        pCreateInfo: *const VkDeviceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDevice: *mut VkDevice
    ) -> VkResult;

    /// Enumerates the physical devices accessible to a Vulkan instance
    ///
    /// # Parameters
    /// instance is a handle to a Vulkan instance previously created with vkCreateInstance.
    ///
    /// pPhysicalDeviceCount is a pointer to an integer related to the number of physical devices available or queried, as described below.
    ///
    /// pPhysicalDevices is either NULL or a pointer to an array of VkPhysicalDevice handles.
    ///
    /// # Description
    /// If `pPhysicalDevices` is NULL, then the number of physical devices available is returned in
    /// `pPhysicalDeviceCount`. Otherwise, `pPhysicalDeviceCount` must point to a variable set by the
    /// user to the number of elements in the `pPhysicalDevices` array, and on return the variable is
    /// overwritten with the number of handles actually written to `pPhysicalDevices`. If
    /// `pPhysicalDeviceCount` is less than the number of physical devices available, at most
    /// `pPhysicalDeviceCount` structures will be written. If `pPhysicalDeviceCount` is smaller than the
    /// number of physical devices available, `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`,
    /// to indicate that not all the available physical devices were returned.
    ///
    /// https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html#_vkenumeratephysicaldevices3
    pub fn vkEnumeratePhysicalDevices(
        instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice
    ) -> VkResult;

    pub fn vkResetEvent(
        device: VkDevice,
        event: VkEvent,
    ) -> VkResult;
}