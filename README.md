# libloading_helper

将`extern "C"`块(或mod中的`extern "C"`块)中的方法、静态变量替换为同名结构并自动派生[`LibrarySymbol`]

# Example

```rust
use libloading_helper::{library, LibrarySymbol};

#[library]
mod ffi {
    extern "C" {
        pub fn ffi_all(a: i32, b: u64, ...) -> i32;
    }
}

#[library]
extern "C" {
    pub static A: i32;
}

assert_eq!(ffi::ffi_all::NAME, "ffi_all");
assert_eq!(A::NAME, "A");

assert_eq!(ffi::ffi_all::SYMBOL, b"ffi_all\0");
assert_eq!(A::SYMBOL, b"A\0");

assert_eq!(std::any::type_name::<<ffi::ffi_all as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn(i32, u64, ...) -> i32");
assert_eq!(std::any::type_name::<<A as LibrarySymbol>::Type>(), "*mut i32");
```