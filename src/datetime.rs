use crate::use_m::*;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use python_comm_macros::auto_func_name;
use std::time;

/// Beijing time, date only
///
/// ## Usage
///
/// ```
/// use chrono::Datelike;
/// use python_comm::use_basic::*;
///
/// assert!(bj_date().year() >= 2021);
/// ```
///
#[inline]
pub fn bj_date() -> NaiveDate {
    bj_time().date()
}

/// Beijing time, date only, %Y-%m-%d format string
///
/// ## Usage
///
/// ```
/// use python_comm::use_basic::*;
///
/// assert!(bj_dates().starts_with("20"));
/// ````
///
#[inline]
pub fn bj_dates() -> String {
    bj_date().format("%Y-%m-%d").to_string()
}

/// Beijing time, date and time
///
/// ## Usage
///
/// ```
/// use chrono::Timelike;
/// use python_comm::use_basic::*;
///
/// assert!(bj_time().time().hour() >= 0);
/// ```
///
#[inline]
pub fn bj_time() -> NaiveDateTime {
    (Utc::now() + Duration::hours(8)).naive_utc()
}

/// Beijing time, date and time, %Y-%m-%d %H:%M:%S format string
///
/// ## Usage
///
/// ```
/// use python_comm::use_basic::*;
///
/// assert_eq!(bj_times().len(), 19);
/// ```
///
#[inline]
pub fn bj_times() -> String {
    bj_time().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Beijing time, timestamp
///
/// ## Usage
///
/// ```
/// use python_comm::use_basic::*;
///
/// let ts = bj_timestamp();
/// assert!(ts > 1623913021 && ts < 1623913021 + 86400 * 36500);
/// ```
///
#[inline]
pub fn bj_timestamp() -> i64 {
    Utc::now().timestamp()
}

/// Beijing time, timestamp, accurate to milliseconds
///
/// ## Usage
///
/// ```
/// use python_comm::use_basic::*;
///
/// let ts = bj_timestamp_millis();
/// assert!(ts > 1623913021000 && ts < 1623913021000 + 86400000 * 36500);
/// ```
///
#[inline]
pub fn bj_timestamp_millis() -> i64 {
    Utc::now().timestamp_millis()
}

/// bjtc_xy: Conversion between various expressions of Beijing time, x -> y
///
/// ## Conversion type
///
/// x,y is one of d,t,s,f,n
///
/// d: NaiveDate
///
/// t: NaiveDateTime
///
/// s: %Y-%m-%d or %Y-%m-%d %H:%M:%S
///
/// f: timestamp float
///
/// n: timestamp integer
///
/// ## Usage
///
/// ```
/// use python_comm::use_basic::*;
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

/// See bjtc_df
#[inline]
pub fn bjtc_dn(date: &NaiveDate) -> i64 {
    bjtc_tn(&bjtc_dt(date))
}

/// See bjtc_df
#[inline]
pub fn bjtc_ds(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// See bjtc_df
#[inline]
pub fn bjtc_dt(date: &NaiveDate) -> NaiveDateTime {
    date.and_hms_opt(0, 0, 0).unwrap() // (0,0,0) 不会返回 None
}

// fx

/// See bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_fd(timestamp: f64) -> Result<NaiveDate, MoreError> {
    bjtc_nd(
        timestamp as i64,
        ((timestamp - (timestamp as i64 as f64)) * 1000.0) as u32,
    )
    .m(m!(__func__, &format!("timestamp={}", timestamp)))
}

/// See bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_fs(timestamp: f64) -> Result<String, MoreError> {
    Ok(bjtc_ts(
        &bjtc_ft(timestamp).m(m!(__func__, &format!("timestamp={}", timestamp)))?,
    ))
}

/// See bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_ft(timestamp: f64) -> Result<NaiveDateTime, MoreError> {
    bjtc_nt(
        timestamp as i64,
        ((timestamp - (timestamp as i64 as f64)) * 1000.0) as u32,
    )
    .m(m!(__func__, &format!("timestamp={}", timestamp)))
}

// nx

/// See bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_nd(timestamp: i64, millis: u32) -> Result<NaiveDate, MoreError> {
    Ok(bjtc_td(
        &bjtc_nt(timestamp, millis).m(m!(__func__, &format!("timestamp={}", timestamp)))?,
    ))
}

/// See bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_ns(timestamp: i64, millis: u32) -> Result<String, MoreError> {
    Ok(bjtc_ts(
        &bjtc_nt(timestamp, millis).m(m!(__func__, &format!("timestamp={}", timestamp)))?,
    ))
}

/// See bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_nt(timestamp: i64, millis: u32) -> Result<NaiveDateTime, MoreError> {
    NaiveDateTime::from_timestamp_opt(timestamp, millis * 1000000).ok_or(m!(
        __func__,
        &format!("invalid timestamp={}", timestamp),
        "more"
    ))
}

// sx

/// See bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_sd(text: &str) -> Result<NaiveDate, MoreError> {
    NaiveDate::parse_from_str(&text[..10], "%Y-%m-%d").m(m!(__func__, text))
}

/// See bjtc_df
#[inline]
#[auto_func_name]
pub fn bjtc_st(text: &str) -> Result<NaiveDateTime, MoreError> {
    NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S").m(m!(__func__, text))
}

// tx

/// See bjtc_df
#[inline]
pub fn bjtc_td(time: &NaiveDateTime) -> NaiveDate {
    time.date()
}

/// See bjtc_df
#[inline]
pub fn bjtc_tf(time: &NaiveDateTime) -> f64 {
    bjtc_tn(time) as f64
}

/// See bjtc_dn
#[inline]
pub fn bjtc_tn(time: &NaiveDateTime) -> i64 {
    time.timestamp()
}

/// See bjtc_df
#[inline]
pub fn bjtc_ts(time: &NaiveDateTime) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

//

/// Convert duration to timestamp, accurate to milliseconds
///
/// At the same time, record datetime as anchor, create time::instant as now,
/// then at any time after,
///
/// bjtc_from_duration convert now.duration() to timestamp,
///
/// bjtc_to_duration convert timestamp to now.duration()
///
/// ## Usage
///
/// ```
/// use chrono::Utc;
/// use python_comm::use_basic::*;
/// use std::{thread, time};
///
/// let anchor = Utc::now();
/// let now = time::Instant::now();
///
/// thread::sleep(time::Duration::from_millis(700));  // 0.7 seconds
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

/// Convert timestamp to duration, see bjtc_from_duration
#[auto_func_name]
pub fn bjtc_to_duration(anchor: &DateTime<Utc>, timestamp_millis: i64) -> Result<time::Duration, MoreError> {
    let elapsed = bjtc_nt(timestamp_millis / 1000, (timestamp_millis % 1000) as u32)
        .m(m!(__func__, &format!("timestamp={}", timestamp_millis)))?
        - anchor.naive_utc();

    if elapsed.num_milliseconds() >= 0 {
        Ok(time::Duration::from_millis(elapsed.num_milliseconds() as u64))
    } else {
        m!(
            __func__,
            &format!("{} 结果为负值 {}", timestamp_millis, elapsed.num_seconds()),
            "result"
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    fn test_bjtc_from_to_duration() {
        let anchor = Utc::now();
        let now = time::Instant::now();

        thread::sleep(time::Duration::from_millis(700)); // 0.7 秒
        let elapsed = now.elapsed().as_secs_f64();
        let t1 = bjtc_from_duration(&anchor, elapsed * 1000.0);
        let t2 = bjtc_to_duration(&anchor, t1).unwrap();
        let diff = t2.as_secs_f64() - elapsed;
        assert!(diff > -0.01 && diff < 0.01);

        let t1 = bjtc_from_duration(&anchor, 1000.0);
        let t3 = bjtc_to_duration(&anchor, t1 - 1000);
        assert_eq!(t3.is_err(), false);
        assert_eq!(t3.unwrap().as_secs_f64(), 0.0);
        let t4 = bjtc_to_duration(&anchor, t1 - 1001);
        assert_eq!(t4.is_err(), true);

        let diff = bjtc_to_duration(&anchor, bj_timestamp_millis()).unwrap().as_secs_f64() - 0.7;
        assert!(diff > -0.1 && diff < 0.1);
    }
}
