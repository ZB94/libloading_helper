# libloading_helper

将`extern "C"`块(或mod中的`extern "C"`块)中的方法、静态变量生成到指定结构体中

# Example

```compile_fail
use libloading_helper::{library, Library, RawSymbol, Error};

#[library(Ffi)]
mod ffi {
    extern "C" {
        pub fn ffi_all(a: i32, b: u64) -> i32;
        pub static A: i32;
    }
}

// 将在`ffi`中生成以下代码

pub struct Ffi {
    pub A: RawSymbol<*mut i32>,
    ffi_all: RawSymbol<extern "C" unsafe fn(i32, u64) -> i32>
}

impl Ffi {
    pub unsafe fn load<S: AsRef<std::ffi::OsStr>>(library_path: S) -> Result<Self, Error> {
        ...
    }

    pub unsafe fn ffi_all(&self, a: i32, b: u64) -> i32 {
        ...
    }
}
```
