use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use python_comm_macros::auto_func_name2;
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

/// 北京时间, 时间戳, 精确到毫秒
///
/// ## 用法
///
/// ```
/// use python_comm::prelude::*;
///
/// let ts = bj_timestamp_millis();
/// assert!(ts > 1623913021000 && ts < 1623913021000 + 86400000 * 36500);
/// ```
///
#[inline]
pub fn bj_timestamp_millis() -> i64 {
    Utc::now().timestamp_millis()
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
/// let a = bjtc_nt(ts, 10).unwrap();
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
#[auto_func_name2]
pub fn bjtc_fd(timestamp: f64) -> Result<NaiveDate, anyhow::Error> {
    bjtc_nd(
        timestamp as i64,
        ((timestamp - (timestamp as i64 as f64)) * 1000.0) as u32,
    )
    .or_else(|err| raise_error!(__func__, timestamp, "\n", err))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name2]
pub fn bjtc_fs(timestamp: f64) -> Result<String, anyhow::Error> {
    Ok(bjtc_ts(&bjtc_ft(timestamp).or_else(|err| {
        raise_error!(__func__, timestamp, "\n", err)
    })?))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name2]
pub fn bjtc_ft(timestamp: f64) -> Result<NaiveDateTime, anyhow::Error> {
    bjtc_nt(
        timestamp as i64,
        ((timestamp - (timestamp as i64 as f64)) * 1000.0) as u32,
    )
    .or_else(|err| raise_error!(__func__, timestamp, "\n", err))
}

// nx

/// 见 bjtc_df
#[inline]
#[auto_func_name2]
pub fn bjtc_nd(timestamp: i64, millis: u32) -> Result<NaiveDate, anyhow::Error> {
    Ok(bjtc_td(&bjtc_nt(timestamp, millis).or_else(|err| {
        raise_error!(__func__, timestamp, "\n", err)
    })?))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name2]
pub fn bjtc_ns(timestamp: i64, millis: u32) -> Result<String, anyhow::Error> {
    Ok(bjtc_ts(&bjtc_nt(timestamp, millis).or_else(|err| {
        raise_error!(__func__, timestamp, "\n", err)
    })?))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name2]
pub fn bjtc_nt(timestamp: i64, millis: u32) -> Result<NaiveDateTime, anyhow::Error> {
    NaiveDateTime::from_timestamp_opt(timestamp, millis * 1000000)
        .ok_or(raise_error!(__func__, format!("无效时间戳 {}", timestamp)))
}

// sx

/// 见 bjtc_df
#[inline]
#[auto_func_name2]
pub fn bjtc_sd(text: &String) -> Result<NaiveDate, anyhow::Error> {
    NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .or_else(|err| raise_error!(__func__, text, "\n", err))
}

/// 见 bjtc_df
#[inline]
#[auto_func_name2]
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

/// 转换 duration 为 timestamp, 精确到毫秒
///
/// 在同一时刻记录 DateTime 为 anchor, 创建 time::Instant 为 now,
/// 则在之后的任意时刻
///
/// bjtc_from_duration 转换 now.duration() 为 timestamp,
///
/// bjtc_to_duration 转换 timestamp 为 now.duration()
///
/// ## 用法
///
/// ```
/// use chrono::Utc;
/// use python_comm::prelude::*;
/// use std::{thread, time};
///
/// let anchor = Utc::now();
/// let now = time::Instant::now();
///
/// thread::sleep(time::Duration::from_millis(700));  // 0.7 秒
/// let t1 = bjtc_from_duration(&anchor, now.elapsed().as_secs_f64() * 1000.0);
/// let t2 = bjtc_to_duration(&anchor, t1).unwrap();
/// let diff = t2.as_secs_f64() - 0.7;
/// assert!(diff > -0.5 && diff < 0.5);
///
/// let t1 = bjtc_from_duration(&anchor, 1000.0);
/// let t3 = bjtc_to_duration(&anchor, t1 - 1000);
/// assert_eq!(t3.is_err(), false);
/// assert_eq!(t3.unwrap().as_secs_f64(), 0.0);
/// let t4 = bjtc_to_duration(&anchor, t1 - 1001);
/// assert_eq!(t4.is_err(), true);
///
/// let diff = bjtc_to_duration(&anchor, bj_timestamp_millis())
///   .unwrap()
///   .as_secs_f64()
///   - 0.7;
/// assert!(diff > -0.5 && diff < 0.5);
/// ```
///
pub fn bjtc_from_duration(anchor: &DateTime<Utc>, millis: f64) -> i64 {
    (*anchor + chrono::Duration::milliseconds(millis as i64)).timestamp_millis()
}

/// 转换 timestamp 为 duration
#[auto_func_name2]
pub fn bjtc_to_duration(
    anchor: &DateTime<Utc>,
    timestamp_millis: i64,
) -> Result<time::Duration, anyhow::Error> {
    let elapsed = bjtc_nt(timestamp_millis / 1000, (timestamp_millis % 1000) as u32)
        .or_else(|err| raise_error!(__func__, timestamp_millis, "\n", err))?
        - anchor.naive_utc();

    if elapsed.num_milliseconds() >= 0 {
        Ok(time::Duration::from_millis(
            elapsed.num_milliseconds() as u64
        ))
    } else {
        Err(raise_error!(
            __func__,
            format!("{} 结果为负值 {}", timestamp_millis, elapsed.num_seconds())
        ))
    }
}

#[cfg(test)]
mod bjtc_test {
    use super::*;
    use std::thread;

    #[test]
    fn test_bjtc_from_to_duration() {
        let anchor = Utc::now();
        let now = time::Instant::now();

        thread::sleep(time::Duration::from_millis(700)); // 0.7 秒
        let t1 = bjtc_from_duration(&anchor, now.elapsed().as_secs_f64() * 1000.0);
        let t2 = bjtc_to_duration(&anchor, t1).unwrap();
        let diff = t2.as_secs_f64() - 0.7;
        assert!(diff > -0.0014 && diff < 0.0014);

        let t1 = bjtc_from_duration(&anchor, 1000.0);
        let t3 = bjtc_to_duration(&anchor, t1 - 1000);
        assert_eq!(t3.is_err(), false);
        assert_eq!(t3.unwrap().as_secs_f64(), 0.0);
        let t4 = bjtc_to_duration(&anchor, t1 - 1001);
        assert_eq!(t4.is_err(), true);

        let diff = bjtc_to_duration(&anchor, bj_timestamp_millis())
            .unwrap()
            .as_secs_f64()
            - 0.7;
        assert!(diff > -0.0014 && diff < 0.0014);
    }
}
