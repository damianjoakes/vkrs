# vkrs
<sub>Raw C-style Vulkan FFI bindings for Rust</sub>

This is primarily a learning project of my own, to kill two birds with one stone 
(using Rust's FFI to compile C ABI-compatible code, and graphics computation).

These bindings do **_not_** contain any safe wrappers around the underlying C code they're calling.
These are (to the best of my ability) one-to-one definitions between the C Vulkan API and Rust. Things like
enums are a bit different, as Rust handles enums in a very different way than C, so these are represented
as `struct`s that use the `#[repr(transparent)]` macro to ensure the structs are laid in memory as integers, 
then the enum members are provided as `const`s within the struct.

## Documentation
*Almost* all Rustdoc comments are sourced directly from the API specification from Vulkan via LunarG's website.
[This can be found here.](https://vulkan.lunarg.com/doc/view/latest/windows/apispec.html) I do not claim to be the 
original author of this documentation. This information is provided as-is for the sole purpose of ensuring
extensive, easily accessible documentation.