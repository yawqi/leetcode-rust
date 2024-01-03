use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
fn main() {
    let mut cache =  GenericLRUCache::new(2, -1);
    cache.put(1, 1);
    cache.put(2, 2);
    let v1 = cache.get(1);
    cache.put(3, 3);
    let v2 = cache.get(1);
    let v3 = cache.get(2);
    println!("{v1} {v2} {v3}");
}

struct Node<K: Hash + Default + PartialEq + Eq + Clone, V: Default + Clone> {
    pub key: K,
    pub val: V,
    pub next: Option<Rc<RefCell<Node<K, V>>>>,
    pub prev: Option<Rc<RefCell<Node<K, V>>>>
}

impl<K: Hash + Default + PartialEq + Eq + Clone, V: Default + Clone> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Node {
            key,
            val,
            next: None,
            prev: None,
        }
    }
}

struct GenericLRUCache<K: Hash + Default + Eq + PartialEq + Clone, V: Default + Clone> {
    capacity: i32,
    head: Rc<RefCell<Node<K, V>>>,
    tail: Rc<RefCell<Node<K, V>>>,
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    invalid: V,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl<K: Hash + Default + Eq + PartialEq + Clone, V: Default + Clone> GenericLRUCache<K, V> {
    fn new(capacity: i32, invalid: V) -> Self {
        let head = Rc::new(RefCell::new(Node::new(K::default(), V::default())));
        let tail = Rc::new(RefCell::new(Node::new(K::default(), V::default())));
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));
        GenericLRUCache {
            capacity,
            head,
            tail,
            map: HashMap::new(),
            invalid,
        }
    }

    fn get(&self, key: K) -> V {
        if let Some(node) = self.map.get(&key) {
            self.remove_node(Rc::clone(node));
            self.insert_head(Rc::clone(node));
            node.borrow().val.clone()
        } else {
            self.invalid.clone()
        }
    }

    fn put(&mut self, key: K, value: V) {
        let node = if let Some(node) = self.map.get(&key) {
            node.borrow_mut().val = value;
            self.remove_node(node.clone());
            Rc::clone(node)
        } else {
            Rc::new(RefCell::new(Node::new(key.clone(), value)))
        };
        self.map.remove(&key);
        if self.map.len() == self.capacity as usize {
            self.map.remove(&self.tail.borrow().prev.as_ref().unwrap().borrow().key);
            let tmp = Rc::clone(self.tail.borrow().prev.as_ref().unwrap());
            self.remove_node(tmp);
        }
        self.map.insert(key, Rc::clone(&node));
        self.insert_head(node);
    }

    fn remove_node(&self, node: Rc<RefCell<Node<K, V>>>) {
        let prev = node.borrow_mut().prev.take().unwrap().clone();
        let next = node.borrow_mut().next.take().unwrap().clone();
        next.borrow_mut().prev = Some(Rc::clone(&prev));
        prev.borrow_mut().next = Some(Rc::clone(&next));
    }

    fn insert_head(&self, node: Rc<RefCell<Node<K, V>>>) {
        let prev = &self.head;
        let next = prev.borrow_mut().next.take().unwrap();
        node.borrow_mut().prev = Some(Rc::clone(prev));
        node.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::clone(&node));
        prev.borrow_mut().next = Some(node);
    }
}