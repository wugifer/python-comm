use std::collections::HashMap;

/// 关键字查找节点
///
/// 因为采用 usize 作为内部引用, 因此 TextSearch 一旦建立, 不允许修改
struct KeywordNode {
    // 关键字
    letters: Vec<char>,

    // 仅用于蓝色节点, 节点名, 缺省是关键字, 可设置别名或用于替换的名字
    name: String,

    // 蓝色节点
    is_blue: bool,
}

impl KeywordNode {
    /// 获取蓝色节点名
    fn name(&self) -> String {
        self.name.clone()
    }

    /// 构造
    fn new(letters: Vec<char>) -> Self {
        Self {
            letters,
            name: String::new(),
            is_blue: false,
        }
    }

    /// debug 用
    #[cfg(test)]
    fn to_string(&self) -> String {
        format!("{:?}, {}, {}", self.letters, self.name, self.is_blue)
    }
}

#[cfg(test)]
mod keyword_node_test {
    use super::*;

    #[test]
    fn test_new() {
        let node = KeywordNode::new("abc".chars().collect::<Vec<char>>());
        assert_eq!(node.to_string(), "[\'a\', \'b\', \'c\'], , false");
    }
}

/// 基于 Aho–Corasick 算法的全文匹配/替换
///
/// ## Aho–Corasick 算法
/// Aho–Corasick 算法通过预先定义的字典, 只扫描一遍文本, 可以完成多个关键字的查找、替换。
///
/// 示例
///
/// 关键字: {a, ab, bab, bc, bca, c, caa}.
///
/// 构造
/// 1. 每个关键字的每个前缀对应 trie 中的一个节点, 比如 bab 对应 (), b, ba, bab 四个节点;
/// 2. 不同关键字可共享节点, 比如 bab, bca 共享 (), b 节点;
/// 3. 对应关键字的节点为**蓝色节点**, 比如 bab 节点, 仅对应前缀的节点为灰色节点, 比如 ba 节点;
/// 4. 同一关键字的相邻前缀间用黑色箭头连接, 比如 ba --> bab;
/// 5. 除根节点外, 每个节点用**蓝色箭头**指向它的最长有效真后缀, 比如 caa 的真后缀包括 aa, a, (), 其中 a, () 在树中,
///    a 是最长的, 所以蓝色箭头 caa --> a
///
/// 节点   颜色	 蓝箭头
///
/// ()	   灰   -
///
/// a	   蓝	()
///
/// ab     蓝	b
///
/// b	   灰	()
///
/// ba     灰	a
///
/// bab    蓝	ab
///
/// bc     蓝	c
///
/// bca    蓝	ca
///
/// c      蓝	()
///
/// ca     灰	a
///
/// caa    蓝	a
///
///  查找
///  1. 从当前节点出发,
///
///    a) 沿黑色箭头匹配下一个字符, 切换到新节点;
///
///    b) 如果不存在有效的黑色箭头, 沿蓝箭头到下一个节点, 重复步骤 a) b);
///
///  2. 重复步骤 1. 直到没有字符需要匹配
///
///  搜索 abccab 过程
///
///  节点    剩余字符串    查找过程                输出
///
///  ()     abccab      黑箭 a 有效
///
///  a      bccab       黑箭 ab 有效             蓝点 a
///
///  ab     ccab        黑箭 abc 无效, 蓝箭 b     蓝点 ab
///
///  b      ccab        黑箭 bc 有效
///
///  bc     cab         黑箭 bcc 无效, 蓝箭 c     蓝点 bc
///
///  c      cab         黑箭 cc 无效, 蓝箭 ()     蓝点 c
///
///  ()     cab         黑箭 c 有效
///
///  c      ab          黑箭 ca 有效             蓝点 c
///
///  ca     b           黑箭 cab 无效, 蓝箭 a
///
///  a      b           黑箭 ab 有效             蓝点 a
///
///  ab     -           -                       蓝点 ab
///
/// ## 用法
///
/// Step1.  ts = TextSearcher::new();
///
/// Step2.  ts.add_keyword();  // 可添加多个关键字
///
/// Step3.  ts.create_blues();
///
/// Step4.  ts.search() / ts.replace();  // ts 可复用
///
/// ```
/// use python_comm::prelude::TextSearcher;
///
/// let mut ts0 = TextSearcher::new();
/// let mut ts1 = TextSearcher::new();
/// for (keyword, title) in &[("bcdef", "X"), ("defghi", "Y"), ("hijk", "Z")] {
///     ts0.add_keyword(String::from(*keyword), None);
///     ts1.add_keyword(String::from(*keyword), Some(String::from(*title)));
/// }
/// ts0.create_blues();
/// ts1.create_blues();
///
/// assert_eq!(
///     ts0.search("abcdefghijklmn"),
///     [
///         (String::from("bcdef"), 1, 6),    // 返回匹配的每个关键字及起始位置
///         (String::from("defghi"), 3, 9),
///         (String::from("hijk"), 7, 11)
///     ]
/// );
/// assert_eq!(
///     ts1.search("abcdefghijklmn"),
///     [
///         (String::from("X"), 1, 6),    // 返回匹配的每个关键字别名及起始位置
///         (String::from("Y"), 3, 9),
///         (String::from("Z"), 7, 11)
///     ]
/// );
/// assert_eq!(
///     ts1.replace("abcdefghijklmn"),    // 替换匹配的每个关键字, 如果出现重叠则不替换
///     "aXgZlmn"
/// );
/// ```
///
pub struct TextSearcher {
    // 节点
    nodes: Vec<KeywordNode>,

    // 黑色箭头, node + letter -> node
    blacks: HashMap<(usize, char), usize>,

    // 蓝色箭头, node -> node
    blues: HashMap<usize, usize>,
}

impl TextSearcher {
    /// 添加关键字
    pub fn add_keyword(&mut self, keyword: String, name: Option<String>) {
        // 从根节点出发
        let mut node_id = 1;

        // 构造 keyword 的每个节点
        let mut letters = Vec::new();
        for letter in keyword.chars() {
            letters.push(letter);
            if let Some(&next_node_id) = self.blacks.get(&(node_id, letter)) {
                // 存在, 继续
                node_id = next_node_id;
            } else {
                // 不存在, 创建
                self.nodes.push(KeywordNode::new(letters.clone()));
                let next_node_id = self.nodes.len();
                // 创建黑色箭头, 继续
                self.blacks.insert((node_id, letter), next_node_id);
                node_id = next_node_id;
            }
        }

        // 设为蓝色节点
        let node = &mut self.nodes[node_id - 1];
        node.is_blue = true;

        // 用 name 或 keyword 命名
        if let Some(name) = name {
            node.name = name;
        } else {
            node.name = keyword;
        }
    }

    /// 创建蓝色箭头
    pub fn create_blues(&mut self) {
        // 遍历每个节点
        for node_id in 1..=self.nodes.len() {
            let letters = &self.nodes[node_id - 1].letters;
            let letters_len = letters.len();

            // 遍历每个真后缀
            for start in 1..letters_len {
                // 如果真后缀也在树中, 创建蓝色箭头, 只要最长后缀
                let target_node_id = self.get_node_by_keyword(&letters[start..]);
                if target_node_id != 0 {
                    self.blues.insert(node_id, target_node_id);
                    break;
                }
            }
        }
    }

    /// 获取关键字在 tree 中的位置
    fn get_node_by_keyword(&self, keyword: &[char]) -> usize {
        // 从根节点出发
        let mut node_id = 1;

        // 依次查询一个 letter
        for letter in keyword {
            if let Some(&next_node_id) = self.blacks.get(&(node_id, *letter)) {
                node_id = next_node_id;
            } else {
                // 一旦无法命中, 失败, 返回 0
                return 0;
            }
        }

        return node_id;
    }

    /// 沿黑色或蓝色箭头前进
    fn move_front(
        &self,
        node_id: usize,
        letter: char,
    ) -> (
        usize, // 新的 node
        bool,  // 是否消耗 letter
    ) {
        if let Some(&next_node_id) = self.blacks.get(&(node_id, letter)) {
            // 沿黑色箭头前进, 消耗 letter
            (next_node_id, true)
        } else {
            if let Some(&next_node_id) = self.blues.get(&node_id) {
                // 沿蓝色箭头前进
                (next_node_id, false)
            } else {
                // 根节点, 消耗/不消耗 letter
                (1, if node_id == 1 { true } else { false })
            }
        }
    }

    /// 构造
    pub fn new() -> Self {
        Self {
            nodes: vec![KeywordNode::new(Vec::new())],
            blacks: HashMap::new(),
            blues: HashMap::new(),
        }
    }

    /// 替换
    pub fn replace(&self, text: &str) -> String {
        // 从 root 出发
        let mut result: (String, usize) = (String::new(), 0);
        let mut last_found: (String, usize, usize) = (String::new(), 0, 0);
        let mut node_id = 1;
        let mut posy = 0;

        // 遍历每个字符
        let letters = text.chars().collect::<Vec<char>>();
        for letter in &letters {
            posy += 1;
            loop {
                // 沿黑色或蓝色箭头前进
                let (next_node_id, used) = self.move_front(node_id, *letter);
                node_id = next_node_id;
                let node = &self.nodes[node_id - 1];
                // 检查蓝色节点
                if node.is_blue {
                    let found = if used {
                        (node.name(), posy - node.letters.len(), posy)
                    } else {
                        (node.name(), posy - node.letters.len() - 1, posy - 1)
                    };
                    if found.1 != last_found.1 {
                        // 使用上一次的结果
                        if last_found.1 >= result.1 {
                            for i in result.1..last_found.1 {
                                result.0.push(letters[i]);
                            }
                            result.0 += &last_found.0;
                            result.1 = last_found.2;
                        }
                        // else: 两次结果有交叉, 并且第一次已经使用, 放弃第二次的
                    }
                    last_found = found;
                }
                if used {
                    break;
                }
            }
        }

        // 使用上一次的结果
        if last_found.2 >= last_found.1 {
            for i in result.1..last_found.1 {
                result.0.push(letters[i]);
            }
            result.0 += &last_found.0;
            result.1 = last_found.2;
        }

        // 使用末尾数据
        for letter in &letters[result.1..] {
            result.0.push(*letter);
        }

        result.0
    }

    /// 查找
    pub fn search(&self, text: &str) -> Vec<(String, usize, usize)> {
        // 从 root 出发
        let mut names = Vec::new();
        let mut node_id = 1;
        let mut posy = 0;

        // 遍历每个字符
        for letter in text.chars() {
            posy += 1;
            loop {
                // 沿黑色或蓝色箭头前进
                let (next_node_id, used) = self.move_front(node_id, letter);
                node_id = next_node_id;
                let node = &self.nodes[node_id - 1];
                // 输出蓝色节点
                if node.is_blue {
                    if used {
                        // 含当前字符
                        names.push((node.name(), posy - node.letters.len(), posy));
                    } else {
                        // 不含当前字符
                        names.push((node.name(), posy - node.letters.len() - 1, posy - 1));
                    }
                }
                // 下一个字符
                if used {
                    break;
                }
            }
        }

        names
    }
}

#[cfg(test)]
mod text_searcher_test {
    use super::*;

    #[test]
    fn test_add_keyword1() {
        let mut ts = TextSearcher::new();
        ts.add_keyword(String::from("ab"), None);

        assert_eq!(ts.nodes.len(), 3);
        assert_eq!(ts.nodes[1].to_string(), "[\'a\'], , false");
        assert_eq!(ts.nodes[2].to_string(), "[\'a\', \'b\'], ab, true");
        let mut blacks = ts
            .blacks
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<((usize, char), usize)>>();
        blacks.sort();
        assert_eq!(blacks, [((1, 'a'), 2), ((2, 'b'), 3)]);
    }

    #[test]
    fn test_add_keyword2() {
        let mut ts = TextSearcher::new();
        for keyword in &["a", "ab", "bab", "bc", "bca", "c", "caa"] {
            ts.add_keyword(String::from(*keyword), None);
        }

        println!(
            "{:?}",
            ts.nodes
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );
        assert_eq!(ts.nodes.len(), 11);
        assert_eq!(ts.nodes[6].to_string(), "[\'b\', \'c\'], bc, true");
        let mut blacks = ts
            .blacks
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<((usize, char), usize)>>();
        blacks.sort();
        assert_eq!(
            blacks,
            [
                ((1, 'a'), 2),
                ((1, 'b'), 4),
                ((1, 'c'), 9),
                ((2, 'b'), 3),
                ((4, 'a'), 5),
                ((4, 'c'), 7),
                ((5, 'b'), 6),
                ((7, 'a'), 8),
                ((9, 'a'), 10),
                ((10, 'a'), 11)
            ]
        );
    }

    #[test]
    fn test_create_blues() {
        let mut ts = TextSearcher::new();
        for keyword in &["a", "ab", "bab", "bc", "bca", "c", "caa"] {
            ts.add_keyword(String::from(*keyword), None);
        }
        ts.create_blues();

        let mut blues = ts
            .blues
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(usize, usize)>>();
        blues.sort();
        assert_eq!(
            blues,
            [(3, 4), (5, 2), (6, 3), (7, 9), (8, 10), (10, 2), (11, 2)]
        );
    }

    #[test]
    fn test_get_node_by_keyword() {
        let mut ts = TextSearcher::new();
        let mut ids = Vec::new();
        for keyword in &["a", "ab", "bab", "bc", "bca", "c", "caa"] {
            ts.add_keyword(String::from(*keyword), None);
            ids.push(ts.nodes.len());
        }

        let mut i = 0;
        for keyword in &["a", "ab", "bab", "bc", "bca", "c", "caa"] {
            println!("{}", keyword);
            assert_eq!(
                ts.get_node_by_keyword(&keyword.chars().collect::<Vec<char>>()),
                ids[i]
            );
            i += 1;
        }

        assert_eq!(
            ts.get_node_by_keyword(&"ac".chars().collect::<Vec<char>>()),
            0
        );
        assert_eq!(
            ts.get_node_by_keyword(&"xy".chars().collect::<Vec<char>>()),
            0
        );
    }

    #[test]
    fn test_new() {
        let ts = TextSearcher::new();
        assert_eq!(ts.nodes.len(), 1);
        assert_eq!(ts.blacks.len(), 0);
        assert_eq!(ts.blues.len(), 0);

        assert_eq!(ts.nodes[0].to_string(), "[], , false");
    }

    #[test]
    fn test_replace1() {
        let mut ts = TextSearcher::new();
        for keyword in &["a", "ab", "bab", "bc", "bca", "c", "caa"] {
            ts.add_keyword(String::from(*keyword), Some(format!("x{}y", keyword)));
        }
        ts.create_blues();

        assert_eq!(ts.replace("abccab"), "xabyxcyxcyxaby");
    }

    #[test]
    fn test_replace2() {
        let mut ts = TextSearcher::new();
        for keyword in &["bcdef", "defghi", "hijk"] {
            ts.add_keyword(String::from(*keyword), Some(format!("x{}y", keyword)));
        }
        ts.create_blues();

        assert_eq!(ts.replace("abcdefghijklmn"), "axbcdefygxhijkylmn");
    }

    #[test]
    fn test_search1() {
        let mut ts = TextSearcher::new();
        for keyword in &["a", "ab", "bab", "bc", "bca", "c", "caa"] {
            ts.add_keyword(String::from(*keyword), None);
        }
        ts.create_blues();

        assert_eq!(
            ts.search("abccab"),
            [
                (String::from("a"), 0, 1),
                (String::from("ab"), 0, 2),
                (String::from("bc"), 1, 3),
                (String::from("c"), 2, 3),
                (String::from("c"), 3, 4),
                (String::from("a"), 4, 5),
                (String::from("ab"), 4, 6)
            ]
        );
    }

    #[test]
    fn test_search2() {
        let mut ts = TextSearcher::new();
        for keyword in &["北京", "欢迎", "你"] {
            ts.add_keyword(String::from(*keyword), None);
        }
        ts.create_blues();

        assert_eq!(
            ts.search("北京欢迎你"),
            [
                (String::from("北京"), 0, 2),
                (String::from("欢迎"), 2, 4),
                (String::from("你"), 4, 5),
            ]
        );
    }

    #[test]
    fn test_search3() {
        let mut ts = TextSearcher::new();
        for keyword in &["bcdef", "defghi", "hijk"] {
            ts.add_keyword(String::from(*keyword), Some(format!("x{}y", keyword)));
        }
        ts.create_blues();

        assert_eq!(
            ts.search("abcdefghijklmn"),
            [
                (String::from("xbcdefy"), 1, 6),
                (String::from("xdefghiy"), 3, 9),
                (String::from("xhijky"), 7, 11)
            ]
        );
    }
}
