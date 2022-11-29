#![cfg(test)]

use libloading_helper::{library, LibrarySymbol};

macro_rules! assert_gen {
    ($( $path: path )?) => {
        $( use $path::*; )?

        assert_eq!(ffi_void::NAME, "ffi_void");
        assert_eq!(ffi_args::NAME, "ffi_args");
        assert_eq!(ffi_ret::NAME, "ffi_ret");
        assert_eq!(ffi_all::NAME, "ffi_all");
        assert_eq!(A::NAME, "A");

        assert_eq!(ffi_void::SYMBOL, b"ffi_void\0");
        assert_eq!(ffi_args::SYMBOL, b"ffi_args\0");
        assert_eq!(ffi_ret::SYMBOL, b"ffi_ret\0");
        assert_eq!(ffi_all::SYMBOL, b"ffi_all\0");
        assert_eq!(A::SYMBOL, b"A\0");

        assert_eq!(std::any::type_name::<<ffi_void as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn()");
        assert_eq!(std::any::type_name::<<ffi_args as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn(i32, u64, ...)");
        assert_eq!(std::any::type_name::<<ffi_ret as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn() -> i32");
        assert_eq!(std::any::type_name::<<ffi_all as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn(i32, u64, ...) -> i32");
        assert_eq!(std::any::type_name::<<A as LibrarySymbol>::Type>(), "*mut i32");
    };
}

#[test]
fn test_parse_mod() {
    #[library]
    mod ffi {
        extern "C" {
            pub fn ffi_void();

            pub fn ffi_args(a: i32, b: u64, ...);

            pub fn ffi_ret() -> i32;

            pub fn ffi_all(a: i32, b: u64, ...) -> i32;

            pub static A: i32;
        }
    }

    assert_gen!(ffi);
}

#[test]
fn test_parse_extern_c_block() {
    #[library]
    extern "C" {
        pub fn ffi_void();

        pub fn ffi_args(a: i32, b: u64, ...);

        pub fn ffi_ret() -> i32;

        pub fn ffi_all(a: i32, b: u64, ...) -> i32;

        pub static A: i32;
    }

    assert_gen!();
}

#[test]
#[ignore = "仅当`test.c`编译为动态库后手动调用"]
fn test_call() -> Result<(), libloading::Error> {
    #[library]
    mod test_call {
        extern "C" {
            pub fn add(a: i32, b: i32) -> i32;

            pub static STATIC_A: i32;
        }
    }

    use test_call::{add, STATIC_A};

    unsafe {
        let lib = libloading::Library::new(libloading::library_filename("test_call"))?;

        let a = lib.get::<<STATIC_A as LibrarySymbol>::Type>(STATIC_A::SYMBOL)?;
        assert_eq!(100i32, a.read());

        let add_fn = lib.get::<<add as LibrarySymbol>::Type>(add::SYMBOL)?;
        assert_eq!(100, add_fn(1, 99));
    }

    Ok(())
}
