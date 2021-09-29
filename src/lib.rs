use pyo3::{proc_macro::pymodule, types::PyModule, PyResult, Python};

#[macro_use]
mod macros;

mod datetime;

mod textsearcher;

pub mod types;

/// to use AsDefaultStruct
///
/// ## Usage
///
/// ```
/// use python_comm::as_default_struct_use::*;
///
/// #[derive(AsDefaultStruct, Default)]
/// struct Class1 {
///   f1: i32,
///   f2: String,
///   f3: String,
/// }
///
/// let _ = Class1::default().f1(1).f2("2".to_string());
/// ```
///
pub mod as_default_struct_use {
    pub use python_comm_macros::AsDefaultStruct;
}

/// to use AsPythonDict
///
/// ## Usage
///
/// ```
/// use python_comm::{as_python_dict_use::*, use_pyo3::*};
/// use rust_decimal_macros::dec;
///
/// #[derive(AsPythonDict)]
/// struct ClassDict {
///   f1: i32,
///   f2: String,
///   f3: Decimal,
///   f4: NaiveDate,
///   f5: NaiveDateTime,
/// }
///
/// #[pyfunction]
/// fn func(a: ClassDict) -> Result<ClassDict, PyErr> {
///   Ok(ClassDict{
///     f1: 2,
///     f2: "2".to_string(),
///     f3: dec!(2),
///     f4: NaiveDate::from_ymd(2002, 2, 2),
///     f5: NaiveDate::from_ymd(2002, 2, 2).and_hms(2, 2, 2)
///   })
/// }
///
/// // In Python:
/// // b = func (
/// //     {
/// //       "f1": 1,
/// //       "f2": "1",
/// //       "f3": Decimal(1),
/// //       "f4": datetime.datetime(2001, 1, 1).date(),
/// //       "f5": datetime.datetime(2001, 1, 1, 1, 1, 1)
/// //     }
/// // )
/// // b["f1"]
/// ```
///
pub mod as_python_dict_use {
    pub use crate::raise_error_use::*;
    pub use python_comm_macros::AsPythonDict;
}

/// to use AsPythonObject
///
/// ## Usage
///
/// ```
/// use python_comm::{as_python_object_use::*, use_pyo3::*};
/// use rust_decimal_macros::dec;
///
/// #[derive(AsPythonObject)]
/// struct ClassObject {
///   f1: i32,
///   f2: String,
///   f3: Decimal,
///   f4: NaiveDate,
///   f5: NaiveDateTime,
/// }
///
/// #[pyfunction]
/// fn func(a: ClassObject) -> Result<ClassObject, PyErr> {
///   Ok(ClassObject{
///     f1: 2,
///     f2: "2".to_string(),
///     f3: dec!(2),
///     f4: NaiveDate::from_ymd(2002, 2, 2),
///     f5: NaiveDate::from_ymd(2002, 2, 2).and_hms(2, 2, 2)
///   })
/// }
///
/// // In Python:
/// // class C():
/// //   def __init__(self, f1, f2, f3, f4, f5):
/// //     self.f1 = f1
/// //     ...
/// //
/// // b = func (C(
/// //       1,
/// //       "1",
/// //       Decimal(1),
/// //       datetime.datetime(2001, 1, 1).date(),
/// //       datetime.datetime(2001, 1, 1, 1, 1, 1)
/// //     )
/// // )
/// // b.f1
/// ```
///
pub mod as_python_object_use {
    pub use crate::raise_error_use::*;
    pub use python_comm_macros::AsPythonObject;
}

/// to use AsSqlTable
pub mod as_sql_table_use {
    pub use crate::raise_error_use::*;
    pub use mysql::{
        params,
        prelude::{ConvIr, FromValue, Queryable},
    };
    pub use python_comm_macros::AsSqlTable;
}

/// Raise anyhow::Error or pyo3::PyErr with file_name/line/function_name.
///
/// # Usage
///
/// ```
/// use python_comm::raise_error_use::*;
/// ```
///
pub mod raise_error_use {
    pub use crate::raise_error;
    pub use python_comm_macros::auto_func_name;
}

/// common basic functions.
///
/// # Usage
///
/// ```
/// use python_comm::use_basic::*;
/// ```
///
pub mod use_basic {
    pub use crate::{
        crate_version,
        datetime::{
            bj_date, bj_dates, bj_time, bj_times, bj_timestamp, bj_timestamp_millis, bjtc_df,
            bjtc_dn, bjtc_ds, bjtc_dt, bjtc_fd, bjtc_from_duration, bjtc_fs, bjtc_ft, bjtc_nd,
            bjtc_ns, bjtc_nt, bjtc_sd, bjtc_st, bjtc_td, bjtc_tf, bjtc_tn, bjtc_to_duration,
            bjtc_ts,
        },
        textsearcher::TextSearcher,
    };
    pub use python_comm_macros::build_time;
    pub use rust_decimal::{prelude::FromPrimitive, Decimal};
}

/// to use pyo3::pyfunction
///
/// ## Usage
///
/// ```
/// use python_comm::{use_pyo3::*};
///
/// #[pyfunction]
/// fn func() -> Result<(), PyErr> {
///   Ok(())
/// }
/// ```
///
pub mod use_pyo3 {
    pub use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    pub use pyo3::{proc_macro::pyfunction, types::PyModule, wrap_pyfunction, PyErr, Python};
    pub use rust_decimal::Decimal;
}

fn initialize(module: &PyModule) -> PyResult<()> {
    textsearcher::initialize(module)?;

    Ok(())
}

/// 函数名必须和目标 xxx.pyd / xxx.so 相同, 和 cargo.toml 中 [lib].name 字段不要求（?）同名
/// 注意, 当前的注释作为 xxx.__doc__ 出现
#[cfg(debug_assertions)]
#[pymodule]
fn python_commd(_python: Python, module: &PyModule) -> PyResult<()> {
    initialize(module)
}

/// 函数名必须和目标 xxx.pyd / xxx.so 相同, 和 cargo.toml 中 [lib].name 字段不要求（?）同名
/// 注意, 当前的注释作为 xxx.__doc__ 出现
#[cfg(not(debug_assertions))]
#[pymodule]
fn python_comm(_python: Python, module: &PyModule) -> PyResult<()> {
    initialize(module)
}
