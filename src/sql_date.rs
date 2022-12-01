use {
    crate::{datetime::*, use_m::*},
    chrono::{NaiveDate, NaiveDateTime, NaiveTime},
    mysql::{
        prelude::{ConvIr, FromValue},
        FromValueError, Value,
    },
    std::fmt,
};

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct SqlDate {
    /// 日期
    sdate: String,

    /// 日期
    ndate: NaiveDate,
}

impl SqlDate {
    #[allow(dead_code)]
    #[inline]
    pub fn n(&self) -> &NaiveDate {
        &self.ndate
    }

    #[auto_func_name]
    pub fn new(date: String) -> Result<Self, MoreError> {
        let ndate = bjtc_sd(&date).m(m!(__func__))?;
        Ok(Self { ndate, sdate: date })
    }

    pub fn new_n(date: NaiveDate) -> Self {
        let sdate = bjtc_ds(&date);
        Self { ndate: date, sdate }
    }

    #[inline]
    pub fn s(&self) -> &String {
        &self.sdate
    }

    #[allow(dead_code)]
    #[inline]
    pub fn set_n(&mut self, date: NaiveDate) {
        self.sdate = bjtc_ds(&date);
        self.ndate = date;
    }

    #[auto_func_name]
    #[inline]
    pub fn set_s(&mut self, date: String) -> Result<(), MoreError> {
        self.ndate = bjtc_sd(&date).m(m!(__func__))?;
        self.sdate = date;

        Ok(())
    }
}

impl fmt::Debug for SqlDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.sdate.fmt(f)
    }
}

impl Default for SqlDate {
    fn default() -> Self {
        Self {
            sdate: "2000-01-01".to_string(),
            ndate: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(), // (2000,1,1) 不会返回 None
        }
    }
}

impl From<SqlDate> for Value {
    fn from(x: SqlDate) -> Value {
        Value::from(x.s())
    }
}

impl FromValue for SqlDate {
    type Intermediate = SqlDateParser;
}

pub struct SqlDateParser {
    value: Value,
    output: SqlDate,
}

impl ConvIr<SqlDate> for SqlDateParser {
    fn new(value: Value) -> Result<Self, FromValueError> {
        let (sdate, ndate) = match value {
            Value::Date(y, m, d, _, _, _, _) => match NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32) {
                Some(ndate) => {
                    let sdate = bjtc_ds(&ndate);
                    (sdate, ndate)
                }
                None => return Err(FromValueError(value)),
            },
            _ => {
                let sdate = String::from_value_opt(value.clone())?;
                let ndate = match bjtc_sd(&sdate) {
                    Ok(date) => date,
                    Err(_) => return Err(FromValueError(value)),
                };
                (sdate, ndate)
            }
        };

        Ok(Self {
            value,
            output: SqlDate { sdate, ndate },
        })
    }

    fn commit(self) -> SqlDate {
        self.output
    }
    fn rollback(self) -> Value {
        self.value
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct SqlTime {
    /// 时间
    stime: String,

    /// 日期
    ntime: NaiveDateTime,
}

impl SqlTime {
    #[allow(dead_code)]
    #[inline]
    pub fn n(&self) -> &NaiveDateTime {
        &self.ntime
    }

    #[auto_func_name]
    pub fn new(time: String) -> Result<Self, MoreError> {
        let ntime = bjtc_st(&time).m(m!(__func__))?;
        Ok(Self { ntime, stime: time })
    }

    pub fn new_n(time: NaiveDateTime) -> Self {
        let stime = bjtc_ts(&time);
        Self { ntime: time, stime }
    }

    #[inline]
    pub fn s(&self) -> &String {
        &self.stime
    }

    #[allow(dead_code)]
    #[inline]
    pub fn set_n(&mut self, time: NaiveDateTime) {
        self.stime = bjtc_ts(&time);
        self.ntime = time;
    }

    #[auto_func_name]
    #[inline]
    pub fn set_s(&mut self, time: String) -> Result<(), MoreError> {
        self.ntime = bjtc_st(&time).m(m!(__func__))?;
        self.stime = time;

        Ok(())
    }
}

impl fmt::Debug for SqlTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.stime.fmt(f)
    }
}

impl Default for SqlTime {
    fn default() -> Self {
        Self {
            stime: "2000-01-01".to_string(),
            ntime: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(), // (2000,1,1) 不会返回 None
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),    // (0,0,0) 不会返回 None
            ),
        }
    }
}

impl From<SqlTime> for Value {
    fn from(x: SqlTime) -> Value {
        Value::from(x.s())
    }
}

impl FromValue for SqlTime {
    type Intermediate = SqlTimeParser;
}

pub struct SqlTimeParser {
    value: Value,
    output: SqlTime,
}

impl ConvIr<SqlTime> for SqlTimeParser {
    fn new(value: Value) -> Result<Self, FromValueError> {
        let (stime, ntime) = match value {
            Value::Date(y, mo, d, h, mi, s, _) => match NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32) {
                Some(ndate) => match NaiveTime::from_hms_opt(h as u32, mi as u32, s as u32) {
                    Some(ntime) => {
                        let ntime = NaiveDateTime::new(ndate, ntime);
                        let stime = bjtc_ts(&ntime);
                        (stime, ntime)
                    }
                    None => return Err(FromValueError(value)),
                },
                None => return Err(FromValueError(value)),
            },
            _ => {
                let stime = String::from_value_opt(value.clone())?;
                let ntime = match bjtc_st(&stime) {
                    Ok(time) => time,
                    Err(_) => return Err(FromValueError(value)),
                };
                (stime, ntime)
            }
        };

        Ok(Self {
            value,
            output: SqlTime { stime, ntime },
        })
    }

    fn commit(self) -> SqlTime {
        self.output
    }
    fn rollback(self) -> Value {
        self.value
    }
}
