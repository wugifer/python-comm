use std::{error::Error, fmt};

/// 包含更多信息的 Error
pub struct MoreError {
    err: String,
}

impl MoreError {
    /// 从 Error 构造
    fn from_error<E>(err: E, file: &str, line: u32, func: &str, text: &str) -> Self
    where
        E: Error,
    {
        Self {
            err: format!("Error: {}:{:3} {}() {}\nError: {:?}", file, line, func, text, err),
        }
    }

    /// 从 MoreError 构造
    fn from_more(err: Self, file: &str, line: u32, func: &str, text: &str) -> Self {
        Self {
            err: format!("Error: {}:{:3} {}() {}\n{}", file, line, func, text, err),
        }
    }

    /// 从零构造
    pub fn new(file: &str, line: u32, func: &str, text: &str) -> Self {
        Self {
            err: format!("Error: {}:{:3} {}() {}", file, line, func, text),
        }
    }
}

impl fmt::Display for MoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.err.fmt(f)
    }
}

/// 给 Error 增加更多信息
pub trait AddMore<T> {
    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError>;
}

impl<T, E> AddMore<T> for Result<T, E>
where
    E: Error,
{
    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError> {
        self.or_else(|err| {
            Err(MoreError::from_error(
                err,
                file_line_func_text.0,
                file_line_func_text.1,
                file_line_func_text.2,
                file_line_func_text.3,
            ))
        })
    }
}

impl<T> AddMore<T> for Result<T, MoreError> {
    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError> {
        self.or_else(|err| {
            Err(MoreError::from_more(
                err,
                file_line_func_text.0,
                file_line_func_text.1,
                file_line_func_text.2,
                file_line_func_text.3,
            ))
        })
    }
}
