//! python_comm prelude.
//!
//! The prelude re-exports most commonly used functions, macros, and traits from this crate.
//!
//! # 用法
//!
//! ```
//! #[allow(unused_imports)]
//! use python_comm::prelude::*;
//! ```

pub use crate::{
    crate_version,
    datetime::{
        bj_date, bj_dates, bj_time, bj_times, bj_timestamp, bj_timestamp_millis, bjtc_df, bjtc_dn,
        bjtc_ds, bjtc_dt, bjtc_fd, bjtc_from_duration, bjtc_fs, bjtc_ft, bjtc_nd, bjtc_ns, bjtc_nt,
        bjtc_sd, bjtc_st, bjtc_td, bjtc_tf, bjtc_tn, bjtc_to_duration, bjtc_ts,
    },
    from_py, raise_error, to_py,
};
pub use chrono::{Datelike, Timelike};
pub use cpython::ObjectProtocol;
pub use rust_decimal::{prelude::FromPrimitive, Decimal};
