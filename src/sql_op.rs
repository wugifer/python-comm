use {
    crate::use_m::*,
    mysql::{
        params::Params,
        prelude::{FromRow, Queryable},
        OptsBuilder, Pool, PooledConn,
    },
    std::sync::MutexGuard,
};

/// 负责通过 lazy_static 创建 DbPool 的类
pub trait CreateDbPool {
    /// 返回加锁的 DbPool, 注意: args 无效时应返回 Error
    fn lock() -> Result<MutexGuard<'static, DbPool>, MoreError>;
}

#[cfg(feature = "use_sql")]
/// 全局数据库连接池
pub struct DbPool {
    pool: Option<Pool>,
    args: &'static DbPoolArgs,
}

#[cfg(feature = "use_sql")]
impl DbPool {
    #[auto_func_name]
    /// 创建或返回当前连接池
    fn _create(&mut self) -> Result<Pool, MoreError> {
        match &self.pool {
            Some(pool) => Ok(pool.clone()),
            None => {
                let opts = OptsBuilder::new()
                    .ip_or_hostname(Some(&self.args.ip_or_hostname))
                    .tcp_port(self.args.port)
                    .user(Some(&self.args.user))
                    .pass(Some(&self.args.password))
                    .db_name(Some(&self.args.db_name));
                let pool = Pool::new_manual(3, 5, opts).m(m!(__func__))?;

                self.pool = Some(pool.clone());
                Ok(pool)
            }
        }
    }

    #[auto_func_name]
    /// 获取可用连接
    fn _get(&mut self) -> Result<PooledConn, MoreError> {
        self._create().m(m!(__func__))?.get_conn().m(m!(__func__))
    }

    #[auto_func_name]
    /// 执行 sql, 返回 id
    fn get_id(&mut self, sql: &str, params: Params) -> Result<Option<u64>, MoreError>
    where
        Self: 'static,
    {
        Ok(self
            ._get()
            .m(m!(__func__))?
            .exec_iter(sql, params)
            .m(m!(__func__, sql))?
            .last_insert_id())
    }

    #[auto_func_name]
    /// 执行 sql, 不关心结果
    fn get_nothing(&mut self, sql: &str, params: Params) -> Result<(), MoreError>
    where
        Self: 'static,
    {
        self._get()
            .m(m!(__func__))?
            .exec_drop(sql, &params)
            .f(m!(__func__, || { format!("{}: {:?}", sql, &params) }))?;

        Ok(())
    }

    pub fn new(args: &'static DbPoolArgs) -> Self {
        Self { pool: None, args }
    }
}

#[derive(Default)]
/// GloabalDbPool 参数
pub struct DbPoolArgs {
    pub ip_or_hostname: String, // 地址
    pub port: u16,              // 端口
    pub user: String,           // 用户
    pub password: String,       // 密码
    pub db_name: String,        // 数据库
}

pub trait SqlModel {
    //
    // 从这里开始是需要 trait 实现的, 在 AsSqlModel 宏实现
    //

    /// 比较两个 obj
    fn equal(&self, other: &Self) -> bool;

    /// 比较两个 obj, 排除 id
    fn equal_without_id(&self, other: &Self) -> bool;

    /// 返回加锁的 DbPool, 注意类名写死了, 使用者需命名并引入 WhoCreateDbPool
    fn lock() -> Result<MutexGuard<'static, DbPool>, MoreError>;

    fn make_create_table() -> &'static str;

    // C-有逗号结尾, Q-有双引号, B-有反引号, I-去掉 id, P-作为参数, E-赋值, V-Value, EE-相等

    /// `a`, `b`, `c`
    fn make_fields_b() -> &'static str;
    /// `a`, `b`, `c`
    fn make_fields_bi() -> &'static str;
    /// a=:a, b=:b
    fn make_fields_e() -> &'static str;
    /// a=:a, b=:b
    fn make_fields_ei() -> &'static str;
    /// :a, :b
    fn make_fields_p() -> &'static str;
    /// :a, :b
    fn make_fields_pi() -> &'static str;
    /// "a", "b", "c"
    fn make_fields_q() -> &'static str;
    /// "a", "b", "c",
    fn make_fields_qc() -> &'static str;
    /// vec![("a", self.a), ("b", self.b)]
    fn make_fields_v(&self) -> Params;
    /// vec![("a", self.a), ("b", self.b)]
    fn make_fields_vi(&self) -> Params;

    /// 数据库表名
    fn table_name() -> &'static str;

    //
    // 从这里开始是 trait 对外提供的
    //

    #[auto_func_name]
    /// 增
    fn create(&self) -> Result<Option<u64>, MoreError> {
        Self::lock().m(m!(__func__))?.get_id(
            &format!(
                "INSERT INTO {} ({}) VALUES ({})",
                Self::table_name(),
                Self::make_fields_bi(),
                Self::make_fields_pi(),
            ),
            self.make_fields_vi(),
        )
    }

    #[auto_func_name]
    /// 删
    fn delete(condition: &str, params: Params) -> Result<(), MoreError> {
        Self::lock().m(m!(__func__))?.get_nothing(
            &format!("DELETE FROM {} WHERE {}", Self::table_name(), condition),
            params,
        )
    }

    #[auto_func_name]
    /// 获取可能的单个记录, 含带参条件
    fn get_row<T>(sql: &str, params: Params) -> Result<Option<T>, MoreError>
    where
        Self: 'static,
        T: FromRow,
    {
        match Self::lock()
            .m(m!(__func__))?
            ._get()
            .m(m!(__func__))?
            .exec_first_opt(sql, &params)
            .f(m!(__func__, || { format!("{}: {:?}", sql, &params) }))?
        {
            Some(Ok(row)) => Ok(Some(row)),
            Some(Err(err)) => Err(err).m(m!(__func__)),
            None => Ok(None),
        }
    }

    #[auto_func_name]
    /// 获取多个记录, 含带参条件
    fn get_rows<T>(sql: &str, params: Params) -> Result<Vec<T>, MoreError>
    where
        Self: 'static,
        T: FromRow,
    {
        // 全部结果
        let rows = Self::lock()
            .m(m!(__func__))?
            ._get()
            .m(m!(__func__))?
            .exec_opt(sql, &params)
            .f(m!(__func__, || { format!("{}: {:?}", sql, &params) }))?;

        // 如果有 FromRowError, 抛出异常, 这样后续可以 unwrap (map 中不可抛出异常)
        for (i, row) in rows.iter().enumerate() {
            if let Err(err) = row {
                return Err(err).f(m!(__func__, || { format!("{}: {:?}, row[{}]", sql, &params, i) }));
            }
        }

        // 已确认 x 不含异常, 收集
        Ok(rows.into_iter().map(|x| x.unwrap()).collect())
    }

    #[auto_func_name]
    fn save_as(&self, id: u32) -> Result<(), MoreError> {
        Self::lock()
            .m(m!(__func__))?
            .get_nothing(
                &format!(
                    "UPDATE {} SET {} WHERE id={}",
                    Self::table_name(),
                    Self::make_fields_ei(),
                    id
                ),
                self.make_fields_vi(),
            )
            .m(m!(__func__))
    }

    #[auto_func_name]
    /// 获取可能的单个记录, 含带参条件
    fn select_one(where_sql: &str, params: Params) -> Result<Option<Self>, MoreError>
    where
        Self: 'static + Sized + FromRow,
    {
        match Self::lock()
            .m(m!(__func__))?
            ._get()
            .m(m!(__func__))?
            .exec_first_opt(
                &format!(
                    "SELECT {} FROM {} {}",
                    Self::make_fields_b(),
                    Self::table_name(),
                    where_sql
                ),
                &params,
            )
            .f(m!(__func__, || { format!("{}: {:?}", where_sql, &params) }))?
        {
            Some(Ok(row)) => Ok(Some(row)),
            Some(Err(err)) => Err(err).m(m!(__func__)),
            None => Ok(None),
        }
    }

    #[auto_func_name]
    /// 获取多个记录, 含带参条件
    fn select_some(where_sql: &str, params: Params) -> Result<Vec<Self>, MoreError>
    where
        Self: 'static + Sized + FromRow,
    {
        // 全部结果
        let rows = Self::lock()
            .m(m!(__func__))?
            ._get()
            .m(m!(__func__))?
            .exec_first_opt(
                &format!(
                    "SELECT {} FROM {} {}",
                    Self::make_fields_b(),
                    Self::table_name(),
                    where_sql
                ),
                &params,
            )
            .f(m!(__func__, || { format!("{}: {:?}", where_sql, &params) }))?;

        // 如果有 FromRowError, 抛出异常, 这样后续可以 unwrap (map 中不可抛出异常)
        for (i, row) in rows.iter().enumerate() {
            if let Err(err) = row {
                return Err(err).f(m!(__func__, || { format!("{}: {:?}, row[{}]", where_sql, &params, i) }));
            }
        }

        // 已确认 x 不含异常, 收集
        Ok(rows.into_iter().map(|x| x.unwrap()).collect())
    }

    #[auto_func_name]
    /// 改
    fn update(fields_ei: &str, condition: &str, params: Params) -> Result<(), MoreError> {
        Self::lock()
            .m(m!(__func__))?
            .get_nothing(
                &format!("UPDATE {} SET {} WHERE {}", Self::table_name(), fields_ei, condition),
                params,
            )
            .m(m!(__func__))
    }
}
