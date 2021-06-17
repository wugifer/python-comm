use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use python_comm_macros::auto_func_name;
use std::time;

/// 北京时间, 日期
///
/// ## 用法
///
/// ```
/// use python_comm::prelude::*;
///
/// assert!(bj_date().year() >= 2021);
/// ```
///
#[inline]
pub fn bj_date() -> NaiveDate {
    bj_time().date()
}

/// 北京时间, 日期, %Y-%m-%d 格式字符串
///
/// ## 用法
///
/// ```
/// use python_comm::prelude::*;
///
/// assert!(bj_dates().starts_with("20"));
/// ````
///
#[inline]
pub fn bj_dates() -> String {
    bj_date().format("%Y-%m-%d").to_string()
}

/// 北京时间, 日期 + 时间
///
/// ## 用法
///
/// ```
/// use python_comm::prelude::*;
///
/// assert!(bj_time().time().hour() >= 0);
/// ```
///
#[inline]
pub fn bj_time() -> NaiveDateTime {
    (Utc::now() + Duration::hours(8)).naive_utc()
}

/// 北京时间, 日期 + 时间, %Y-%m-%d %H:%M:%S 格式字符串
///
/// ## 用法
///
/// ```
/// use python_comm::prelude::*;
///
/// assert_eq!(bj_times().len(), 19);
/// ```
///
#[inline]
pub fn bj_times() -> String {
    bj_time().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 北京时间, 时间戳
///
/// ## 用法
///
/// ```
/// use python_comm::prelude::*;
///
/// let ts = bj_timestamp();
/// assert!(ts > 1623913021 && ts < 1623913021 + 86400 * 36500);
/// ```
///
#[inline]
pub fn bj_timestamp() -> i64 {
    Utc::now().timestamp()
}

/// bjtc_xy 北京时间的各种表达方式之间转换
///
/// ## 转换类型
///
/// x,y 分别是 d,t,s,f,n 之一
///
/// d: NaiveDate
///
/// t: NaiveDateTime
///
/// s: %Y-%m-%d 或 %Y-%m-%d %H:%M:%S
///
/// f: timestamp 浮点数
///
/// n: timestamp 整数
///
/// ## 用法
///
/// ```
/// use python_comm::prelude::*;
///
/// let ts = bj_timestamp();
/// let a = bjtc_nt(ts).unwrap();
/// let b = bjtc_ts(&a);
/// let c = bjtc_st(&b).unwrap();
/// let d = bjtc_tf(&c);
/// assert_eq!(d as i64, ts);
/// ```
///
#[inline]
pub fn bjtc_df(date: &NaiveDate) -> f64 {
    bjtc_dn(date) as f64
}

/// 见 bjtc_df
#[inline]
pub fn bjtc_dn(date: &NaiveDate) -> i64 {
    bjtc_tn(&bjtc_dt(date))
}

/// 见 bjtc_df
#[inline]
pub fn bjtc_ds(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// 见 bjtc_df
#[inline]
pub fn bjtc_dt(date: &NaiveDate) -> NaiveDateTime {
    date.and_hms(0, 0, 0)
}

// fx

/// 见 bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_fd(timestamp: f64) -> Result<NaiveDate, anyhow::Error> {
    bjtc_nd(timestamp as i64).or_else(|err| raise_error!(__func__, timestamp, "\n", err))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_fs(timestamp: f64) -> Result<String, anyhow::Error> {
    Ok(bjtc_ts(&bjtc_ft(timestamp).or_else(|err| {
        raise_error!(__func__, timestamp, "\n", err)
    })?))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_ft(timestamp: f64) -> Result<NaiveDateTime, anyhow::Error> {
    bjtc_nt(timestamp as i64).or_else(|err| raise_error!(__func__, timestamp, "\n", err))
}

// nx

/// 见 bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_nd(timestamp: i64) -> Result<NaiveDate, anyhow::Error> {
    Ok(bjtc_td(&bjtc_nt(timestamp).or_else(|err| {
        raise_error!(__func__, timestamp, "\n", err)
    })?))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_ns(timestamp: i64) -> Result<String, anyhow::Error> {
    Ok(bjtc_ts(&bjtc_nt(timestamp).or_else(|err| {
        raise_error!(__func__, timestamp, "\n", err)
    })?))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_nt(timestamp: i64) -> Result<NaiveDateTime, anyhow::Error> {
    NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .ok_or(raise_error!(__func__, format!("无效时间戳 {}", timestamp)))
}

// sx

/// 见 bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_sd(text: &String) -> Result<NaiveDate, anyhow::Error> {
    NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .or_else(|err| raise_error!(__func__, text, "\n", err))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_st(text: &String) -> Result<NaiveDateTime, anyhow::Error> {
    NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S")
        .or_else(|err| raise_error!(__func__, text, "\n", err))
}

// tx

/// 见 bjtc_df
#[inline]
pub fn bjtc_td(time: &NaiveDateTime) -> NaiveDate {
    time.date()
}

/// 见 bjtc_df
#[inline]
pub fn bjtc_tf(time: &NaiveDateTime) -> f64 {
    bjtc_tn(time) as f64
}

/// 见 bjtc_dn
#[inline]
pub fn bjtc_tn(time: &NaiveDateTime) -> i64 {
    time.timestamp()
}

/// 见 bjtc_df
#[inline]
pub fn bjtc_ts(time: &NaiveDateTime) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

//

/// 转换 duration 为 timestamp, 舍弃不足 1 秒的时间
///
/// 在同一时刻记录 DateTime 为 anchor, 创建 time::Instant 为 now,
/// 则在之后的任意时刻
///
/// bjtc_from_duration 转换 now.duration() 为 timestamp,
///
/// bjtc_to_duration 转换 timestamp 为 now.duration()
///
/// bjtc_xxx_duration(), as_secs() 分别有一些舍入, 除极端情况外, 来回合计恰好 1 秒
///
/// ## 用法
///
/// ```
/// use chrono::Utc;
/// use python_comm::prelude::*;
/// use std::{thread, time};
///
/// let anchor = Utc::now();  // x.y 秒
/// let now = time::Instant::now();
///
/// thread::sleep(time::Duration::from_millis(2700));  // 2.7 秒
/// let t1 = bjtc_from_duration(&anchor, now.elapsed().as_secs());  // 先舍弃 0.7 秒, 再舍弃 0.y 秒 => x+2
/// let t2 = bjtc_to_duration(&anchor, t1).unwrap();  // 2-0.y 秒, 舍弃 1-0.y 秒
/// assert_eq!(t2.as_secs(), 1);
///
/// let t1 = bjtc_from_duration(&anchor, 1000);
/// let t3 = bjtc_to_duration(&anchor, t1 - 1000);
/// assert_eq!(t3.is_err(), false);
/// let t4 = bjtc_to_duration(&anchor, t1 - 1001);
/// assert_eq!(t4.is_err(), true);
/// assert_eq!(t3.unwrap().as_secs(), 0);
///
/// assert_eq!(
///   bjtc_to_duration(&anchor, bj_timestamp())
///     .unwrap()
///     .as_secs(),
///   2
/// );
/// ```
///
pub fn bjtc_from_duration(anchor: &DateTime<Utc>, duration: u64) -> i64 {
    (*anchor + chrono::Duration::seconds(duration as i64)).timestamp()
}

/// 转换 timestamp 为 duration
#[auto_func_name]
pub fn bjtc_to_duration(
    anchor: &DateTime<Utc>,
    timestamp: i64,
) -> Result<time::Duration, anyhow::Error> {
    let elapsed = bjtc_nt(timestamp).or_else(|err| raise_error!(__func__, timestamp, "\n", err))?
        - anchor.naive_utc();

    if elapsed.num_seconds() >= 0 {
        Ok(time::Duration::from_secs(elapsed.num_seconds() as u64))
    } else {
        Err(raise_error!(
            __func__,
            format!("{} 结果为负值 {}", timestamp, elapsed.num_seconds())
        ))
    }
}
