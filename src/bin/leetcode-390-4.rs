struct Solution;

use std::collections::HashMap;
#[derive(Clone)]
struct Trie {
    ends: bool,
    closest_index: i32,
    sons: Vec<Option<Box<Trie>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            ends: false,
            closest_index: -1,
            sons: vec![None; 26],
        }
    }

    fn insert(&mut self, word: String, index: usize, words: &Vec<String>) {
        let mut curr_node = self;
        if curr_node.closest_index == -1
            || words[curr_node.closest_index as usize].len() > word.len()
        {
            curr_node.closest_index = index as i32;
        }
        for ch in word.chars() {
            let idx = (ch as u8 - 'a' as u8) as usize;
            curr_node = curr_node.sons[idx].get_or_insert(Box::new(Trie::new()));
            if curr_node.closest_index == -1
                || words[curr_node.closest_index as usize].len() > word.len()
            {
                curr_node.closest_index = index as i32;
            }
        }
        curr_node.ends = true;
    }

    fn search(&self, word: String) -> i32 {
        let mut curr_node = self;
        for ch in word.chars() {
            let index = (ch as u8 - 'a' as u8) as usize;
            if let Some(nxt_node) = curr_node.sons[index].as_deref() {
                curr_node = nxt_node;
            } else {
                break;
            }
        }
        curr_node.closest_index
    }
}

impl Solution {
    pub fn string_indices_2(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut ans = vec![];
        let mut root = Trie::new();
        let words = words_container
            .into_iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>();

        for index in 0..words.len() {
            root.insert(words[index].clone(), index, &words);
        }

        let queries = words_query
            .into_iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>();

        for q in queries {
            ans.push(root.search(q));
        }
        ans
    }

    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut ans = vec![];
        let mut map = HashMap::new();
        let mut words = words_container
            .iter()
            .enumerate()
            .map(|(idx, s)| {
                let new_s = s.chars().rev().collect::<String>();
                (new_s, idx)
            })
            .collect::<Vec<_>>();

        words.sort();
        for (word, idx) in &words {
            for end in 0..word.len() {
                let e = map.entry(&word[0..=end]).or_insert(*idx);
                if *e != *idx {
                    let prev = &words_container[*e];
                    let curr = &words_container[*idx];
                    if prev.len() > curr.len() {
                        *e = *idx;
                    } else if prev.len() == curr.len() {
                        if *e > *idx {
                            *e = *idx;
                        }
                    }
                }
            }
        }

        for q in words_query {
            let rev_q = q.chars().rev().collect::<String>();
            for end in (0..rev_q.len()).rev() {
                let prefix = &rev_q[0..=end];
                if let Some(v) = map.get(prefix) {
                    ans.push(*v as i32);
                    break;
                }
            }
        }
        ans
    }
}

fn main() {}
