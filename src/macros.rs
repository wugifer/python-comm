/// Current version number
///
/// ## Usage
///
/// ```
/// use python_comm::basic_use::*;
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

/// Extract the specified field from Python dict / obj
///
/// ## Usage
///
/// ```
/// use pyo3::{prepare_freethreaded_python, Python};
/// use python_comm::{from_py_use::*, raise_error_use::*};
/// use rust_decimal_macros::dec;
///
/// prepare_freethreaded_python();
///
/// let gil = Python::acquire_gil();
/// let py = gil.python();
/// let os = py.import("os").unwrap();
/// let pobj: &PyAny = os.getattr("environ").unwrap().extract().unwrap();
///
/// // Usage 1: extract the specified field from Python obj, where obj is a keyword
/// let pdict: &PyDict = from_py!(obj, pobj, "_data",).unwrap();
/// let error: Result<&PyDict, _> = from_py!(obj, pobj, "none",);
/// assert!(error.is_err());
///
/// // Usage 2: extract the specified field from Python dict, where dict is a keyword
/// // let path:String = from_py!(dict, pdict, "PATH").unwrap(); under linux the key is b'PATH'
/// let error:Result<String, _> = from_py!(dict, pdict, "none",);
/// assert!(error.is_err());
///
/// let locals = PyDict::new(py);
/// locals.set_item("decimal", py.import("decimal").unwrap()).unwrap();
/// locals.set_item("datetime", py.import("datetime").unwrap()).unwrap();
/// let pdict: &PyDict = py
///     .eval(
///         r#"{"time": datetime.datetime.now(), "num": decimal.Decimal('2.5'), "text": "abc"}"#,
///         None,
///         Some(&locals),
///     )
///     .unwrap()
///     .extract()
///     .unwrap();
///
/// // Usage 3: specify datetime type based on usage 1-2
/// let time = from_py!(dict, pdict, "time", datetime).unwrap();
/// assert!(time.date().year() >= 2021);
///
/// // Usage 4: specify Decimal type based on usage 1-2
/// let num = from_py!(dict, pdict, "num", Decimal).unwrap();
/// assert_eq!(num, dec!(2.5));
///
/// // Usage 5: specify other type based on usage 1-2
/// let text = from_py!(dict, pdict, "text", String).unwrap();
/// assert_eq!(text, "abc");
///
/// // Usage 6: specify the default value based on usage 1-5
/// let default = from_py!(dict, pdict, "none", default String::from("default")).unwrap();
/// assert_eq!(default, "default");
/// ```
///
/// See <https://danielkeep.github.io/tlborm/book/mbe-min-captures-and-expansion-redux.html>
///
/// Once captured, it can no longer be matched as general text, except ident / TT
///
/// Stringify is built-in and cannot be matched by TT. Expr must be used
#[macro_export]
macro_rules! from_py {
    // - 从 dict 中提取
    ( dict, $obj:tt, $any1:expr, default $default:expr ) => {
        from_py!(0 get_item, take, $obj, $any1, default $default)
    };

    // - 从 dict 中提取
    // expr 之后必须是逗号, 因此 any2 即使没有, 也需要前面的逗号
    ( dict, $obj:tt, $any1:expr, $($any2:tt),* ) => {
        from_py!(0 get_item, take, $obj, $any1, $($any2),*)
    };

    // - 从 obj 中提取
    ( obj, $obj:tt, $any1:expr, default $default:expr ) => {
        from_py!(0 getattr, ok, $obj, $any1, default $default)
    };

    // - 从 obj 中提取
    // expr 之后必须是逗号, 因此 any2 即使没有, 也需要前面的逗号
    ( obj, $obj:tt, $any1:expr, $($any2:tt),* ) => {
        from_py!(0 getattr, ok, $obj, $any1, $($any2),*)
    };

    // 0 - 指定缺省值
    ( 0 $fn1:ident, $fn2:ident, $obj:ident, $field:expr, default $default:expr ) => {
        {
            let ret: Result<_, anyhow::Error> = match from_py!(0 $fn1, $fn2, $obj, $field,) {
                Ok(object) => Ok(object),
                Err(_) => Ok($default),
            };
            ret
        }
    };

    // 0 - 不指定类型
    ( 0 $fn1:ident, $fn2:ident, $obj:ident, $field:expr, ) => {
        from_py!(1 $fn1, $fn2, $obj, $field).and_then(|object| {
            object.extract().or_else(|err| {
                let __func__ = "from_py!";
                raise_error!(__func__, format!("解析 {} 字段失败", $field), "\n", err)
            })
        })
    };

    // 0 - 指定 datetime 类型
    ( 0 $fn1:ident, $fn2:ident, $obj:ident, $field:expr, datetime ) => {
        from_py!(1 $fn1, $fn2, $obj, $field).and_then(|object| {
            object
                .call_method("timestamp", (), None)
                .or_else(|err| {
                    let __func__ = "from_py!";
                    raise_error!(__func__, $field, "\n", err)
                })
                .and_then(|tm_object| {
                    tm_object.extract::<f64>().or_else(|err| {
                        let __func__ = "from_py!";
                        raise_error!(__func__, $field, "\n", err)
                    })
                })
                .and_then(|timestamp| {
                    python_comm::basic_use::bjtc_ft(timestamp)
                        .or_else(|err| {
                            let __func__ = "from_py!";
                            raise_error!(__func__, $field, "\n", err)
                        })
                })
        })
    };

    // 0 - 指定 Decimal 类型
    ( 0 $fn1:ident, $fn2:ident, $obj:ident, $field:expr, Decimal ) => {
        from_py!(1 $fn1, $fn2, $obj, $field).and_then(|object| {
            object
                .call_method("as_integer_ratio", (), None)
                .or_else(|err| {
                    let __func__ = "from_py!";
                    raise_error!(__func__, $field, "\n", err)
                })
                .and_then(|ratio_object| {
                    ratio_object
                        .extract::<(i64, i64)>()
                        .or_else(|err| {
                            let __func__ = "from_py!";
                            raise_error!(__func__, $field, "\n", err)
                        })
                        .and_then(|ii| {
                            Decimal::from_i128(ii.0 as i128)
                                .ok_or_else(|| {
                                    let __func__ = "from_py!";
                                    raise_error!(__func__, format!("转换 {} 字段={} 失败", $field, ii.0))
                                })
                                .and_then(|d1| {
                                    Decimal::from_i128(ii.1 as i128)
                                        .ok_or_else(|| {
                                            let __func__ = "from_py!";
                                            raise_error!(
                                                __func__,
                                                format!("转换 {} 字段={} 失败", $field, ii.1)
                                            )
                                        })
                                        .and_then(|d2| Ok(d1 / d2))
                                })
                        })
                })
        })
    };

    // 0 - 指定其它类型
    ( 0 $fn1:ident, $fn2:ident, $obj:ident, $field:expr, $type:ty ) => {
        from_py!(1 $fn1, $fn2, $obj, $field).and_then(|object| {
            object.extract::<$type>().or_else(|err| {
                let __func__ = "from_py!";
                raise_error!(__func__, $field, "\n", err)
            })
        })
    };

    // 1 - 提取指定字段
    ( 1 $fn1:ident, $fn2:ident, $obj:ident, $field:expr ) => {
        $obj.$fn1($field).$fn2().ok_or_else(|| {
            let __func__ = "from_py!";
            raise_error!(__func__, format!("获取 {} 字段失败", $field))
        });
    };
}

/// Generate pyo3:: pyerr or anyhow:: error with file name, line number and function
///
/// ## Usage
///
/// ```
/// use pyo3::{prepare_freethreaded_python, Python};
/// use python_comm::raise_error_use::*;
/// use python_comm_macros::auto_func_name;
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
///   let _d = v.or_else(|_err| Err(raise_error!(__func__, "some text")));
///
///   b
/// }
///
/// let err = format!("{:?}", test(py).err().unwrap());
/// let msg1 = "Error: src\\macros.rs:21 test() some text\n\"e\"";
/// let msg2 = "Error: src/macros.rs:21 test() some text\n\"e\"";
/// assert!(err == msg1 || err == msg2, "\n left: {:?}\n  msg1:{:?}\n  msg2:{:?}", err, msg1, msg2);
/// ```
///
#[macro_export]
macro_rules! raise_error {
    ("py", $func:ident, $text:expr, "\n", $err:expr) => {
        Err(pyo3::PyErr::new::<pyo3::exceptions::PyException, _>(
            format!(
                "Some error in rust.\nError: {}:{} {}() {}\n{:?}",
                file!(),
                line!(),
                $func,
                $text,
                $err
            ),
        ))
    };
    ($func:ident, $text:expr, "\n", $err:expr) => {
        Err(anyhow::anyhow!(
            "Error: {}:{} {}() {}\n{:?}",
            file!(),
            line!(),
            $func,
            $text,
            $err
        ))
    };
    ($func:ident, "\n", $err:expr) => {
        Err(anyhow::anyhow!(
            "Error: {}:{} {}()\n{:?}",
            file!(),
            line!(),
            $func,
            $err
        ))
    };
    ($func:ident, $text:expr) => {
        anyhow::anyhow!("Error: {}:{} {}() {}", file!(), line!(), $func, $text)
    };
}

/// Set the specified fields in Python dict / obj
///
/// ## Usage
///
/// ```
/// use pyo3::{prepare_freethreaded_python, Python};
/// use python_comm::{from_py_use::*, raise_error_use::*, to_py_use::*};
/// use rust_decimal_macros::dec;
///
/// prepare_freethreaded_python();
///
/// let gil = Python::acquire_gil();
/// let py = gil.python();
///
/// let locals = PyDict::new(py);
///
/// let _ = py.run(
///     r#"
/// class Something():
///   def __init__(self):
///     self.a=1
///     self.b=1
///
/// class Factory():
///   def create_decimal(self, s):
///     import decimal  # 真实代码不用在这里 import, 嵌入在 py.run 中时, 其它地方 import 总是失败
///     return decimal.Decimal(s)
///
/// some = Something()
///
/// factory = Factory()
/// "#,
///     None,
///     Some(&locals),
/// ).unwrap();
///
/// let some: &PyAny = from_py!(dict, locals, "some",).unwrap();
/// let factory: &PyAny = from_py!(dict, locals, "factory",).unwrap();
///
/// let a:i32 = from_py!(obj, some, "a",).unwrap();
/// assert_eq!(a, 1);
///
/// // Usage 1: set the specified field in Python obj, where obj is a keyword
/// let _ = to_py!(py, obj, some, [
///     ("a", 2),
///     ("b", 3),
/// ]).unwrap();
/// let a:i32 = from_py!(obj, some, "a",).unwrap();
/// assert_eq!(a, 2);
///
/// // Usage 2: set the specified field in Python dict, where dict is a keyword
///
/// // Usage 3: specify decimal type on the basis of usage 1-2,
/// // need to provide a factory object that implements create_decimal()
/// let _ = to_py!(py, obj, some, factory, [
///     ("a", dec!(2.5), Decimal)
/// ]).unwrap();
/// let a = from_py!(obj, some, "a", Decimal).unwrap();
/// assert_eq!(a, dec!(2.5));
///
/// // Note: the decimal module of Python 3.10 supports capsule, 
/// // which can be created directly instead
/// ```
///
#[macro_export]
macro_rules! to_py {
    // 0. 设置 dict
    ( $py:tt, dict, $obj:tt, $($any:tt),+ ) => {
        to_py!(set_item, $py, $obj, $($any),+)
    };

    // 1. 设置 obj
    ( $py:tt, obj, $obj:tt, $($any:tt),+ ) => {
        to_py!(setattr, $py, $obj, $($any),+)
    };

    // 无 factory 参数, 自动填充 no_factory, 实际不使用
    ( $func:ident, $py:expr, $obj:expr, [ $($any:tt),+ $(,)* ] ) => {
        to_py!($func, $py, $obj, "no_factory", [ $($any),+ ])
    };

    // 含 factory 参数, 展开 []
    ( $func:ident, $py:expr, $obj:expr, $factory:expr, [ $($any:tt),+ $(,)* ] ) => {
        {
            let mut ret1 = Ok(());
            $(
                let ret2 = to_py!($func, $py, $obj, $factory, $any);
                if ret2.is_err() {
                    ret1 = ret2;
                }
            )+
            ret1
        }
    };

    // 展开式, 含类型 Decimal
    ( $func:ident, $py:expr, $obj:expr, $factory:expr, ($field:expr, $value:expr, Decimal) ) => {
        $factory
            .call_method("create_decimal", ($value.to_string(),), None)
            .or_else(|err| {
                let __func__ = "to_py!";
                raise_error!(__func__, $field, "\n", err)
            })
            .and_then(|object| to_py!($func, $py, $obj, $factory, ($field, object)))
    };

    // 展开式, 内置类型
    ( $func:ident, $py:expr, $obj:expr, $factory:expr, ($field:expr, $value:expr, $any:tt) ) => {
        {
            let __func__ = "to_py!";
            $obj.$func($field, $value).or_else(
                |err| raise_error!(__func__, $field, "\n", err)
            ).and_then(|x| Ok(()))
        }
    };

    // 展开式, 无类型
    ( $func:ident, $py:expr, $obj:expr, $factory:expr, ($field:expr, $value:expr) ) => {
        {
            let __func__ = "to_py!";
            $obj.$func($field, $value).or_else(
                |err| raise_error!(__func__, $field, "\n", err)
            ).and_then(|x| Ok(()))
        }
    }
}
