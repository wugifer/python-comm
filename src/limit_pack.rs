use std::collections::{HashMap, HashSet, VecDeque};

pub struct Limit {
    array_limit: usize,
    dict_limit: usize,
    str_limit: usize,
    pair_seq: u32,
    pair_stack: Vec<u32>,
}

impl Limit {
    /// 构造
    pub fn clone(&self, str_limit: usize) -> Self {
        Self {
            array_limit: self.array_limit,
            dict_limit: self.dict_limit,
            str_limit,
            pair_seq: self.pair_seq,
            pair_stack: Vec::new(),
        }
    }

    /// 构造
    pub fn new(array_limit: usize, dict_limit: usize, str_limit: usize) -> Self {
        Self {
            array_limit,
            dict_limit,
            str_limit,
            pair_seq: 0,
            pair_stack: Vec::new(),
        }
    }

    /// 构造 dict 类型
    pub fn new_dict<T1, T2>(&mut self, data: &Vec<(T1, T2)>) -> String
    where
        T1: LimitPackAble,
        T2: LimitPackAble,
    {
        // 左标识
        let pair_seq = self.pair_seq;
        let mut text = format!("{}{} ", '{', pair_seq);
        self.pair_seq += 1;

        let skip = if data.len() <= self.dict_limit {
            0
        } else {
            data.len() - self.dict_limit / 2 * 2
        };
        for (i, (k, v)) in data.iter().enumerate() {
            if skip == 0 || i < self.dict_limit / 2 || i >= self.dict_limit / 2 + skip {
                // 前半部 or 后半部
                let k_text = k.to_limit_str(self);
                let v_text = v.to_limit_str(self);
                text += &format!("{}:{}{}", k_text, v_text, if i < data.len() - 1 { "," } else { "" });
            } else if i == self.dict_limit / 2 {
                // 第一个 skip
                text += &format!("...{}...", skip);
            } else {
                // 其它 skip
            }
        }

        // 右标识
        text += &format!("{}{}{}", if data.len() > 0 { " " } else { "" }, pair_seq, '}');

        text
    }

    /// 构造 list 类型
    pub fn new_list<T>(&mut self, data: &Vec<T>) -> String
    where
        T: LimitPackAble,
    {
        // 左标识
        let pair_seq = self.pair_seq;
        let mut text = format!("{}{} ", '[', pair_seq);
        self.pair_seq += 1;

        let skip = if data.len() <= self.array_limit {
            0
        } else {
            data.len() - self.array_limit / 2 * 2
        };

        for (i, v) in data.iter().enumerate() {
            if skip == 0 || i < self.array_limit / 2 || i >= self.array_limit / 2 + skip {
                // 前半部 or 后半部
                let v_text = v.to_limit_str(self);
                text += &format!("{}{}", v_text, if i < data.len() - 1 { "," } else { "" });
            } else if i == self.array_limit / 2 {
                // 第一个 skip
                text += &format!("...{}...", skip);
            } else {
                // 其它 skip
            }
        }

        // 右标识
        text += &format!("{}{}{}", if data.len() > 0 { " " } else { "" }, pair_seq, ']');
        text
    }

    /// 构造 string 类型
    pub fn new_string(&self, text: String) -> String {
        let len = text.len();

        if self.str_limit <= 10 || len <= self.str_limit {
            // 完整保留
            text
        } else {
            // {左 half}...{skip}...{右 half}
            let full: Vec<char> = text.chars().collect();
            let half = self.str_limit / 2;

            let mut l: Vec<_> = full
                .iter()
                .enumerate()
                .map_while(|(i, ch)| if i < half { Some(*ch) } else { None })
                .collect();
            let mut m: Vec<_> = format!("...{}...", len - self.str_limit).chars().collect();
            let mut r: Vec<_> = full
                .iter()
                .rev()
                .enumerate()
                .map_while(|(i, ch)| if i < half { Some(*ch) } else { None })
                .collect();
            r.reverse();
            l.append(&mut m);
            l.append(&mut r);
            l.iter().map(|ch| ch.to_string()).collect::<Vec<_>>().join("")
        }
    }

    /// 构造 tuple 类型
    pub fn new_tuple(&mut self, data: &Vec<String>) -> String {
        // 左标识
        let pair_seq = self.pair_seq;
        let mut text = format!("{}{} ", '(', pair_seq);
        self.pair_seq += 1;

        for (i, v) in data.iter().enumerate() {
            text += &format!("{}{}", v, if i < data.len() - 1 { "," } else { "" });
        }

        // 右标识
        text += &format!("{}{}{}", if data.len() > 0 { " " } else { "" }, pair_seq, ')');
        text
    }

    /// 恢复 pop_start() 前的 pair_seq
    pub fn pop_end(&mut self, pair_seq: u32) {
        self.pair_seq = pair_seq;
    }

    /// 恢复 push 保存的 pair_seq, 返回当前 pair_seq, pop_end() 使用
    pub fn pop_start(&mut self) -> u32 {
        // 备份
        let pair_seq = self.pair_seq;

        // 恢复
        self.pair_seq = self.pair_stack.pop().unwrap_or(0);

        // 再备份
        pair_seq
    }

    /// 保存当前 pair_seq 备用, 然后 inc 给内部结构用
    pub fn push_and_inc(&mut self) {
        self.pair_stack.push(self.pair_seq);
        self.pair_seq += 1;
    }
}

pub struct ForStruct<T> {
    pub k: String,
    pub v: T,
}

pub trait LimitPackAble {
    /// 各类型转化为压缩后的字符串
    fn to_limit_str(&self, limit: &mut Limit) -> String;

    /// 各类型转化为压缩后的字符串
    fn to_limit_str3(&self, array_limit: usize, dict_limit: usize, str_limit: usize) -> String {
        let mut limit = Limit::new(array_limit, dict_limit, str_limit);
        self.to_limit_str(&mut limit)
    }
}

macro_rules! default_limit_pack {
    ($type:ident, $fix:expr) => {
        impl LimitPackAble for $type {
            fn to_limit_str(&self, limit: &mut Limit) -> String {
                limit
                    .clone(if $fix { 0 } else { limit.str_limit })
                    .new_string(format!("{}", self))
            }
        }
    };
}

// 参考 https://doc.rust-lang.org/std/fmt/trait.Debug.html

default_limit_pack!(bool, true);
default_limit_pack!(char, true);
default_limit_pack!(f32, true);
default_limit_pack!(f64, true);
default_limit_pack!(i8, true);
default_limit_pack!(i16, true);
default_limit_pack!(i32, true);
default_limit_pack!(i64, true);
default_limit_pack!(i128, true);
default_limit_pack!(isize, true);
default_limit_pack!(str, false);
default_limit_pack!(u8, true);
default_limit_pack!(u16, true);
default_limit_pack!(u32, true);
default_limit_pack!(u64, true);
default_limit_pack!(u128, true);
default_limit_pack!(usize, true);
default_limit_pack!(String, false);

impl<T> LimitPackAble for &T
where
    T: LimitPackAble + ?Sized,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        (**self).to_limit_str(limit)
    }
}

impl<T> LimitPackAble for &mut T
where
    T: LimitPackAble + ?Sized,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        (**self).to_limit_str(limit)
    }
}

impl<T> LimitPackAble for Option<T>
where
    T: LimitPackAble,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        match self {
            Some(obj) => obj.to_limit_str(limit),
            None => Limit::new(0, 0, 0).new_string("None".to_string()),
        }
    }
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<$($name:LimitPackAble),+> LimitPackAble for ($($name,)+) where last_type!($($name,)+): ?Sized {
            #[allow(non_snake_case, unused_assignments)]
            fn to_limit_str(&self, limit: &mut Limit) -> String {
                let ($(ref $name,)+) = *self;

                limit.push_and_inc();
                let v = vec![
                    $(
                        $name.to_limit_str(limit),
                    )+
                ];
                let pair_seq = limit.pop_start();
                let text = limit.new_tuple(&v);
                limit.pop_end(pair_seq);
                text
            }
        }
        peel! { $($name,)+ }
    )
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple! { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, }

impl<T> LimitPackAble for [T]
where
    T: LimitPackAble,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        limit.new_list(&self.iter().collect())
    }
}

impl<T, const N: usize> LimitPackAble for [T; N]
where
    T: LimitPackAble,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        limit.new_list(&self.iter().collect())
    }
}

impl<T> LimitPackAble for Vec<T>
where
    T: LimitPackAble,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        limit.new_list(self)
    }
}

impl<T> LimitPackAble for VecDeque<T>
where
    T: LimitPackAble,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        limit.new_list(&self.iter().collect())
    }
}

impl<T, S> LimitPackAble for HashSet<T, S>
where
    T: LimitPackAble,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        limit.new_list(&self.iter().collect())
    }
}

impl<K, V, S> LimitPackAble for HashMap<K, V, S>
where
    K: LimitPackAble,
    V: LimitPackAble,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        limit.new_dict(&self.iter().collect())
    }
}

impl<T> LimitPackAble for ForStruct<T>
where
    T: LimitPackAble,
{
    fn to_limit_str(&self, limit: &mut Limit) -> String {
        format!("{}:{}", self.k, self.v.to_limit_str(limit))
    }
}
