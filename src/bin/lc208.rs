#[derive(Clone)]
struct Trie {
    ends: bool,
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
            sons: vec![None; 26],
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr_node = self;
        for ch in word.chars() {
            let index = (ch as u8 - 'a' as u8) as usize;
            curr_node = curr_node.sons[index].get_or_insert(Box::new(Trie::new()));
        }
        curr_node.ends = true;
    }

    fn search(&self, word: String) -> bool {
        let mut curr_node = self;
        for ch in word.chars() {
            let index = (ch as u8 - 'a' as u8) as usize;
            if let Some(nxt_node) = curr_node.sons[index].as_deref() {
                curr_node = nxt_node;
            } else {
                return false;
            }
        }
        curr_node.ends
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr_node = self;
        for ch in prefix.chars() {
            let index = (ch as u8 - 'a' as u8) as usize;
            if let Some(nxt_node) = curr_node.sons[index].as_deref() {
                curr_node = nxt_node;
            } else {
                return false;
            }
        }
        true
    }
}

fn main() {}
