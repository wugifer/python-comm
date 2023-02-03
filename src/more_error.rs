use std::{error::Error, fmt};

/// 包含更多信息的 Error
pub struct MoreError {
    simple_text: String,
    detail_text: String,
}

impl MoreError {
    /// 详细内容
    pub fn detail(&self) -> &String {
        &self.detail_text
    }

    /// 从 Error 构造
    fn from_error<E>(err: &E, file: &str, line: u32, func: &str, text: &str) -> Self
    where
        E: Error,
    {
        Self {
            simple_text: format!("Error: {} {}()\nError: {}", file, func, err),
            detail_text: format!("Error: {}:{:3} {}() {}\nError: {:?}", file, line, func, text, err),
        }
    }

    /// 从 MoreError 构造
    fn from_more(err: &Self, file: &str, line: u32, func: &str, text: &str) -> Self {
        Self {
            simple_text: format!("Error: {} {}()\n{}", file, func, err.simple()),
            detail_text: format!("Error: {}:{:3} {}() {}\n{}", file, line, func, text, err.detail()),
        }
    }

    /// 从零构造
    pub fn new(file: &str, line: u32, func: &str, text: &str) -> Self {
        Self {
            simple_text: format!("Error: {} {}()", file, func),
            detail_text: format!("Error: {}:{:3} {}() {}", file, line, func, text),
        }
    }

    /// 简单内容
    pub fn simple(&self) -> &String {
        &self.simple_text
    }
}

impl fmt::Debug for MoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.simple().fmt(f)
    }
}

impl fmt::Display for MoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.detail().fmt(f)
    }
}

/// 给 Error 增加更多信息
pub trait AddMore<T> {
    /// 附加文件名、行号、函数名、附加说明
    fn f<F>(self, file_line_func_func: (&str, u32, &str, F)) -> Result<T, MoreError>
    where
        F: Fn() -> String;

    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError>;

    /// 附加文件名、行号、函数名、附加说明
    fn p(&self, file_line_func_text: (&str, u32, &str, &str));
}

impl<T, E> AddMore<T> for Result<T, E>
where
    E: Error,
{
    /// 附加文件名、行号、函数名、附加说明
    fn f<F>(self, file_line_func_func: (&str, u32, &str, F)) -> Result<T, MoreError>
    where
        F: Fn() -> String,
    {
        self.or_else(|err| {
            Err(MoreError::from_error(
                &err,
                file_line_func_func.0,
                file_line_func_func.1,
                file_line_func_func.2,
                &file_line_func_func.3(),
            ))
        })
    }

    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError> {
        self.or_else(|err| {
            Err(MoreError::from_error(
                &err,
                file_line_func_text.0,
                file_line_func_text.1,
                file_line_func_text.2,
                file_line_func_text.3,
            ))
        })
    }

    /// 附加文件名、行号、函数名、附加说明
    fn p(&self, file_line_func_text: (&str, u32, &str, &str)) {
        if let Err(err) = self {
            println!(
                "{}",
                MoreError::from_error(
                    err,
                    file_line_func_text.0,
                    file_line_func_text.1,
                    file_line_func_text.2,
                    file_line_func_text.3,
                )
                .detail()
            );
        }
    }
}

impl<T, E> AddMore<T> for &E
where
    E: Error,
{
    /// 附加文件名、行号、函数名、附加说明
    fn f<F>(self, file_line_func_func: (&str, u32, &str, F)) -> Result<T, MoreError>
    where
        F: Fn() -> String,
    {
        Err(MoreError::from_error(
            self,
            file_line_func_func.0,
            file_line_func_func.1,
            file_line_func_func.2,
            &file_line_func_func.3(),
        ))
    }

    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError> {
        Err(MoreError::from_error(
            self,
            file_line_func_text.0,
            file_line_func_text.1,
            file_line_func_text.2,
            file_line_func_text.3,
        ))
    }

    /// 附加文件名、行号、函数名、附加说明
    fn p(&self, file_line_func_text: (&str, u32, &str, &str)) {
        println!(
            "{}",
            MoreError::from_error(
                self,
                file_line_func_text.0,
                file_line_func_text.1,
                file_line_func_text.2,
                file_line_func_text.3,
            )
            .detail()
        );
    }
}

impl<T> AddMore<T> for Result<T, MoreError> {
    /// 附加文件名、行号、函数名、附加说明
    fn f<F>(self, file_line_func_func: (&str, u32, &str, F)) -> Result<T, MoreError>
    where
        F: Fn() -> String,
    {
        self.or_else(|err| {
            Err(MoreError::from_more(
                &err,
                file_line_func_func.0,
                file_line_func_func.1,
                file_line_func_func.2,
                &file_line_func_func.3(),
            ))
        })
    }

    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError> {
        self.or_else(|err| {
            Err(MoreError::from_more(
                &err,
                file_line_func_text.0,
                file_line_func_text.1,
                file_line_func_text.2,
                file_line_func_text.3,
            ))
        })
    }

    /// 附加文件名、行号、函数名、附加说明
    fn p(&self, file_line_func_text: (&str, u32, &str, &str)) {
        if let Err(err) = self {
            println!(
                "{}",
                MoreError::from_more(
                    err,
                    file_line_func_text.0,
                    file_line_func_text.1,
                    file_line_func_text.2,
                    file_line_func_text.3,
                )
                .detail()
            );
        }
    }
}

impl<T> AddMore<T> for &MoreError {
    /// 附加文件名、行号、函数名、附加说明
    fn f<F>(self, file_line_func_func: (&str, u32, &str, F)) -> Result<T, MoreError>
    where
        F: Fn() -> String,
    {
        Err(MoreError::from_more(
            self,
            file_line_func_func.0,
            file_line_func_func.1,
            file_line_func_func.2,
            &file_line_func_func.3(),
        ))
    }

    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError> {
        Err(MoreError::from_more(
            self,
            file_line_func_text.0,
            file_line_func_text.1,
            file_line_func_text.2,
            file_line_func_text.3,
        ))
    }

    /// 附加文件名、行号、函数名、附加说明
    fn p(&self, file_line_func_text: (&str, u32, &str, &str)) {
        println!(
            "{}",
            MoreError::from_more(
                self,
                file_line_func_text.0,
                file_line_func_text.1,
                file_line_func_text.2,
                file_line_func_text.3,
            )
            .detail()
        );
    }
}
