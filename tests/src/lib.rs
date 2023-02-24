#![cfg(test)]

use libloading_helper::library;

#[test]
#[ignore = "仅当`test.c`编译为动态库后手动调用"]
fn library_mod() -> Result<(), libloading_helper::Error> {
    /// test
    #[library(TestCall)]
    #[allow(non_snake_case)]
    mod test_call {
        extern "C" {
            /// add num
            fn add(a: i32, b: i32) -> i32;

            /// STATIC
            static STATIC_A: i32;
        }
    }

    unsafe {
        let t = test_call::TestCall::load(&libloading_helper::library_filename("test_call"))?;

        assert_eq!(100i32, t.STATIC_A.read());

        assert_eq!(100, t.add(1, 99));
    }

    Ok(())
}

#[test]
#[ignore = "仅当`test.c`编译为动态库后手动调用"]
#[allow(non_snake_case)]
fn library_extern_block() -> Result<(), libloading_helper::Error> {
    /// test
    #[library(TestCall)]
    extern "C" {
        /// add num
        fn add(a: i32, b: i32) -> i32;

        /// STATIC
        static STATIC_A: i32;
    }

    unsafe {
        let t = TestCall::load(libloading_helper::library_filename("test_call"))?;

        assert_eq!(100i32, t.STATIC_A.read());

        assert_eq!(100, t.add(1, 99));
    }

    Ok(())
}
