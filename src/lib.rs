#[macro_use]
mod macros;

mod datetime;
mod limit_pack;
mod more_error;

#[cfg(feature = "use_sql")]
mod sql_date;

#[cfg(feature = "use_sql")]
mod sql_op;

#[cfg(feature = "use_tokio")]
mod tokio_helper;

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
                bj_date, bj_dates, bj_time, bj_time_init, bj_timeb, bj_times, bj_timestamp, bj_timestamp_millis,
                bjtc_bd, bjtc_bf, bjtc_bn, bjtc_bs, bjtc_bt, bjtc_df, bjtc_dn, bjtc_ds, bjtc_dt, bjtc_fb, bjtc_fd,
                bjtc_from_duration, bjtc_fs, bjtc_ft, bjtc_nb, bjtc_nd, bjtc_ns, bjtc_nt, bjtc_sb, bjtc_sd, bjtc_sf,
                bjtc_sn, bjtc_st, bjtc_tb, bjtc_td, bjtc_tf, bjtc_tn, bjtc_to_duration, bjtc_ts, bjtc_tt,
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
/// use python_comm::use_limit_pack::*;
///
/// #[derive(LimitPack)]
/// struct Abc {
///     a: i32,
///     b: &'static str,
///     c: i32,
///     d: Vec<Vec<i32>>,
///     e: Vec<i32>,
/// }
///
/// assert_eq!("01234567890123456789".to_limit_str3(4, 4, 12), "012345...8...456789");
/// assert_eq!(vec![0,1,2,3,4,5,6,7,8,9].to_limit_str3(4, 4, 12), "[0 0,1,...6...8,9 0]");
/// assert_eq!((0,1,2,3,4,5,6,7,8,9).to_limit_str3(4, 4, 12), "(0 0,1,2,3,4,5,6,7,8,9 0)");
/// assert_eq!((1,1.1,-1.1,true,false).to_limit_str3(4, 4, 12), "(0 1,1.1,-1.1,true,false 0)");
/// assert_eq!((1, "1", 1, vec![vec![1]], vec![1]).to_limit_str3(4, 4, 12), "(0 1,1,1,[1 [2 1 2] 1],[3 1 3] 0)");
/// assert_eq!(Abc{a:1, b:"1", c:1, d:vec![vec![1]], e:vec![1]}.to_limit_str3(4, 4, 12), "Abc(0 a:1,b:1,c:1,d:[1 [2 1 2] 1],e:[3 1 3] 0)");
/// ```
///
pub mod use_limit_pack {
    pub use {
        crate::limit_pack::{ForStruct, Limit, LimitPackAble},
        python_comm_macros::LimitPack,
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
///     File::open("not exist").m(m!(fname))?;
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
            more_error::{AddMoreError, AsMoreError, LessError, MoreError},
        },
        python_comm_macros::auto_func_name,
    };
}

/// ## Usage
///
/// ```
/// use python_comm::use_quick_assign::*;
///
/// #[derive(Default, QuickAssign)]
/// struct Abc {
///     a: i32,
///     b: &'static str,
/// }
///
/// let _ = Abc::default().a(1).b("2");
/// ```
///
pub mod use_quick_assign {
    pub use python_comm_macros::QuickAssign;
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

#[cfg(feature = "use_tokio")]
pub mod use_tokio {
    pub use crate::tokio_helper::{join_all, join_all_and_reduce, join_to_happy};
}
