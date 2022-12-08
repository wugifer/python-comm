#[macro_use]
mod macros;

mod datetime;
mod limit_pack;
mod more_error;

#[cfg(feature = "use_sql")]
mod sql_date;

#[cfg(feature = "use_sql")]
mod sql_op;

pub mod textsearcher;

/// common basic functions.
///
/// # Usage
///
/// ```
/// use python_comm::use_basic::*;
/// ```
///
pub mod use_basic {
    pub use {
        crate::{
            crate_version,
            datetime::{
                bj_date, bj_dates, bj_time, bj_times, bj_timestamp, bj_timestamp_millis, bjtc_df, bjtc_dn, bjtc_ds,
                bjtc_dt, bjtc_fd, bjtc_from_duration, bjtc_fs, bjtc_ft, bjtc_nd, bjtc_ns, bjtc_nt, bjtc_sd, bjtc_st,
                bjtc_td, bjtc_tf, bjtc_tn, bjtc_to_duration, bjtc_ts,
            },
            ok_or_return, some_or_return,
            textsearcher::TextSearcher,
        },
        python_comm_macros::build_time,
        rust_decimal::{prelude::FromPrimitive, Decimal},
    };
}

/// ## Usage
///
/// ```
/// use python_comm::use_m::*;
/// use std::fs::File;
///
/// #[auto_func_name]
/// #[auto_func_name]
/// fn has_error(n: i32) -> Result<(), MoreError> {
///     File::open("not exist").m(m!(__func__))?;
///
///    Ok(())
/// }
///
/// fn main() {
///    if let Err(err) = has_error(0) {
///        println!("{}", err);
///    }
/// }
/// ```
///
pub mod use_m {
    pub use {
        crate::{
            m,
            more_error::{AddMore, MoreError},
        },
        python_comm_macros::auto_func_name,
    };
}

/// ## Usage
///
/// ```
/// use python_comm::use_limit_pack::*;
///
/// #[derive(LimitPack)]
/// struct Abc {
///     a: i32,
///     b: &'static str,
/// }
///
/// assert_eq!(Abc{a:1, b:"abcdefghijk"}.limit_pack(15), "(a:1,b:abcdef~)");
/// ```
///
pub mod use_limit_pack {
    pub use {
        crate::limit_pack::{LimitObj, LimitPackAble},
        python_comm_macros::LimitPack,
    };
}

#[cfg(feature = "use_sql")]
/// to use AsSqlModel
pub mod use_sql {
    pub use {
        crate::{
            sql_date::{SqlDate, SqlTime},
            sql_op::{CreateDbPool, DbPool, DbPoolArgs, SqlModel},
        },
        mysql::{
            params,
            prelude::{ConvIr, FromValue},
        },
        python_comm_macros::AsSqlModel,
    };
}
