#[cfg(test)]
use libloading_helper::{library, LibrarySymbol};

#[test]
fn test_gen() {
    #[library]
    mod ffi {
        extern "C" {
            pub fn ffi();

            pub fn ffi_args(a: i32, b: u64, ...);

            pub fn ffi_ret() -> i32;

            pub fn ffi_all(a: i32, b: u64, ...) -> i32;

            pub static A: i32;
        }
    }

    assert_eq!(ffi::ffi::NAME, "ffi");
    assert_eq!(ffi::ffi_args::NAME, "ffi_args");
    assert_eq!(ffi::ffi_ret::NAME, "ffi_ret");
    assert_eq!(ffi::ffi_all::NAME, "ffi_all");
    assert_eq!(ffi::A::NAME, "A");

    assert_eq!(ffi::ffi::SYMBOL, b"ffi\0");
    assert_eq!(ffi::ffi_args::SYMBOL, b"ffi_args\0");
    assert_eq!(ffi::ffi_ret::SYMBOL, b"ffi_ret\0");
    assert_eq!(ffi::ffi_all::SYMBOL, b"ffi_all\0");
    assert_eq!(ffi::A::SYMBOL, b"A\0");

    assert_eq!(
        std::any::type_name::<<ffi::ffi as LibrarySymbol>::Type>(),
        "unsafe extern \"C\" fn()"
    );
    assert_eq!(
        std::any::type_name::<<ffi::ffi_args as LibrarySymbol>::Type>(),
        "unsafe extern \"C\" fn(i32, u64, ...)"
    );
    assert_eq!(
        std::any::type_name::<<ffi::ffi_ret as LibrarySymbol>::Type>(),
        "unsafe extern \"C\" fn() -> i32"
    );
    assert_eq!(
        std::any::type_name::<<ffi::ffi_all as LibrarySymbol>::Type>(),
        "unsafe extern \"C\" fn(i32, u64, ...) -> i32"
    );
    assert_eq!(
        std::any::type_name::<<ffi::A as LibrarySymbol>::Type>(),
        "*mut i32"
    );
}

#[test]
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
        let lib = libloading::Library::new("./libtest_call.so")?;

        let a = lib.get::<<STATIC_A as LibrarySymbol>::Type>(STATIC_A::SYMBOL)?;

        assert_eq!(100i32, a.read());

        let add_fn = lib.get::<<add as LibrarySymbol>::Type>(add::SYMBOL)?;

        assert_eq!(100, add_fn(1, 99));
    }

    Ok(())
}
