use pyo3::{proc_macro::pymodule, types::PyModule, PyResult, Python};

#[macro_use]
mod macros;

mod datetime;

mod textsearcher;

/// Other common basic functions.
///
/// # Usage
///
/// ```
/// use python_comm::basic_use::*;
/// ```
pub mod basic_use {
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
    // use chrono::{Datelike, Timelike};
}

/// Convert python objects to rust.
///
/// # Usage
///
/// ```
/// use python_comm::from_py_use::*;
/// ```
pub mod from_py_use {
    pub use crate::from_py;
    pub use chrono::Datelike;
    pub use pyo3::types::{PyAny, PyDict};
    pub use rust_decimal::{prelude::FromPrimitive, Decimal};
}

/// Raise anyhow::Error or pyo3::PyErr with file_name/line/function_name.
///
/// # Usage
///
/// ```
/// use python_comm::raise_error_use::*;
/// ```
pub mod raise_error_use {
    pub use crate::raise_error;
}

/// Convert rust objects to python.
///
/// # Usage
///
/// ```
/// use python_comm::to_py_use::*;
/// ```
pub mod to_py_use {
    pub use crate::to_py;
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
