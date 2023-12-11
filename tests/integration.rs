use trident::Sorted;
use trident::Trie;

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
