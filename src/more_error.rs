use std::{error::Error, fmt};

/// 包含更多信息的 Error: 每次调用的文件名、行号、函数名、可选的附加内容
pub struct MoreError {
    text: String,
}

impl MoreError {
    /// 从 Error 构造
    fn from_error<E>(err: &E, file: &str, line: u32, func: &str, text: &str) -> Self
    where
        E: fmt::Debug,
    {
        Self {
            text: format!("Error: {}:{:3} {}() {}\nError: {:?}", file, line, func, text, err),
        }
    }

    /// 从 MoreError 构造
    fn from_more(err: &Self, file: &str, line: u32, func: &str, text: &str) -> Self {
        Self {
            text: format!("Error: {}:{:3} {}() {}\n{}", file, line, func, text, err.text),
        }
    }

    /// 从零构造
    pub fn new(file: &str, line: u32, func: &str, text: &str) -> Self {
        Self {
            text: format!("Error: {}:{:3} {}() {}", file, line, func, text),
        }
    }
}

impl fmt::Debug for MoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text.fmt(f)
    }
}

impl fmt::Display for MoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text.fmt(f)
    }
}

/// 给 Error, ... 增加更多信息
pub trait AddMoreError<T> {
    /// 附加文件名、行号、函数名、附加说明, 生成 MoreError, 与 m() 类似, 但附加内容由闭包产生
    fn f<F>(self, file_line_func_func: (&str, u32, &str, F)) -> Result<T, MoreError>
    where
        F: Fn() -> String;

    /// 附加文件名、行号、函数名、附加说明, 生成 MoreError
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError>;

    /// 附加文件名、行号、函数名、附加说明, 输出
    fn p(&self, file_line_func_text: (&str, u32, &str, &str));
}

/// 给 Error, ... 增加更多信息, 但抛弃 Error, ... 自身的内容
pub trait AsMoreError<T> {
    /// 附加文件名、行号、函数名、附加说明, 抛弃 Error 自身的内容
    fn as_m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError>;

    /// 转化为 MoreError, 不附加信息
    fn to_m(self) -> Result<T, MoreError>;
}

/// 合并 Result<Result<T, E>, E> 为 Result<T, E>
pub trait LessError<T, E> {
    /// Ok(Result) -> Result, Err(*) -> Err(*)
    fn l(self) -> Result<T, E>;
}

impl<T, E> AddMoreError<T> for Result<T, E>
where
    E: Error,
{
    /// 附加文件名、行号、函数名、附加说明
    fn f<F>(self, file_line_func_func: (&str, u32, &str, F)) -> Result<T, MoreError>
    where
        F: Fn() -> String,
    {
        self.map_err(|err| {
            MoreError::from_error(
                &err,
                file_line_func_func.0,
                file_line_func_func.1,
                file_line_func_func.2,
                &file_line_func_func.3(),
            )
        })
    }

    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError> {
        self.map_err(|err| {
            MoreError::from_error(
                &err,
                file_line_func_text.0,
                file_line_func_text.1,
                file_line_func_text.2,
                file_line_func_text.3,
            )
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
            );
        }
    }
}

impl<T, E> AddMoreError<T> for &E
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
        );
    }
}

impl<T> AddMoreError<T> for Result<T, MoreError> {
    /// 附加文件名、行号、函数名、附加说明
    fn f<F>(self, file_line_func_func: (&str, u32, &str, F)) -> Result<T, MoreError>
    where
        F: Fn() -> String,
    {
        self.map_err(|err| {
            MoreError::from_more(
                &err,
                file_line_func_func.0,
                file_line_func_func.1,
                file_line_func_func.2,
                &file_line_func_func.3(),
            )
        })
    }

    /// 附加文件名、行号、函数名、附加说明
    fn m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError> {
        self.map_err(|err| {
            MoreError::from_more(
                &err,
                file_line_func_text.0,
                file_line_func_text.1,
                file_line_func_text.2,
                file_line_func_text.3,
            )
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
            );
        }
    }
}

impl<T> AddMoreError<T> for &MoreError {
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
        );
    }
}

impl<T, E> AsMoreError<T> for Result<T, E>
where
    E: fmt::Debug,
{
    /// 附加文件名、行号、函数名、附加说明, 抛弃 Error 自身的内容
    fn as_m(self, file_line_func_text: (&str, u32, &str, &str)) -> Result<T, MoreError> {
        self.map_err(|_| {
            MoreError::new(
                file_line_func_text.0,
                file_line_func_text.1,
                file_line_func_text.2,
                file_line_func_text.3,
            )
        })
    }

    /// 转化为 MoreError, 不附加信息
    fn to_m(self) -> Result<T, MoreError> {
        self.map_err(|err| MoreError {
            text: format!("{:?}", err),
        })
    }
}

impl<T, E> LessError<T, E> for Result<Result<T, E>, E> {
    /// Ok(Result) -> Result, Err(*) -> Err(*)
    fn l(self) -> Result<T, E> {
        self.and_then(|x| x)
    }
}
