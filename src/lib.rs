#![doc = include_str!("../README.md")]

pub use libloading;
pub use libloading::{library_filename, Error, Library, Symbol};
/// 该宏将`extern "C"`块(或mod中的`extern "C"`块)中的方法、静态变量替换为同名结构并自动派生[`LibrarySymbol`]
pub use libloading_helper_macro::library;

/// 动态库符号定义
pub trait LibrarySymbol {
    /// 符号显示名称
    const NAME: &'static str;
    /// 符号。一般等价于`NAME` + `\0`
    const SYMBOL: &'static [u8];
    /// 符号对应的类型
    type Type;

    /// 从库中获取符号
    ///
    /// 调用该方法等价以下语句
    /// ```ignore
    /// unsafe { lib.get(Self::SYMBOL) }
    /// ```
    #[inline]
    fn get(lib: &Library) -> Result<Symbol<Self::Type>, Error> {
        unsafe { lib.get(Self::SYMBOL) }
    }
}

/// 快速调用库方法
///
/// # Examples
/// ```ignore
/// use libloading_helper::{call, library, Library};
///
/// #[library]
/// extern "C" {
///     fn test(a: i32, b: f64);
/// }
///
/// let lib = Library::new("lib").unwrap();
/// let _ = libloading_helper::call!(lib.test(1, 2.0));
/// ```
#[macro_export]
macro_rules! call {
    ($lib: ident . $method: ident ( $( $args: expr ),*  $(,)? ) ) => {
        <$method as $crate::LibrarySymbol>::get(&$lib).map(|f| f( $( $args, )* ))
    };
}
