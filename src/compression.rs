use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

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

    pub fn new_leaf(c: char) -> Self {
        Node {
            leaf: Some(c.to_string()),
            left: None,
            right: None,
        }
    }
}

fn count_occ(s: &String) -> HashMap<char, i32> {
    let mut hash: HashMap<char, i32> = HashMap::new();

    s.chars().for_each(|c| {
        match hash.get_mut(&c) {
            Some(v) => {
                *v += 1;
            }
            None => {
                hash.insert(c, 1);
            }
        };
    });

    hash
}

fn hash_to_huffman_tree(hash: HashMap<char, i32>) -> Node {
    let mut heap = BinaryHeap::new();
    hash.iter()
        .for_each(|(c, p)| heap.push(PriorityElement::new(*p, Node::new_leaf(*c))));

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
    let mut v = Vec::new();
    parcours_huffman_tree(&mut v, node, "");
    let mut hash = HashMap::new();
    v.iter().for_each(|(c, r)| {
        hash.insert(c.to_string(), r.to_string());
    });
    hash
}

pub fn display(init: &String, result: HashMap<String, String>) -> String {
    let code = result
        .iter()
        .fold(String::new(), |acc, (v, k)| format!("{acc}{k}={v}\n"));

    let compressed = init
        .chars()
        .map(|c| result.get(&c.to_string()).unwrap())
        .fold(String::new(), |acc, repr| format!("{acc}{repr}"));

    format!("{code}\n{compressed}")
}

pub fn compress_word(word: String) -> String {
    let node = hash_to_huffman_tree(count_occ(&word));
    let hash = eval_huffman_tree(node);
    display(&word, hash)
}

#[cfg(test)]
mod test {
    use super::{count_occ, eval_huffman_tree, hash_to_huffman_tree};

    #[test]
    fn count() {
        let hash = count_occ(&String::from("bcaadddccacacac"));
        assert_eq!(hash.get(&'a'), Some(&5));
        assert_eq!(hash.get(&'b'), Some(&1));
        assert_eq!(hash.get(&'c'), Some(&6));
        assert_eq!(hash.get(&'d'), Some(&3));

        assert_eq!(hash.get(&'z'), None);
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
