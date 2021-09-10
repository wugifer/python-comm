/// 当前版本号
///
/// ## 用法
///
/// ```
/// use python_comm::prelude::*;
///
/// let version = crate_version!();
/// assert_eq!(&version[0..3], "0.1");
/// ```
///
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// 从 python dict/obj 中提取指定字段
///
/// ## 用法
///
/// ```
/// use cpython::{Python, PyDict, PyObject};
/// use python_comm::prelude::*;
/// use rust_decimal_macros::dec;
///
/// let gil = Python::acquire_gil();
/// let py = gil.python();
/// let os = py.import("os").unwrap();
/// let pobj: PyObject = os.get(py, "environ").unwrap().extract(py).unwrap();
///
/// // 用法1：从 python obj 中提取指定字段, 其中 obj 是关键字
/// let pdict:PyDict = from_py!(py, obj, pobj, "_data",).unwrap();
/// let error:Result<PyDict, _> = from_py!(py, obj, pobj, "none",);
/// assert!(error.is_err());
///
/// // 用法2：从 python dict 中提取指定字段, 其中 dict 是关键字
/// // let path:String = from_py!(py, dict, pdict, "PATH").unwrap(); linux 下是 b'PATH' 为 key
/// let error:Result<String, _> = from_py!(py, dict, pdict, "none",);
/// assert!(error.is_err());
///
/// let locals = PyDict::new(py);
/// locals.set_item(py, "decimal", py.import("decimal").unwrap()).unwrap();
/// locals.set_item(py, "datetime", py.import("datetime").unwrap()).unwrap();
/// let pdict:PyDict = py
///     .eval(
///         r#"{"time": datetime.datetime.now(), "num": decimal.Decimal('2.5'), "text": "abc"}"#,
///         None,
///         Some(&locals),
///     )
///     .unwrap()
///     .extract(py)
///     .unwrap();
///
/// // 用法3：在用法 1-2 基础上指定 datetime 类型
/// let time = from_py!(py, dict, pdict, "time", datetime).unwrap();
/// assert!(time.date().year() >= 2021);
///
/// // 用法4：在用法 1-2 基础上指定 Decimal 类型
/// let num = from_py!(py, dict, pdict, "num", Decimal).unwrap();
/// assert_eq!(num, dec!(2.5));
///
/// // 用法5：在用法 1-2 基础上指定 其它 类型
/// let text = from_py!(py, dict, pdict, "text", String).unwrap();
/// assert_eq!(text, "abc");
///
/// // 用法6：在用法 1-5 基础上指定缺省值
/// let default = from_py!(py, dict, pdict, "none", default String::from("default")).unwrap();
/// assert_eq!(default, "default");
/// ```
///
/// 参考 <https://danielkeep.github.io/tlborm/book/mbe-min-captures-and-expansion-redux.html>
/// 一旦被捕获, 则不能再当作一般文本进行 match, ident/tt 除外
/// stringify 是内置的, 不能被 tt 匹配, 必须用 expr
#[macro_export]
macro_rules! from_py {
    // - 从 dict 中提取
    ( $py:tt, dict, $obj:tt, $any1:expr, default $default:expr ) => {
        from_py!(0 get_item, take, $py, $obj, $any1, default $default)
    };

    // - 从 dict 中提取
    // expr 之后必须是逗号, 因此 any2 即使没有, 也需要前面的逗号
    ( $py:tt, dict, $obj:tt, $any1:expr, $($any2:tt),* ) => {
        from_py!(0 get_item, take, $py, $obj, $any1, $($any2),*)
    };

    // - 从 obj 中提取
    ( $py:tt, obj, $obj:tt, $any1:expr, default $default:expr ) => {
        from_py!(0 getattr, ok, $py, $obj, $any1, default $default)
    };

    // - 从 obj 中提取
    // expr 之后必须是逗号, 因此 any2 即使没有, 也需要前面的逗号
    ( $py:tt, obj, $obj:tt, $any1:expr, $($any2:tt),* ) => {
        from_py!(0 getattr, ok, $py, $obj, $any1, $($any2),*)
    };

    // 0 - 指定缺省值
    ( 0 $fn1:ident, $fn2:ident, $py:ident, $obj:ident, $field:expr, default $default:expr ) => {
        {
            let ret: Result<_, anyhow::Error> = match from_py!(0 $fn1, $fn2, $py, $obj, $field,) {
                Ok(object) => Ok(object),
                Err(_) => Ok($default),
            };
            ret
        }
    };

    // 0 - 不指定类型
    ( 0 $fn1:ident, $fn2:ident, $py:ident, $obj:ident, $field:expr, ) => {
        from_py!(1 $fn1, $fn2, $py, $obj, $field).and_then(|object| {
            object.extract($py).or_else(|err| {
                let __func__ = "from_py!";
                raise_error!(__func__, format!("解析 {} 字段失败", $field), "\n", err)
            })
        })
    };

    // 0 - 指定 datetime 类型
    ( 0 $fn1:ident, $fn2:ident, $py:ident, $obj:ident, $field:expr, datetime ) => {
        from_py!(1 $fn1, $fn2, $py, $obj, $field).and_then(|object| {
            object
                .call_method($py, "timestamp", cpython::NoArgs, None)
                .or_else(|err| {
                    let __func__ = "from_py!";
                    raise_error!(__func__, $field, "\n", err)
                })
                .and_then(|tm_object| {
                    tm_object.extract::<f64>($py).or_else(|err| {
                        let __func__ = "from_py!";
                        raise_error!(__func__, $field, "\n", err)
                    })
                })
                .and_then(|timestamp| {
                    python_comm::prelude::bjtc_ft(timestamp)
                        .or_else(|err| {
                            let __func__ = "from_py!";
                            raise_error!(__func__, $field, "\n", err)
                        })
                })
        })
    };

    // 0 - 指定 Decimal 类型
    ( 0 $fn1:ident, $fn2:ident, $py:ident, $obj:ident, $field:expr, Decimal ) => {
        from_py!(1 $fn1, $fn2, $py, $obj, $field).and_then(|object| {
            object
                .call_method($py, "as_integer_ratio", cpython::NoArgs, None)
                .or_else(|err| {
                    let __func__ = "from_py!";
                    raise_error!(__func__, $field, "\n", err)
                })
                .and_then(|ratio_object| {
                    ratio_object
                        .extract::<(i64, i64)>($py)
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
    ( 0 $fn1:ident, $fn2:ident, $py:ident, $obj:ident, $field:expr, $type:ty ) => {
        from_py!(1 $fn1, $fn2, $py, $obj, $field).and_then(|object| {
            object.extract::<$type>($py).or_else(|err| {
                let __func__ = "from_py!";
                raise_error!(__func__, $field, "\n", err)
            })
        })
    };

    // 1 - 提取指定字段
    ( 1 $fn1:ident, $fn2:ident, $py:ident, $obj:ident, $field:expr ) => {
        $obj.$fn1($py, $field).$fn2().ok_or_else(|| {
            let __func__ = "from_py!";
            raise_error!(__func__, format!("获取 {} 字段失败", $field))
        });
    };
}

/// 生成带文件名、行号、函数的 cpython::PyErr 或 anyhow::Error
///
/// ## 用法
///
/// ```
/// use cpython::Python;
/// use python_comm::prelude::*;
/// use python_comm_macros::auto_func_name;
///
/// let gil = Python::acquire_gil();
/// let py = gil.python();
///
/// #[auto_func_name]
/// fn test(py:Python) -> Result<(), anyhow::Error> {
///   let v = Ok(2).and(Err("e"));
///
///   // 用法1：生成 cpython::PyErr
///   let _a = v.or_else(|err| raise_error!(py, __func__, "some text", "\n", err));
///
///   // 用法2：生成 anyhow::Error, 包含补充说明及另一个 error
///   let b = v.or_else(|err| raise_error!(__func__, "some text", "\n", err));
///
///   // 用法3：生成 anyhow::Error, 包含另一个 error
///   let _c = v.or_else(|err| raise_error!(__func__, "\n", err));
///
///   // 用法3：生成 anyhow::Error, 补充说明
///   let _d = v.or_else(|_err| Err(raise_error!(__func__, "some text")));
///
///   b
/// }
///
/// let err = format!("{:?}", test(py).err().unwrap());
/// let msg1 = "Error: src\\macros.rs:19 test() some text\n\"e\"";
/// let msg2 = "Error: src/macros.rs:19 test() some text\n\"e\"";
/// assert!(err == msg1 || err == msg2, "\n left: {:?}\n  msg1:{:?}\n  msg2:{:?}", err, msg1, msg2);
/// ```
///
#[macro_export]
macro_rules! raise_error {
    ($python:expr, $func:ident, $text:expr, "\n", $err:expr) => {
        Err(cpython::PyErr::new::<cpython::exc::Exception, _>(
            $python,
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

/// 设置 python dict/obj 中指定字段
///
/// ## 用法
///
/// ```
/// use cpython::{Python, PyDict, PyObject};
/// use python_comm::prelude::*;
/// use rust_decimal_macros::dec;
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
/// let some:PyObject = from_py!(py, dict, locals, "some",).unwrap();
/// let factory:PyObject = from_py!(py, dict, locals, "factory",).unwrap();
///
/// let a:i32 = from_py!(py, obj, some, "a",).unwrap();
/// assert_eq!(a, 1);
///
/// // 用法1：设置 python obj 中指定字段, 其中 obj 是关键字
/// let _ = to_py!(py, obj, some, [
///     ("a", 2),
///     ("b", 3),
/// ]).unwrap();
/// let a:i32 = from_py!(py, obj, some, "a",).unwrap();
/// assert_eq!(a, 2);
///
/// // 用法2：设置 python dict 中指定字段, 其中 dict 是关键字
///
/// // 用法3：在用法 1-2 基础上指定 Decimal 类型, 需要提供一个实现了 create_decimal 的 factory 对象
/// let _ = to_py!(py, obj, some, factory, [
///     ("a", dec!(2.5), Decimal)
/// ]).unwrap();
/// let a = from_py!(py, obj, some, "a", Decimal).unwrap();
/// assert_eq!(a, dec!(2.5));
///
/// // 注：python 3.10 的 decimal 模块支持 Capsule, 可以改为直接创建
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
            .call_method($py, "create_decimal", ($value.to_string(),), None)
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
            $obj.$func($py, $field, $value).or_else(
                |err| raise_error!(__func__, $field, "\n", err)
            ).and_then(|x| Ok(()))
        }
    };
    // 展开式, 无类型
    ( $func:ident, $py:expr, $obj:expr, $factory:expr, ($field:expr, $value:expr) ) => {
        {
            let __func__ = "to_py!";
            $obj.$func($py, $field, $value).or_else(
                |err| raise_error!(__func__, $field, "\n", err)
            ).and_then(|x| Ok(()))
        }
    }
}
