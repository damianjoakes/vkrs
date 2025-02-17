pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;

/// VK_MAX_DRIVER_NAME_SIZE is the length in char values of an array containing a driver name string,
/// as returned in VkPhysicalDeviceDriverProperties::driverName.
pub const VK_MAX_DRIVER_NAME_SIZE: usize = 256;

/// Equivalent to VK_MAX_DRIVER_NAME_SIZE.
pub const VK_MAX_DRIVER_NAME_SIZE_KHR: usize = 256;

/// VK_MAX_DRIVER_INFO_SIZE is the length in char values of an array containing a driver information
/// string, as returned in VkPhysicalDeviceDriverProperties::driverInfo.
pub const VK_MAX_DRIVER_INFO_SIZE: usize = 256;

/// Equivalent to VK_MAX_DRIVER_INFO_SIZE.
pub const VK_MAX_DRIVER_INFO_SIZE_KHR: usize = 256;


pub const VK_UUID_SIZE: usize = 16;
pub const VK_LUID_SIZE: usize = VK_UUID_SIZE;

pub const VK_MAX_GLOBAL_PRIORITY_SIZE: usize = 16;
pub const VK_MAX_GLOBAL_PRIORITY_SIZE_KHR: usize = VK_MAX_GLOBAL_PRIORITY_SIZE;
pub const VK_MAX_GLOBAL_PRIORITY_SIZE_EXT: usize = VK_MAX_GLOBAL_PRIORITY_SIZE;

/// VK_MAX_DEVICE_GROUP_SIZE is the length of an array containing VkPhysicalDevice handle values
/// representing all physical devices in a group, as returned in
/// VkPhysicalDeviceGroupProperties::physicalDevices.
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;

/// Equivalent to VK_MAX_DEVICE_GROUP_SIZE.
pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: usize = VK_MAX_DEVICE_GROUP_SIZE;

