use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use pyo3::{
    proc_macro::pyclass,
    types::{PyAny, PyDate, PyDateAccess, PyDateTime, PyTimeAccess, PyTuple},
    FromPyObject, IntoPy, PyErr, PyObject, Python, ToPyObject,
};
use python_comm_macros::auto_func_name;
use rust_decimal::{prelude::FromPrimitive, Decimal};

macro_rules! new_type {
    ( $name:ident, $inner_type:ty) => {
        pub struct $name(pub $inner_type);

        impl Into<$inner_type> for $name {
            fn into(self) -> $inner_type {
                self.0
            }
        }
    };
}

#[pyclass(dict)]
pub struct PyClassObject {}

new_type!(PyDecimal, Decimal);

impl FromPyObject<'_> for PyDecimal {
    #[auto_func_name]
    fn extract(obj: &PyAny) -> Result<Self, PyErr> {
        let a: (i64, i64) = obj
            .call_method("as_integer_ratio", (), None)
            .or_else(|err| raise_error!("py", __func__, "\n", err))?
            .extract()
            .or_else(|err| raise_error!("py", __func__, "\n", err))?;

        Ok(PyDecimal(
            Decimal::from_i128(a.0 as i128).ok_or(raise_error!(
                "raw",
                "py",
                __func__,
                format!(r#"Decimal::from_i128({}) error"#, a.0)
            ))? / Decimal::from_i128(a.1 as i128).ok_or(raise_error!(
                "raw",
                "py",
                __func__,
                format!(r#"Decimal::from_i128({}) error"#, a.1)
            ))?,
        ))
    }
}

impl IntoPy<PyObject> for PyDecimal {
    fn into_py(self, python: Python) -> PyObject {
        ToPyObject::to_object(&self, python)
    }
}

impl ToPyObject for PyDecimal {
    fn to_object(&self, python: pyo3::Python) -> PyObject {
        PyTuple::new(python, vec![self.0.mantissa(), self.0.scale() as i128]).to_object(python)
    }
}

new_type!(PyNaiveDate, NaiveDate);

impl FromPyObject<'_> for PyNaiveDate {
    #[auto_func_name]
    fn extract(obj: &PyAny) -> Result<Self, PyErr> {
        let pydate: &PyDate = obj
            .extract()
            .or_else(|err| raise_error!("py", __func__, "\n", err))?;
        Ok(PyNaiveDate(NaiveDate::from_ymd(
            pydate.get_year(),
            pydate.get_month() as u32,
            pydate.get_day() as u32,
        )))
    }
}

impl ToPyObject for PyNaiveDate {
    fn to_object(&self, python: Python) -> PyObject {
        match PyDate::new(
            python,
            self.0.year(),
            self.0.month() as u8,
            self.0.day() as u8,
        ) {
            Ok(date) => date.to_object(python),
            Err(_) => python.None(),
        }
    }
}

impl IntoPy<PyObject> for PyNaiveDate {
    fn into_py(self, python: Python) -> PyObject {
        ToPyObject::to_object(&self, python)
    }
}

new_type!(PyNaiveDateTime, NaiveDateTime);

impl FromPyObject<'_> for PyNaiveDateTime {
    #[auto_func_name]
    fn extract(obj: &PyAny) -> Result<Self, PyErr> {
        let pydatetime: &PyDateTime = obj
            .extract()
            .or_else(|err| raise_error!("py", __func__, "\n", err))?;
        Ok(PyNaiveDateTime(
            NaiveDate::from_ymd(
                pydatetime.get_year(),
                pydatetime.get_month() as u32,
                pydatetime.get_day() as u32,
            )
            .and_hms(
                pydatetime.get_hour() as u32,
                pydatetime.get_minute() as u32,
                pydatetime.get_second() as u32,
            ),
        ))
    }
}

impl ToPyObject for PyNaiveDateTime {
    fn to_object(&self, python: Python) -> pyo3::PyObject {
        match PyDateTime::new(
            python,
            self.0.year(),
            self.0.month() as u8,
            self.0.day() as u8,
            self.0.hour() as u8,
            self.0.minute() as u8,
            self.0.second() as u8,
            0,
            None,
        ) {
            Ok(datetime) => datetime.to_object(python),
            Err(_) => python.None(),
        }
    }
}

impl pyo3::IntoPy<pyo3::PyObject> for PyNaiveDateTime {
    fn into_py(self, python: Python) -> PyObject {
        ToPyObject::to_object(&self, python)
    }
}
