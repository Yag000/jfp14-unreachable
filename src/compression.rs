use core::str;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    i32,
};

use crate::program::{Mode, Program};

#[derive(Clone, Eq, PartialEq)]
pub struct Node {
    leaf: Option<String>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct PriorityElement {
    priority: i32,
    element: Node,
}

impl PriorityElement {
    pub fn new(priority: i32, node: Node) -> Self {
        PriorityElement {
            priority,
            element: node,
        }
    }
}

impl Ord for PriorityElement {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for PriorityElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn new() -> Self {
        Node {
            leaf: None,
            left: None,
            right: None,
        }
    }

    pub fn new_leaf(c: String) -> Self {
        Node {
            leaf: Some(c),
            left: None,
            right: None,
        }
    }
}

fn count_occ(s: &String) -> HashMap<String, i32> {
    let mut hash: HashMap<String, i32> = HashMap::new();

    for i in 0..s.len() / 4 + 1 {
        let slice = s[..i].to_string();
        let occ = s.split(&slice).collect::<Vec<&str>>().len() as i32;
        let len = slice.len() as i32;

        s.chars().for_each(|c| {
            match hash.get_mut(&c.to_string()) {
                Some(v) => {
                    if *v >= 30 {
                        *v = 30
                    } else {
                        *v += 1;
                    }
                }
                None => {
                    hash.insert(c.to_string(), 1);
                }
            };
        });

        hash.insert(slice, occ * len - 10);
    }

    hash
}

fn hash_to_huffman_tree(hash: HashMap<String, i32>) -> Node {
    if hash.len() == 1 {
        let value: &String = hash.keys().nth(0).unwrap();
        return Node::new_leaf(value.to_string());
    }

    let mut heap = BinaryHeap::new();
    hash.iter()
        .for_each(|(c, p)| heap.push(PriorityElement::new(*p, Node::new_leaf(c.to_string()))));

    while heap.len() > 1 {
        let fst = heap.pop().unwrap();
        let snd = heap.pop().unwrap();

        let mut new_node = Node::new();

        if fst.priority <= snd.priority {
            new_node.left = Some(Box::new(fst.element));
            new_node.right = Some(Box::new(snd.element));
        } else {
            new_node.left = Some(Box::new(snd.element));
            new_node.right = Some(Box::new(fst.element));
        }

        heap.push(PriorityElement::new(fst.priority + snd.priority, new_node));
    }

    heap.pop().unwrap().element
}

fn parcours_huffman_tree(
    vector: &mut Vec<(String, String)>,
    node: Node,
    repr: &str,
) -> Vec<(String, String)> {
    if let Some(left) = node.left {
        parcours_huffman_tree(vector, *left, &format!("{repr}0"));
    };
    if let Some(right) = node.right {
        parcours_huffman_tree(vector, *right, &format!("{repr}1"));
    };
    if let Some(leaf) = node.leaf {
        vector.push((leaf, repr.to_string()));
    };
    vector.to_vec()
}

pub fn eval_huffman_tree(node: Node) -> HashMap<String, String> {
    if node.left.is_none() || node.right.is_none() {
        let mut hash: HashMap<String, String> = HashMap::new();
        hash.insert(node.leaf.unwrap(), "0".to_string());
        return hash;
    }

    let mut v = Vec::new();
    parcours_huffman_tree(&mut v, node, "");
    let mut hash = HashMap::new();
    v.iter().for_each(|(c, r)| {
        hash.insert(c.to_string(), r.to_string());
    });
    hash
}

fn simplify_huff(code: Program) -> Program {
    let mut hash = HashMap::new();

    let max_len = code.instr.iter().map(|(_, b)| b.len()).max().unwrap();

    for (value, key) in code.instr {
        hash.insert(value, (max_len - key.len()) as i32);
    }

    let node = hash_to_huffman_tree(hash);
    let binding = eval_huffman_tree(node);
    let hash_: Vec<(&String, &String)> = binding.iter().collect();

    let vec_ = hash_
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();

    Program::new(vec_, Mode::Compress)
}

pub struct Compressed {
    compressed: String,
    code: Program,
}

impl Compressed {
    fn new(input: String, code: Program) -> Compressed {
        Compressed {
            compressed: input,
            code,
        }
    }
}

pub fn compress(mut init: String, result: HashMap<String, String>) -> Compressed {
    let tmp: Vec<(&String, &String)> = result.iter().collect();
    let prog = Program::new(
        tmp.iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect(),
        Mode::Compress,
    );

    let mut init_copy = init.clone();

    let mut hit_count: HashSet<(String, String)> = HashSet::new();

    while !init.is_empty() {
        for (key, value) in prog.instr.iter() {
            if let Some(suff) = init.strip_prefix(key.as_str()) {
                init = suff.to_string();
                hit_count.insert((key.to_string(), value.to_string()));
                break;
            };
        }
    }

    let hit: Vec<(String, String)> = hit_count
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();

    let program = Program::new(hit, Mode::Compress);

    let new_prog = simplify_huff(program);

    let mut answer = String::new();

    while !init_copy.is_empty() {
        for (key, value) in new_prog.instr.iter() {
            if let Some(suff) = init_copy.strip_prefix(key.as_str()) {
                answer.push_str(value);
                init_copy = suff.to_string();
                break;
            };
        }
    }

    Compressed::new(answer, new_prog)
}
pub fn display(init: String, result: HashMap<String, String>) -> String {
    let compressed = compress(init, result);

    let code = compressed
        .code
        .instr
        .iter()
        .fold(String::new(), |acc, (v, k)| format!("{acc}{k}={v}\n"));

    format!("{code}\n{}", compressed.compressed)
}

pub fn compress_word(word: String) -> String {
    let node = hash_to_huffman_tree(count_occ(&word));
    let hash = eval_huffman_tree(node);
    display(word, hash)
}

fn _read_words() -> HashMap<String, u32> {
    let input = std::fs::read_to_string("tables_mots").unwrap();

    let mut hash = HashMap::new();

    let mut line_iter = input.lines();

    while let Some(word) = line_iter.next() {
        if word.is_empty() {
            break;
        }

        if let Some(value) = line_iter.next() {
            if value.is_empty() {
                break;
            }

            hash.insert(word.to_string(), value.parse::<u32>().unwrap() + 10000);
        } else {
            break;
        }
    }

    hash
}

fn _q4() {
    let _hash = _read_words();
}

#[cfg(test)]
mod test {
    use super::{count_occ, eval_huffman_tree, hash_to_huffman_tree};

    #[test]
    fn count() {
        let hash = count_occ(&String::from("bcaadddccacacac"));
        assert_eq!(hash.get("a"), Some(&5));
        assert_eq!(hash.get("b"), Some(&1));
        assert_eq!(hash.get("c"), Some(&6));
        assert_eq!(hash.get("d"), Some(&3));

        assert_eq!(hash.get("z"), None);
    }

    #[test]
    fn huffman() {
        let node = hash_to_huffman_tree(count_occ(&String::from("bcaadddccacacac")));
        let right = node.right.unwrap();
        let right_left = right.left.unwrap();

        assert_eq!(node.left.unwrap().leaf.unwrap(), "c".to_string());
        assert_eq!(right.right.unwrap().leaf.unwrap(), "a".to_string());
        assert_eq!(right_left.left.unwrap().leaf.unwrap(), "b".to_string());
        assert_eq!(right_left.right.unwrap().leaf.unwrap(), "d".to_string());
    }

    #[test]
    fn eval_huffman() {
        let node = hash_to_huffman_tree(count_occ(&String::from("bcaadddccacacac")));

        let hash = eval_huffman_tree(node);

        assert_eq!(hash.get(&"a".to_string()), Some(&String::from("11")));
        assert_eq!(hash.get(&"b".to_string()), Some(&String::from("100")));
        assert_eq!(hash.get(&"c".to_string()), Some(&String::from("0")));
        assert_eq!(hash.get(&"d".to_string()), Some(&String::from("101")));
    }
}
