# libloading_helper

将`extern "C"`块(或mod中的`extern "C"`块)中的方法、静态变量生成到指定结构体中

# Example

```compile_fail
use libloading_helper::{library, Library, Symbol, Error};

#[library(Ffi)]
mod ffi {
    extern "C" {
        pub fn ffi_all(a: i32, b: u64) -> i32;
        pub static A: i32;
    }
}

// 将在`ffi`中生成以下代码

pub struct Ffi<'lib> {
    pub A: Symbol<'lib, *mut i32>,
    ffi_all: Symbol<'lib, extern "C" unsafe fn(i32, u64) -> i32>
}

impl<'lib> Ffi<'lib> {
    pub unsafe fn load(library: &'lib Library) -> Result<Self, Error> {
        ...
    }

    pub unsafe fn ffi_all(&self, a: i32, b: u64) -> i32 {
        ...
    }
}
```
