use {
    crate::{datetime::*, use_m::*},
    chrono::NaiveDate,
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
            ndate: NaiveDate::from_ymd(2000, 1, 1),
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
            Value::Date(y, m, d, _, _, _, _) => {
                let ndate = NaiveDate::from_ymd(y as i32, m as u32, d as u32);
                let sdate = bjtc_ds(&ndate);
                (sdate, ndate)
            }
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
