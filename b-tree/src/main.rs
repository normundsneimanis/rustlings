use std::fmt::{Debug};

type Node = Option<Box<NodeElem>>;

#[derive(Debug)]
struct NodeElem {
    data: i32,
    left: Node,
    right: Node,
}

trait BinaryTree {
    fn new(data: i32) -> Self;
    fn member(&self, data: i32) -> bool;
    fn add(&mut self, data: i32);
    fn delete_min(&mut self) -> i32;
    fn delete(&mut self, data: i32);
    fn walk_preordial(&self, vec: &mut Vec<i32>);
}

impl BinaryTree for Node {
    fn new(data: i32) -> Self {
        return Some(Box::new(NodeElem {
            data,
            left: None,
            right: None
        }))
    }

    fn member(&self, data: i32) -> bool {
        return if self.is_some() {
            self.as_ref().map(|d| {
                if d.data == data {
                    true
                } else if data > d.data {
                    d.right.member(data)
                } else {
                    d.left.member(data)
                }
            }).expect("must have output")
        } else { false }
    }

    fn add(&mut self, data: i32) {
        if self.is_some() {
            self.as_mut().map(|d| {
                if d.data == data {

                } else if data > d.data {
                    d.right.add(data)
                } else {
                    d.left.add(data)
                }
            });
        } else {
            *self = Self::new(data);
        }
    }

    fn delete_min(&mut self) -> i32 {
        enum DoWhat {
            Nothing,
            Right
        }
        let mut elem = None;

        let mut action = DoWhat::Nothing;

        self.as_mut().map(|x| {
            if x.left.is_none() {
                elem = Some(x.data);
                action = DoWhat::Right;
            } else {
                elem = Some(x.left.delete_min());
            }
        });

        match action {
            DoWhat::Nothing => {}
            DoWhat::Right => {
                let right = self.as_deref_mut().unwrap().right.take();
                *self = right;
            }
        }
        elem.unwrap()
    }

    fn delete(&mut self, data: i32) {
        enum DoWhat {
            Nothing,
            NullSelf,
            Right,
            Left
        }
        let mut delete_self = DoWhat::Nothing;
        self.as_mut().map(|x| {
            if data < x.data {
                x.left.delete(data)
            } else if data > x.data {
                x.right.delete(data)
            } else if x.left.is_none() && x.right.is_none() {
                delete_self = DoWhat::NullSelf;
            } else if x.left.is_none() {
                delete_self = DoWhat::Right;
            } else if x.right.is_none() {
                delete_self = DoWhat::Left;
            } else {
                x.data = Self::delete_min(&mut x.right)
            }
        });
        match delete_self {
            DoWhat::Nothing => {}
            DoWhat::NullSelf => {
                *self = None
            }
            DoWhat::Right => {
                let i = self.as_deref_mut().unwrap().right.take();
                *self = i;
            }
            DoWhat::Left => {
                let i = self.as_deref_mut().unwrap().left.take();
                *self = i;
            }
        }
    }
    fn walk_preordial(&self, vec: &mut Vec<i32>) {
        self.as_ref().map(|a| {
            a.left.walk_preordial(vec);
            vec.push(a.data.clone());
            a.right.walk_preordial(vec);
        });
    }
}

fn main() {
    let mut tree = <Option<Box<NodeElem>> as BinaryTree>::new(199);
    tree.add(100);
    tree.add(205);
    tree.add(99);
    assert_eq!(true, tree.member(205));
    assert_eq!(true, tree.member(100));
    assert_eq!(true, tree.member(99));
    assert_eq!(false, tree.member(-10));
    let mut vec = vec![];
    tree.walk_preordial(&mut vec);
    assert_eq!(vec![99, 100, 199, 205], vec);
    tree.delete(100);
    assert_eq!(false, tree.member(100));
    vec.clear();
    tree.walk_preordial(&mut vec);
    assert_eq!(vec![99, 199, 205], vec);
}

#[cfg(test)]
mod test {
    use rand::Rng;
    use crate::{BinaryTree, NodeElem};

    #[test]
    fn delete_leaf() {
        let mut tree = <Option<Box<NodeElem>> as BinaryTree>::new(199);
        tree.add(100);
        tree.add(205);
        tree.add(99);
        assert_eq!(true, tree.member(205));
        assert_eq!(true, tree.member(100));
        assert_eq!(true, tree.member(99));
        assert_eq!(false, tree.member(-10));
        let mut vec = vec![];
        tree.walk_preordial(&mut vec);
        assert_eq!(vec![99, 100, 199, 205], vec);
        tree.delete(100);
        assert_eq!(false, tree.member(100));
        vec.clear();
        tree.walk_preordial(&mut vec);
        assert_eq!(vec![99, 199, 205], vec);
    }

    #[test]
    fn delete_one_child() {
        let mut tree = <Option<Box<NodeElem>> as BinaryTree>::new(199);
        tree.add(100);
        tree.add(205);
        tree.add(99);
        tree.add(98);
        tree.add(97);
        assert_eq!(true, tree.member(205));
        assert_eq!(true, tree.member(100));
        assert_eq!(true, tree.member(99));
        assert_eq!(false, tree.member(-10));
        let mut vec = vec![];
        tree.walk_preordial(&mut vec);
        assert_eq!(vec![97, 98, 99, 100, 199, 205], vec);
        tree.delete(100);
        assert_eq!(false, tree.member(100));
        let mut vec = vec![];
        tree.walk_preordial(&mut vec);
        assert_eq!(vec![97, 98, 99, 199, 205], vec);
    }

    #[test]
    fn delete_both_children() {
        let mut tree = <Option<Box<NodeElem>> as BinaryTree>::new(199);
        tree.add(205);
        tree.add(101);
        tree.add(105);
        tree.add(50);
        tree.add(60);
        tree.add(25);
        tree.add(27);
        let mut vec = vec![];
        tree.walk_preordial(&mut vec);
        assert_eq!(vec![25, 27, 50, 60, 101, 105, 199, 205], vec);
        tree.delete(101);
        vec.clear();
        tree.walk_preordial(&mut vec);
        assert_eq!(vec![25, 27, 50, 60, 105, 199, 205], vec);
    }

    #[test]
    fn delete_leaf_with_child() {
        let mut tree = <Option<Box<NodeElem>> as BinaryTree>::new(199);
        tree.add(99);
        tree.add(98);
        tree.delete(99);
        let mut vec = vec![];
        tree.walk_preordial(&mut vec);
        assert_eq!(vec![98, 199], vec);
    }

    #[test]
    fn some_other_test() {
        let mut tree = <Option<Box<NodeElem>> as BinaryTree>::new(199);
        tree.add(100);
        tree.add(205);
        tree.add(99);
        assert_eq!(true, tree.member(205));
        assert_eq!(true, tree.member(100));
        assert_eq!(true, tree.member(99));
        assert_eq!(false, tree.member(-10));
        let mut vec = vec![];
        tree.walk_preordial(&mut vec);
        assert_eq!(vec![99, 100, 199, 205], vec);
        tree.delete(100);
        assert_eq!(false, tree.member(100));
        vec.clear();
        tree.walk_preordial(&mut vec);
        assert_eq!(vec![99, 199, 205], vec);
    }

    #[test]
    #[ignore]
    fn random() {
        use rand;
        use std::time::Instant;
        let mut rng = rand::rng();
        let mut tree = <Option<Box<NodeElem>> as BinaryTree>::new(rng.random());
        for _ in 0..1000000 {
            tree.add(rng.random())
        }

        let mut vec = vec![];
        tree.walk_preordial(&mut vec);
        let num = vec.remove(rng.random_range(0..1000000));
        let start_time = Instant::now();
        tree.delete(num);
        println!("Removal time {}μs", start_time.elapsed().as_micros());
        let mut vec2 = vec![];
        tree.walk_preordial(&mut vec2);
        assert_eq!(vec, vec2)
    }
}
