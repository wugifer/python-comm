/// Current version number
///
/// ## Usage
///
/// ```
/// use python_comm::use_basic::*;
///
/// let version = crate_version!();
/// assert_eq!(&version[0..3], "0.2");
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

/// Generate pyo3::PyErr or anyhow::Error with file name, line number and function name
///
/// ## Usage
///
/// The rust macro cannot write this, but if is expressed in a regular expression:
///
/// raise_error!( ("raw",)? ("py",)? \_\_func\_\_, ("some text",)? ("\n", inner_err)? )
///
/// raw:  return E,
/// ---:  return Result<_, E>
///
/// py:   return PyErr,
/// --:   return anyhow::Error
///
/// ```
/// use pyo3::{prepare_freethreaded_python, Python};
/// use python_comm::raise_error_use::*;
///
/// prepare_freethreaded_python();
///
/// let gil = Python::acquire_gil();
/// let py = gil.python();
///
/// #[auto_func_name]
/// fn test(py:Python) -> Result<(), anyhow::Error> {
///   let v = Ok(2).and(Err("e"));
///
///   // Usage 1: generate pyo3::PyErr
///   let _a = v.or_else(|err| raise_error!("py", __func__, "some text", "\n", err));
///
///   // Usage 2: generate anyhow::Error, including supplementary instruction and another error
///   let b = v.or_else(|err| raise_error!(__func__, "some text", "\n", err));
///
///   // Usage 3: generate anyhow::Error, including another error
///   let _c = v.or_else(|err| raise_error!(__func__, "\n", err));
///
///   // Usage 4: generate anyhow::Error, including supplementary description
///   let _d = v.or_else(|_err| raise_error!(__func__, "some text"));
///
///   b
/// }
///
/// let err = format!("{:?}", test(py).err().unwrap());
/// let msg1 = "Error: src\\macros.rs:20 test() some text\n\"e\"";
/// let msg2 = "Error: src/macros.rs:20 test() some text\n\"e\"";
/// assert!(err == msg1 || err == msg2, "\n left: {:?}\n  msg1:{:?}\n  msg2:{:?}", err, msg1, msg2);
/// ```
///
/// See <https://danielkeep.github.io/tlborm/book/mbe-min-captures-and-expansion-redux.html>
///
/// Once captured, it can no longer be matched as general text, except ident / TT
///
/// stringify!/format! is built-in and cannot be matched by TT. Expr must be used
#[macro_export]
macro_rules! raise_error {
    ( "raw", "py", $func:ident, $text:expr, "\n", $err:expr ) => {     pyo3::PyErr::new::<pyo3::exceptions::PyException, _>(raise_error!(@base, $func, $text, "\n", $err ))  };
    ( "raw",       $func:ident, $text:expr, "\n", $err:expr ) => {     anyhow::anyhow!(                                     raise_error!(@base, $func, $text, "\n", $err ))  };
    (        "py", $func:ident, $text:expr, "\n", $err:expr ) => { Err(pyo3::PyErr::new::<pyo3::exceptions::PyException, _>(raise_error!(@base, $func, $text, "\n", $err ))) };
    (              $func:ident, $text:expr, "\n", $err:expr ) => { Err(anyhow::anyhow!(                                     raise_error!(@base, $func, $text, "\n", $err ))) };

    ( "raw", "py", $func:ident, $text:expr                  ) => {     pyo3::PyErr::new::<pyo3::exceptions::PyException, _>(raise_error!(@base, $func, $text             ))  };
    ( "raw",       $func:ident, $text:expr                  ) => {     anyhow::anyhow!(                                     raise_error!(@base, $func, $text             ))  };
    (        "py", $func:ident, $text:expr                  ) => { Err(pyo3::PyErr::new::<pyo3::exceptions::PyException, _>(raise_error!(@base, $func, $text             ))) };
    (              $func:ident, $text:expr                  ) => { Err(anyhow::anyhow!(                                     raise_error!(@base, $func, $text             ))) };

    ( "raw", "py", $func:ident,             "\n", $err:expr ) => {     pyo3::PyErr::new::<pyo3::exceptions::PyException, _>(raise_error!(@base, $func,        "\n", $err ))  };
    ( "raw",       $func:ident,             "\n", $err:expr ) => {     anyhow::anyhow!(                                     raise_error!(@base, $func,        "\n", $err ))  };
    (        "py", $func:ident,             "\n", $err:expr ) => { Err(pyo3::PyErr::new::<pyo3::exceptions::PyException, _>(raise_error!(@base, $func,        "\n", $err ))) };
    (              $func:ident,             "\n", $err:expr ) => { Err(anyhow::anyhow!(                                     raise_error!(@base, $func,        "\n", $err ))) };

    (@base, $func:ident, $text:expr, "\n", $err:expr) => {
        format!("Error: {}:{} {}() {}\n{:?}", file!(), line!(), $func, $text, $err)
    };
    (@base, $func:ident, "\n", $err:expr) => {
        format!("Error: {}:{} {}()\n{:?}", file!(), line!(), $func, $err)
    };
    (@base, $func:ident, $text:expr) => {
        format!("Error: {}:{} {}() {}", file!(), line!(), $func, $text)
    };
}
