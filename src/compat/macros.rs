//! Contains macros for making code generation easier.


/// Boilerplate code for getting the function instance address, and casting it to an underlying
/// compatible Rust type.
///
/// `$func_type` should generally be a `vkrs`-scoped function pointer type definition.
#[macro_export]
macro_rules! vkrs_get_instance_proc_addr_ext {
    ($instance:expr, $func_name:literal, $func_type:ty) => {{
        use crate::ffi::c_types::fn_ptrs::PFN_vkVoidFunction;
        use std::ffi::CString;

        let c_name = CString::new($func_name).unwrap();
        let p_name = c_name.as_ptr();
        let function_ptr = vkGetInstanceProcAddr($instance, p_name);

        std::mem::transmute::<
            PFN_vkVoidFunction,
            $func_type
        >(function_ptr)
    }};
}

/// Boilerplate code for getting the function instance address, and casting it to an underlying
/// compatible Rust type.
///
/// `$func_type` should generally be a `vkrs`-scoped function pointer type definition.
#[macro_export]
macro_rules! vkrs_get_device_proc_addr_ext {
    ($device:expr, $func_name:literal, $func_type:ty) => {{
        use crate::ffi::c_types::fn_ptrs::PFN_vkVoidFunction;
        use std::ffi::CString;
        use std::mem;
        use std::ptr;

        let c_name = CString::new($func_name).unwrap();
        let p_name = c_name.as_ptr();
        let function_ptr = vkGetDeviceProcAddr($device, p_name);

        std::mem::transmute::<
            PFN_vkVoidFunction,
            $func_type
        >(function_ptr)
    }};
}