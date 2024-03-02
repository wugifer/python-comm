use {
    crate::use_m::*,
    chrono::{DateTime, Datelike, FixedOffset, NaiveDate, TimeZone, Utc},
    python_comm_macros::auto_func_name,
    std::time,
};

// https://docs.python.org/3/library/datetime.html#datetime.datetime.fromisoformat

// 格式代码
// b:YYYY-MM-DDTHH:MM:SS               文本, 假定为 +8 时区
// d-date                              零点, 假定为 +8 时区
// f-float                             浮点时间戳
// n:int                               整数时间戳
// s:YYYY-MM-DDTHH:MM:SS+08:00         文本, 含 +8 时区
// t-time                              标准格式, 含 +8 时区

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
    bjtc_td(&bj_time())
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
    bjtc_ds(&bj_date())
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
pub fn bj_time() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
}

#[inline]
pub fn bj_time_init(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<FixedOffset> {
    FixedOffset::east_opt(8 * 3600)
        .unwrap()
        .with_ymd_and_hms(year, month, day, hour, min, sec)
        .single()
        .unwrap()
}

/// Beijing time, date and time, %Y-%m-%d %H:%M:%S format string
///
/// ## Usage
///
/// ```
/// use python_comm::use_basic::*;
///
/// assert_eq!(bj_timeb().len(), 19);
/// ```
///
#[inline]
pub fn bj_timeb() -> String {
    bjtc_tb(&bj_time())
}

/// Beijing time, date and time, %Y-%m-%d %H:%M:%S+08:00 format string
///
/// ## Usage
///
/// ```
/// use python_comm::use_basic::*;
///
/// assert_eq!(bj_times().len(), 25);
/// ```
///
#[inline]
pub fn bj_times() -> String {
    bjtc_ts(&bj_time())
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
    bj_time().timestamp()
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
    bj_time().timestamp_millis()
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
#[auto_func_name]
pub fn bjtc_bd(text: &str) -> Result<NaiveDate, MoreError> {
    bjtc_sd(&bjtc_bs(text)).m(m!(fname))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_bf(text: &str) -> Result<f64, MoreError> {
    bjtc_sf(&bjtc_bs(text)).m(m!(fname))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_bn(text: &str) -> Result<i64, MoreError> {
    bjtc_sn(&bjtc_bs(text)).m(m!(fname))
}

/// See bjtc_bd
#[inline]
pub fn bjtc_bs(text: &str) -> String {
    format!("{}+08:00", text)
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_bt(text: &str) -> Result<DateTime<FixedOffset>, MoreError> {
    bjtc_st(&bjtc_bs(text)).m(m!(fname))
}

/// See bjtc_bd
#[inline]
pub fn bjtc_df(date: &NaiveDate) -> f64 {
    bjtc_dn(date) as f64
}

/// See bjtc_bd
#[inline]
pub fn bjtc_dn(date: &NaiveDate) -> i64 {
    bjtc_tn(&bjtc_dt(date))
}

/// See bjtc_bd
#[inline]
pub fn bjtc_ds(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// See bjtc_bd
#[inline]
pub fn bjtc_dt(date: &NaiveDate) -> DateTime<FixedOffset> {
    FixedOffset::east_opt(8 * 3600)
        .unwrap()
        .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
        .unwrap()
}

// fx

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_fb(timestamp: f64) -> Result<String, MoreError> {
    bjtc_ft(timestamp)
        .m(m!(fname, &format!("timestamp={}", timestamp)))
        .map(|time| bjtc_tb(&time))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_fd(timestamp: f64) -> Result<NaiveDate, MoreError> {
    bjtc_ft(timestamp)
        .m(m!(fname, &format!("timestamp={}", timestamp)))
        .map(|time| bjtc_td(&time))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_fs(timestamp: f64) -> Result<String, MoreError> {
    bjtc_ft(timestamp)
        .m(m!(fname, &format!("timestamp={}", timestamp)))
        .map(|time| bjtc_ts(&time))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_ft(timestamp: f64) -> Result<DateTime<FixedOffset>, MoreError> {
    bjtc_nt(
        timestamp as i64,
        ((timestamp - (timestamp as i64 as f64)) * 1000.0) as u32,
    )
    .m(m!(fname, &format!("timestamp={}", timestamp)))
}

// nx

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_nb(timestamp: i64, millis: u32) -> Result<String, MoreError> {
    bjtc_nt(timestamp, millis)
        .m(m!(fname, &format!("timestamp={}, millis={}", timestamp, millis)))
        .map(|time| bjtc_tb(&time))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_nd(timestamp: i64, millis: u32) -> Result<NaiveDate, MoreError> {
    bjtc_nt(timestamp, millis)
        .m(m!(fname, &format!("timestamp={}, millis={}", timestamp, millis)))
        .map(|time| bjtc_td(&time))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_ns(timestamp: i64, millis: u32) -> Result<String, MoreError> {
    bjtc_nt(timestamp, millis)
        .m(m!(fname, &format!("timestamp={}, millis={}", timestamp, millis)))
        .map(|time| bjtc_ts(&time))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_nt(timestamp: i64, millis: u32) -> Result<DateTime<FixedOffset>, MoreError> {
    DateTime::from_timestamp(timestamp, millis * 1000000)
        .map(|t| t.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()))
        .ok_or(m!(fname, &format!("timestamp={}", timestamp), "more"))
}

// sx

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_sb(text: &str) -> String {
    text.replace("+08:00", "")
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_sd(text: &str) -> Result<NaiveDate, MoreError> {
    NaiveDate::parse_from_str(&text[..10], "%Y-%m-%d").m(m!(fname, text))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_sf(text: &str) -> Result<f64, MoreError> {
    bjtc_st(text)
        .m(m!(fname, &format!("text={}", text)))
        .map(|time| bjtc_tf(&time))
}

/// See bjtc_dn
#[inline]
#[auto_func_name]
pub fn bjtc_sn(text: &str) -> Result<i64, MoreError> {
    bjtc_st(text)
        .m(m!(fname, &format!("text={}", text)))
        .map(|time| bjtc_tn(&time))
}

/// See bjtc_bd
#[inline]
#[auto_func_name]
pub fn bjtc_st(text: &str) -> Result<DateTime<FixedOffset>, MoreError> {
    DateTime::parse_from_str(text, "%Y-%m-%dT%H:%M:%S%:z").m(m!(fname, text))
}

// tx

/// See bjtc_bd
#[inline]
pub fn bjtc_tb(time: &DateTime<FixedOffset>) -> String {
    bjtc_sb(&bjtc_ts(time))
}

/// See bjtc_bd
#[inline]
pub fn bjtc_td(time: &DateTime<FixedOffset>) -> NaiveDate {
    time.date_naive()
}

/// See bjtc_bd
#[inline]
pub fn bjtc_tf(time: &DateTime<FixedOffset>) -> f64 {
    bjtc_tn(time) as f64
}

/// See bjtc_dn
#[inline]
pub fn bjtc_tn(time: &DateTime<FixedOffset>) -> i64 {
    time.timestamp()
}

/// See bjtc_bd
#[inline]
pub fn bjtc_ts(time: &DateTime<FixedOffset>) -> String {
    bjtc_tt(time).format("%Y-%m-%dT%H:%M:%S%:z").to_string()
}

/// See bjtc_tt
#[inline]
pub fn bjtc_tt(time: &DateTime<FixedOffset>) -> DateTime<FixedOffset> {
    bjtc_nt(bjtc_tn(time), 0).unwrap()
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
        .m(m!(fname, &format!("timestamp={}", timestamp_millis)))?
        - anchor.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());

    if elapsed.num_milliseconds() >= 0 {
        Ok(time::Duration::from_millis(elapsed.num_milliseconds() as u64))
    } else {
        m!(
            fname,
            &format!("{} 结果为负值 {}", timestamp_millis, elapsed.num_seconds()),
            "result"
        )
    }
}

#[cfg(test)]
mod test {
    use chrono::Duration;

    use super::*;
    use std::thread;

    #[test]
    fn test_bjtc() {
        println!("date  {}", bj_date());
        println!("dates {}", bj_dates());
        println!("time  {}", bj_time());
        println!("times {}", bj_times());
        println!("timeb {}", bj_timeb());

        let b129 = "1970-01-02T09:00:00";
        let d129 = NaiveDate::from_ymd_opt(1970, 1, 2).unwrap();
        let e129 = FixedOffset::east_opt(7 * 3600)
            .unwrap()
            .with_ymd_and_hms(1970, 1, 2, 8, 0, 0)
            .single()
            .unwrap();
        let f129 = 86400.0 + 3600.0;
        let n129 = 86400 + 3600;
        let s129 = "1970-01-02T09:00:00+08:00";
        let t129 = bj_time_init(1970, 1, 2, 9, 0, 0);

        assert_eq!(bjtc_bd(b129).unwrap(), d129);
        assert_eq!(bjtc_bf(b129).unwrap(), f129);
        assert_eq!(bjtc_bn(b129).unwrap(), n129);
        assert_eq!(bjtc_bs(b129), s129);
        assert_eq!(bjtc_bt(b129).unwrap(), t129);

        assert_eq!(bjtc_df(&d129), f129 - 9.0 * 3600.0);
        assert_eq!(bjtc_dn(&d129), n129 - 9 * 3600);
        assert_eq!(bjtc_ds(&d129), "1970-01-02");
        assert_eq!(bjtc_dt(&d129), t129 - Duration::hours(9));

        assert_eq!(bjtc_fb(f129).unwrap(), b129);
        assert_eq!(bjtc_fd(f129).unwrap(), d129);
        assert_eq!(bjtc_fs(f129).unwrap(), s129);
        assert_eq!(bjtc_ft(f129).unwrap(), t129);

        assert_eq!(bjtc_nb(n129, 0).unwrap(), b129);
        assert_eq!(bjtc_nd(n129, 0).unwrap(), d129);
        assert_eq!(bjtc_ns(n129, 0).unwrap(), s129);
        assert_eq!(bjtc_nt(n129, 0).unwrap(), t129);

        assert_eq!(bjtc_sb(s129), b129);
        assert_eq!(bjtc_sd(s129).unwrap(), d129);
        assert_eq!(bjtc_sf(s129).unwrap(), f129);
        assert_eq!(bjtc_sn(s129).unwrap(), n129);
        assert_eq!(bjtc_st(s129).unwrap(), t129);

        assert_eq!(bjtc_tb(&e129), b129);
        assert_eq!(bjtc_td(&e129), d129);
        assert_eq!(bjtc_tf(&e129), f129);
        assert_eq!(bjtc_tn(&e129), n129);
        assert_eq!(bjtc_ts(&e129), s129);
        assert_eq!(bjtc_tt(&e129), t129);
    }

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
