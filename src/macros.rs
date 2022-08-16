/// Current version number
///
/// ## Usage
///
/// ```
/// use python_comm::use_basic::*;
///
/// let version = crate_version!();
/// assert_eq!(&version[0..3], "0.3");
/// ```
///
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// 用最短的代码引入文件名、行号
/// 1. 仅函数名
/// 2. 函数名 + 补充信息
/// 3. 函数名 + 补充信息, 直接构造
#[macro_export]
macro_rules! m {
    ($func:ident) => {
        (file!(), line!(), $func, "")
    };
    ($func:ident, $text:expr) => {
        (file!(), line!(), $func, $text)
    };
    ($func:ident, $text:expr, "more") => {
        MoreError::new(file!(), line!(), $func, $text)
    };
    ($func:ident, $text:expr, "result") => {
        Err(MoreError::new(file!(), line!(), $func, $text))
    };
}
