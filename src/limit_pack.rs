use std::collections::{HashMap, HashSet, VecDeque};

// str~         单纯字符串, 可裁剪长度
// fix          由整型等转换而成的字符串, 不可裁剪长度
// (obj~,)      元组, 不可裁剪数量
// [obj~^]      列表, 可裁剪数量
// {obj~:obj~^} 字典, 可裁剪数量
// {obj~^}      集合, 可裁剪数量
// ~  字符串被截断
// ^  列表/字典/集合被截断
// 裁剪过程
// 1. 当前
//     可裁剪的最长字符串有 n 个
//     可裁剪的列表/字典/集合有若干个, 其中裁剪效果最差的能减少 m 个字符
// 2. 如果 m < n, 裁剪掉列表/字典/集合的一个元素, 否则裁剪掉一个字符

pub struct LimitObj {
    uid: u32,                              // 裁剪过程中的唯一编号
    chars: Vec<char>,                      // 裁剪后的字符串, 不用 String, 后者 utf-8 切片困难
    v1: Option<Vec<LimitObj>>,             // 裁剪后的列表
    v2: Option<Vec<(LimitObj, LimitObj)>>, // 裁剪后的字典
    c: (char, char),                       // 界定符, () 或 [] 或 {}
    space: usize,                          // 表示为 String 应占据的空间
    trim: usize,                           // 当前状态可 trim 的空间, Vec 仅 trim 最后一项
    more: usize,                           // 1 = 已产生 trim 占位符(string 类型不参与)
}

impl LimitObj {
    /// 获取 trim 空间最大的 STRING 类型
    fn get_max_string_trim(&self, mut string_trim: usize, uids: &mut Vec<u32>) -> usize {
        // string 类型
        if self.chars.len() > 0 {
            if self.trim > string_trim {
                uids.clear();
            }
            if self.trim >= string_trim {
                uids.push(self.uid);
                return self.trim;
            }
            return string_trim;
        }

        // list 类型
        if let Some(v) = &self.v1 {
            for obj in v {
                string_trim = obj.get_max_string_trim(string_trim, uids);
            }
        }

        // dict 类型
        if let Some(v) = &self.v2 {
            for (k, v) in v {
                string_trim = k.get_max_string_trim(string_trim, uids);
                string_trim = v.get_max_string_trim(string_trim, uids);
            }
        }

        string_trim
    }

    /// 获取 trim 空间最小的 LIST/MAP/TUPLE 类型, list_trim = (trim, len, uid), trim/len 参与比较
    fn get_min_list_trim(&self, mut list_trim: (usize, usize, u32)) -> (usize, usize, u32) {
        // 递归获取
        if let Some(v) = &self.v1 {
            for obj in v {
                list_trim = obj.get_min_list_trim(list_trim);
            }
        }

        // 递归获取
        if let Some(v) = &self.v2 {
            for (k, v) in v {
                list_trim = k.get_min_list_trim(list_trim);
                list_trim = v.get_min_list_trim(list_trim);
            }
        }

        // 无法裁剪
        if self.trim == 0 {
            return list_trim;
        }

        let mut trim = (self.trim, 1);
        if let Some(v) = &self.v1 {
            trim = (trim.0, v.len() + 1)
        }
        if let Some(v) = &self.v2 {
            trim = (trim.0, v.len() + 1)
        }

        // 更优, 比较 trim/len
        if (self.v1.is_some() || self.v2.is_some())
            && self.trim > 0
            && (list_trim.0 == usize::MAX || trim.0 * list_trim.1 < list_trim.0 * trim.1)
        {
            return (trim.0, trim.1, self.uid);
        }

        // 不动
        list_trim
    }

    /// 获取最终文本
    fn get_text(&self) -> String {
        // string 类型
        if self.chars.len() > 0 {
            return self.chars.iter().map(|ch| ch.to_string()).collect::<Vec<_>>().join("");
        }

        // list 类型
        if let Some(v) = &self.v1 {
            return format!(
                "{}{}{}{}",
                self.c.0,
                v.iter().map(|x| x.get_text()).collect::<Vec<_>>().join(","),
                if self.more == 1 { "^" } else { "" },
                self.c.1
            );
        }

        // dict 类型
        if let Some(v) = &self.v2 {
            return format!(
                "{}{}{}{}",
                self.c.0,
                v.iter()
                    .map(|x| format!("{}:{}", x.0.get_text(), x.1.get_text()))
                    .collect::<Vec<_>>()
                    .join(","),
                if self.more == 1 { "^" } else { "" },
                self.c.1
            );
        }

        // string 类型, 空串无法进入第一个 if 流程
        "".to_string()
    }

    /// 获取 trim 空间: 替换为 ~ 后减少的空间
    fn get_trim(chars: &Vec<char>, total: usize) -> usize {
        let mut sum_size = 0;
        for ch in chars {
            if sum_size >= 3 {
                // 从这里开始换成 ~
                return total - (sum_size + 1);
            }
            sum_size += ch.len_utf8();
        }

        // 始终不能 >= 3, 不必 trim
        return 0;
    }

    /// 限制最终文本长度
    fn limit_as_root(&mut self, max_size: usize) {
        self.set_uid(1);

        while self.space > max_size {
            // 统计 string, list 类型, 假定字段总量很小, 不优化统计过程
            let mut uids = Vec::new();
            let string_trim = self.get_max_string_trim(1, &mut uids);
            let (list_trim, list_len, uid) = self.get_min_list_trim((usize::MAX, 1, 0));

            if uids.len() == 0 {
                if uid == 0 {
                    // 无 trim 空间
                    break;
                } else {
                    // 只能 trim list
                    self.trim_match(&vec![uid]);
                }
            } else {
                if uid == 0 {
                    // 只能 trim string
                    self.trim_match(&uids);
                } else {
                    // list_trim / list_len vs 1 / string_trim * count
                    if list_trim * string_trim < 1 * list_len * uids.len() {
                        // 优先 trim list
                        self.trim_match(&vec![uid]);
                    } else {
                        self.trim_match(&uids);
                    }
                }
            }
        }
    }

    /// 构造 dict 类型
    pub fn new_dict(v: Vec<(LimitObj, LimitObj)>, c: (char, char)) -> Self {
        // 总空间: {a:x,b:y,c:z} => {} a:x b:y c:z , ,
        let space = 2 + v.iter().fold(0, |x, y| x + y.0.space + 1 + y.1.space) + (v.len().max(1) - 1);

        // 可裁剪空间: {a:x,b:y,c:z} => {a:x,b:y^}
        let trim = if v.len() > 1 {
            v[v.len() - 1].0.space + 1 + v[v.len() - 1].1.space
        } else {
            0
        };

        Self {
            v2: Some(v),
            c,
            space,
            trim,
            ..Self::new_empty()
        }
    }

    /// 构造
    fn new_empty() -> Self {
        Self {
            uid: 0,
            chars: Vec::new(),
            v1: None,
            v2: None,
            c: (' ', ' '),
            space: 0,
            trim: 0,
            more: 0,
        }
    }

    /// 构造, uid 后续统一设置
    pub fn new_list(v: Vec<LimitObj>, c: (char, char), fix: bool) -> Self {
        // 总空间: [x,y,z] => [] x y z , ,
        let space = 2 + v.iter().fold(0, |x, y| x + y.space) + (v.len().max(1) - 1);

        // 可裁剪空间: [x,y,z] => [x,y^]
        let trim = if fix {
            0
        } else {
            if v.len() > 1 {
                v[v.len() - 1].space
            } else {
                0
            }
        };

        Self {
            v1: Some(v),
            c,
            space,
            trim,
            ..Self::new_empty()
        }
    }

    /// 构造, uid 后续统一设置
    pub fn new_string(text: String, fix: bool) -> Self {
        // 总空间
        let space = text.len();

        // 拆解
        let chars = text.chars().collect();

        // 可裁剪空间: abcxyz => abc~
        let trim = if fix { 0 } else { Self::get_trim(&chars, space) };

        Self {
            chars,
            space,
            trim,
            ..Self::new_empty()
        }
    }

    /// 设置裁剪过程中的唯一编号
    fn set_uid(&mut self, mut uid: u32) -> u32 {
        self.uid = uid;
        uid = uid + 1;

        // 递归
        if let Some(v) = &mut self.v1 {
            for obj in v {
                uid = obj.set_uid(uid);
            }
        }

        // 递归
        if let Some(v) = &mut self.v2 {
            for (k, v) in v {
                uid = k.set_uid(uid);
                uid = v.set_uid(uid);
            }
        }

        uid
    }

    /// 裁剪, 假定调用者保证可裁剪
    fn trim(&mut self) -> usize {
        // string 类型
        if self.chars.len() >= 2 {
            let last = self.chars.pop().unwrap(); // len() >= 2 确保 last() 有效
            let prev = self.chars.pop().unwrap(); // len() >= 2 确保 last() 有效
            let trim = last.len_utf8() + prev.len_utf8() - 1;
            self.chars.push('~');
            self.space -= trim;
            self.trim -= trim;
            return trim;
        }

        // list 类型, [x,y,z] => [x,y^] or [x,y,z^] => [x,y^]
        if let Some(v) = &mut self.v1 {
            let cut = v[v.len() - 1].space + self.more;
            v.remove(v.len() - 1);
            self.space -= cut;
            self.trim = if v.len() > 1 {
                v[v.len() - 1].space + self.more
            } else {
                0
            };
            self.more = 1;
            return cut;
        }

        // dict 类型, {a:x,b:y,c:z} => {a:x,b:y^} or {a:x,b:y,c:z^} => {a:x,b:y^}
        if let Some(v) = &mut self.v2 {
            let cut = v[v.len() - 1].0.space + 1 + v[v.len() - 1].1.space + self.more;
            v.remove(v.len() - 1);
            self.space -= cut;
            self.trim = if v.len() > 1 {
                v[v.len() - 1].0.space + 1 + v[v.len() - 1].1.space + self.more
            } else {
                0
            };
            self.more = 1;
            return cut;
        }

        // 正常不应该走到这里, 返回 1 的目的是: 即使什么也没做, limit 循环会因此而结束
        return 1;
    }

    /// 对 uids 中的元素执行 trim
    fn trim_match(&mut self, uids: &Vec<u32>) -> usize {
        // string 类型
        if self.chars.len() >= 2 && uids.contains(&self.uid) {
            return self.trim();
        }

        let mut total_cut = 0;

        // list 类型
        if let Some(v) = &mut self.v1 {
            if uids.contains(&self.uid) {
                return self.trim();
            }
            for obj in v {
                total_cut += obj.trim_match(uids);
            }
        }

        // dict 类型
        if let Some(v) = &mut self.v2 {
            if uids.contains(&self.uid) {
                return self.trim();
            }
            for (k, v) in v {
                total_cut += k.trim_match(uids) + v.trim_match(uids);
            }
        }

        self.space -= total_cut;
        total_cut
    }
}

pub trait LimitPackAble {
    fn to_limit_obj(&self) -> LimitObj;

    fn limit_pack(&self, limit: usize) -> String {
        let mut root = self.to_limit_obj();
        root.limit_as_root(limit);
        root.get_text()
    }
}

macro_rules! default_limit_pack {
    ($type:ident, $fix:expr) => {
        impl LimitPackAble for $type {
            fn to_limit_obj(&self) -> LimitObj {
                LimitObj::new_string(format!("{}", self), $fix)
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
    fn to_limit_obj(&self) -> LimitObj {
        (**self).to_limit_obj()
    }
}

impl<T> LimitPackAble for &mut T
where
    T: LimitPackAble + ?Sized,
{
    fn to_limit_obj(&self) -> LimitObj {
        (**self).to_limit_obj()
    }
}

impl<T> LimitPackAble for Option<T>
where
    T: LimitPackAble,
{
    fn to_limit_obj(&self) -> LimitObj {
        match self {
            Some(obj) => obj.to_limit_obj(),
            None => LimitObj::new_string("None".to_string(), true),
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
            fn to_limit_obj(&self) -> LimitObj {
                let ($(ref $name,)+) = *self;

                LimitObj::new_list(vec![
                    $(
                        $name.to_limit_obj(),
                    )+
                ], ('(',')'), true)
            }
        }
        peel! { $($name,)+ }
    )
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple! { E, D, C, B, A, Z, Y, X, W, V, U, T, }

impl<T> LimitPackAble for [T]
where
    T: LimitPackAble,
{
    fn to_limit_obj(&self) -> LimitObj {
        LimitObj::new_list(self.iter().map(|x| x.to_limit_obj()).collect(), ('[', ']'), false)
    }
}

impl<T, const N: usize> LimitPackAble for [T; N]
where
    T: LimitPackAble,
{
    fn to_limit_obj(&self) -> LimitObj {
        LimitObj::new_list(self.iter().map(|x| x.to_limit_obj()).collect(), ('[', ']'), false)
    }
}

impl<T> LimitPackAble for Vec<T>
where
    T: LimitPackAble,
{
    fn to_limit_obj(&self) -> LimitObj {
        LimitObj::new_list(self.iter().map(|x| x.to_limit_obj()).collect(), ('[', ']'), false)
    }
}

impl<T> LimitPackAble for VecDeque<T>
where
    T: LimitPackAble,
{
    fn to_limit_obj(&self) -> LimitObj {
        LimitObj::new_list(self.iter().map(|x| x.to_limit_obj()).collect(), ('[', ']'), false)
    }
}

impl<T, S> LimitPackAble for HashSet<T, S>
where
    T: LimitPackAble,
{
    fn to_limit_obj(&self) -> LimitObj {
        LimitObj::new_list(self.iter().map(|x| x.to_limit_obj()).collect(), ('{', '}'), false)
    }
}

impl<K, V, S> LimitPackAble for HashMap<K, V, S>
where
    K: LimitPackAble,
    V: LimitPackAble,
{
    fn to_limit_obj(&self) -> LimitObj {
        LimitObj::new_dict(
            self.iter().map(|(x, y)| (x.to_limit_obj(), y.to_limit_obj())).collect(),
            ('{', '}'),
        )
    }
}
