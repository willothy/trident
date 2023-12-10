use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Node {
    children: Vec<Node>,
    key: Option<char>,
    // Some(str) means that the node is an end node
    val: Option<String>,
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_key(c: char) -> Self {
        Node {
            key: Some(c),
            ..Self::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: Node::new() }
    }

    pub fn insert(&mut self, s: impl AsRef<str>) {
        let s = s.as_ref();
        let mut current = &mut self.root;

        for c in s.chars() {
            match current.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(idx) => {
                    current = &mut current.children[idx];
                }
                Err(i) => {
                    current.children.insert(i, Node::with_key(c));
                    current = &mut current.children[i];
                }
            }
        }

        current.val.replace(s.to_string());
    }

    pub fn contains(&self, s: impl AsRef<str>) -> bool {
        let s = s.as_ref();
        let mut current = &self.root;
        for c in s.chars() {
            match current.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(idx) => {
                    current = &current.children[idx];
                }
                Err(_) => {
                    return false;
                }
            }
        }
        current.val.is_some()
    }

    pub fn remove(&mut self, s: impl AsRef<str>) {
        let s = s.as_ref();
        let mut current = &mut self.root;

        for c in s.chars() {
            match current.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(idx) => {
                    let next = &mut current.children[idx];
                    if next.children.len() == 0 {
                        current.children.remove(idx);
                    } else {
                        current.val = None;
                        current = &mut current.children[idx];
                    }
                }
                Err(_) => break,
            }
        }
        current.val = None;
    }

    pub fn search_rec(&self, s: &mut str) -> Vec<&String> {
        fn inner_search<'b, 'a: 'b>(
            node: &'a Node,
            s: &mut str,
            depth: usize,
            results: &mut Vec<&'b String>,
        ) {
            let len = s.len();
            if len == 0 {
                for child in &node.children {
                    if let Some(val) = &child.val {
                        results.push(&val);
                    }
                    inner_search(child, &mut s[len.min(1)..], depth + 1, results);
                }
            } else {
                if let Ok(idx) = node
                    .children
                    .binary_search_by(|node| node.key.cmp(&s.chars().next()))
                {
                    if len == 1 {
                        if let Some(val) = &node.children[idx].val {
                            results.push(val);
                        }
                    }
                    inner_search(&node.children[idx], &mut s[1..], depth + 1, results);
                    return;
                }
            }
        }

        let mut results = Vec::new();

        inner_search(&self.root, s, 1, &mut results);

        results
    }

    pub fn search_rec2(&self, s: impl AsRef<str>) -> Vec<&String> {
        fn inner_search<'b, 'a: 'b>(
            node: &'a Node,
            s: &mut std::iter::Peekable<impl Iterator<Item = char>>,
            results: &mut Vec<&'b String>,
        ) {
            if let Some(prefix) = s.next() {
                if let Ok(idx) = node
                    .children
                    .binary_search_by(|node| node.key.cmp(&Some(prefix)))
                {
                    if node.children[idx].val.is_some() && s.peek().is_none() {
                        results.push(node.children[idx].val.as_ref().unwrap())
                    }
                    inner_search(&node.children[idx], s, results);
                }
            } else {
                for child in &node.children {
                    if let Some(val) = &child.val {
                        results.push(&*val)
                    }
                    inner_search(child, s, results);
                }
            }
        }

        let mut results = Vec::with_capacity(64);

        let mut iter = s.as_ref().chars().peekable();

        inner_search(&self.root, &mut iter, &mut results);

        results
    }

    pub fn search(&self, s: impl AsRef<str>) -> Vec<String> {
        let mut results = Vec::new();

        let mut current = &self.root;
        for c in s.as_ref().chars() {
            match current
                .children
                .binary_search_by(|node| node.key.cmp(&Some(c)))
            {
                Ok(i) => {
                    current = &current.children[i];
                }
                Err(_) => {
                    return results;
                }
            }
        }

        let mut queue: VecDeque<&Node> = VecDeque::from([current]);

        while let Some(node) = queue.pop_front() {
            if let Some(str) = &node.val {
                results.push(str.clone());
            }
            for child in &node.children {
                queue.push_back(child);
            }
        }

        results.sort();

        results
    }
}

pub trait Sorted {
    fn sorted(self) -> Self;
}

impl Sorted for Vec<String> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

impl<'a> Sorted for Vec<&'a String> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

impl<'a> Sorted for Vec<&'a str> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

#[test]
fn search_recursive2() {
    let mut trie = Trie::new();
    trie.insert("hellish");
    trie.insert("hello");
    trie.insert("world");
    trie.insert("hell");
    trie.insert("help");

    assert_eq!(
        trie.search_rec2("he"),
        vec!["hell", "hello", "hellish", "help"].sorted(),
        "recursive 2"
    );
    assert_eq!(
        trie.search_rec2("hell"),
        vec!["hell", "hello", "hellish"].sorted(),
        "recursive 2"
    );
    assert_eq!(trie.search_rec2("world"), vec!["world"], "recursive 2");
    assert_eq!(
        trie.search_rec2("worlds"),
        Vec::<&String>::new(),
        "recursive 2"
    );
}

#[test]
fn search_iterative() {
    let mut trie = Trie::new();
    trie.insert("hellish");
    trie.insert("hello");
    trie.insert("world");
    trie.insert("hell");
    trie.insert("help");
    assert_eq!(
        trie.search("he"),
        vec!["hell", "hello", "hellish", "help"].sorted(),
        "iterative"
    );
    assert_eq!(
        trie.search("hell"),
        vec!["hell", "hello", "hellish"].sorted(),
        "iterative"
    );
    assert_eq!(trie.search("world"), vec!["world"], "iterative");
    assert_eq!(trie.search("worlds"), Vec::<String>::new(), "iterative");
}

#[test]
fn search_recursive1() {
    let mut trie = Trie::new();
    trie.insert("hellish");
    trie.insert("hello");
    trie.insert("world");
    trie.insert("hell");
    trie.insert("help");

    assert_eq!(
        trie.search_rec(&mut String::from("he")),
        vec!["hell", "hello", "hellish", "help"].sorted(),
        "recursive 1"
    );
    assert_eq!(
        trie.search_rec(&mut String::from("hell")),
        vec!["hell", "hello", "hellish"].sorted(),
        "recursive 1"
    );
    assert_eq!(
        trie.search_rec(&mut String::from("world")),
        vec!["world"],
        "recursive 1"
    );
    assert_eq!(
        trie.search_rec(&mut String::from("worlds")),
        Vec::<&String>::new(),
        "recursive 1"
    );
}

#[test]
fn insert() {
    let mut trie = Trie::new();
    trie.insert("hello");
    trie.insert("world");
    trie.insert("hell");
    trie.insert("help");
    trie.insert("hellish");
    assert!(trie.contains("hello"));
    assert!(trie.contains("hell"));
    assert!(trie.contains("hellish"));
    assert!(trie.contains("help"));
    assert!(trie.contains("world"));

    assert!(!trie.contains("hellis"));
    assert!(!trie.contains("hellishhh"));
    assert!(!trie.contains("helps"));
    assert!(!trie.contains("worlds"));
    assert!(!trie.contains("worldss"));
}

#[test]
fn remove() {
    let mut trie = Trie::new();
    trie.insert("hello");
    trie.insert("world");

    assert!(trie.contains("hello"));
    assert!(trie.contains("world"));

    trie.remove("hello");

    assert!(!trie.contains("hello"));
    assert!(trie.contains("world"));

    let mut trie = Trie::new();

    // Empty trie
    trie.remove("hello");
    assert!(!trie.contains("hello"));

    // Single word
    trie.insert("hello");
    trie.remove("hello");
    assert!(!trie.contains("hello"));

    // Multiple words
    trie.insert("hello");
    trie.insert("world");
    trie.remove("hello");
    assert!(!trie.contains("hello"));
    assert!(trie.contains("world"));

    // Remove non-existent word
    trie.remove("foo");
    assert!(!trie.contains("foo"));

    // Remove prefix of existing word
    trie.insert("hello");
    trie.insert("helloworld");
    trie.remove("hello");
    assert!(!trie.contains("hello"));
    assert!(trie.contains("helloworld"));
}
