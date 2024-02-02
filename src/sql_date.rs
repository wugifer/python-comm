use {
    crate::{datetime::*, use_m::*},
    chrono::{DateTime, FixedOffset, NaiveDate, TimeZone},
    mysql::{
        prelude::{ConvIr, FromValue},
        FromValueError, Value,
    },
    serde::{Deserialize, Deserializer, Serialize, Serializer},
    std::fmt,
};

// mysql
// show VARIABLES like '%time_zone%'
//
// 存储时，MySQL将 TIMESTAMP 值从当前时区转换为 UTC 时间进行存储，查询时，将数据从 UTC 转换为检索的当前时区。（其他类型（如DATETIME）不会发生这种情况。）
// 默认情况下，每个连接的当前时区是服务器的时间。时区可以根据每个连接进行设置。只要时区设置保持不变，就可以得到存储的相同值。
//
// Date、Time、DateTime 类型不支持时区转换。DateTime 的格式为 YYYY-MM-DD HH:MM:SS[.fraction] 其中小数部分字段定义时需要更多存储
// 范围为 1000-01-01 00:00:00 至 9999-12-31 23:59:59
// DateTime 不支持 .000Z, +08:00 等包含时区的字符串

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
    fn commit(self) -> SqlDate {
        self.output
    }
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

    fn rollback(self) -> Value {
        self.value
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct SqlTime {
    /// 时间, mysql 仅支持 YYYY-MM-DD HH:MM:SS 格式, 不带时区, 统一按照北京时间写入
    stime: String,

    /// 日期
    ntime: DateTime<FixedOffset>,
}

impl SqlTime {
    #[allow(dead_code)]
    #[inline]
    pub fn n(&self) -> &DateTime<FixedOffset> {
        &self.ntime
    }

    #[auto_func_name]
    pub fn new(time: String) -> Result<Self, MoreError> {
        let ntime = bjtc_bt(&time).m(m!(__func__))?;
        Ok(Self { ntime, stime: time })
    }

    pub fn new_n(time: DateTime<FixedOffset>) -> Self {
        let stime = bjtc_tb(&time);
        Self { ntime: time, stime }
    }

    #[inline]
    pub fn s(&self) -> &String {
        &self.stime
    }

    #[allow(dead_code)]
    #[inline]
    pub fn set_n(&mut self, time: DateTime<FixedOffset>) {
        self.stime = bjtc_tb(&time);
        self.ntime = time;
    }

    #[auto_func_name]
    #[inline]
    pub fn set_s(&mut self, time: String) -> Result<(), MoreError> {
        self.ntime = bjtc_bt(&time).m(m!(__func__))?;
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
            stime: "2000-01-01T00:00:00".to_string(),
            ntime: FixedOffset::east_opt(8 * 3600)
                .unwrap()
                .with_ymd_and_hms(2000, 1, 1, 0, 0, 0)
                .unwrap(),
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

impl Serialize for SqlTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.stime)
    }
}

impl<'de> Deserialize<'de> for SqlTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        SqlTime::new(s).map_err(serde::de::Error::custom)
    }
}

pub struct SqlTimeParser {
    value: Value,
    output: SqlTime,
}

impl ConvIr<SqlTime> for SqlTimeParser {
    fn commit(self) -> SqlTime {
        self.output
    }
    fn new(value: Value) -> Result<Self, FromValueError> {
        let (stime, ntime) = match value {
            // Date 是哪个时区的? 下面代码假定是北京时间
            Value::Date(y, mo, d, h, mi, s, _) => match FixedOffset::east_opt(8 * 3600)
                .unwrap()
                .with_ymd_and_hms(y as i32, mo as u32, d as u32, h as u32, mi as u32, s as u32)
                .single()
            {
                Some(ntime) => {
                    let stime = bjtc_tb(&ntime);
                    (stime, ntime)
                }
                None => return Err(FromValueError(value)),
            },
            _ => {
                let stime = String::from_value_opt(value.clone())?;
                let ntime = match bjtc_bt(&stime) {
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

    fn rollback(self) -> Value {
        self.value
    }
}
