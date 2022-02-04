use std::ptr;

#[derive(PartialEq,Clone,Debug)]
struct Node {
    value: usize,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
    parent: Box<Option<Node>>,
    colour: bool
}

impl Node {
    fn new(_value: usize) -> Node {
        Node {
            value: _value,
            left: Box::new(None),
            right: Box::new(None),
            parent: Box::new(None),
            colour: true
        }
    }

    

    fn get_uncle(self) -> Box<Option<Node>> {
        if self.parent.is_some() {
            let parent = self.parent.clone().unwrap();
            if !parent.parent.is_none() {
                let grandparent = parent.clone().parent.unwrap();
                if parent == grandparent.clone().left.unwrap() && !grandparent.right.is_none() {
                    return grandparent.right;
                }
                else if self.parent == grandparent.right && !grandparent.left.is_none() {
                    return grandparent.right;
                }
                else {
                    return Box::new(None);
                }
            }
            else {
                return Box::new(None);
            }
        }
        else {
            return Box::new(None);
        }
    }

    fn is_left_child(self) -> bool {
        self.clone().parent.unwrap().left.is_some() && self.clone() == self.clone().parent.unwrap().left.unwrap()
    }

    fn is_right_child(self) -> bool {
        self.clone().parent.unwrap().right.is_some() && self.clone() == self.clone().parent.unwrap().right.unwrap()
    }
}

#[derive(Clone)]
struct Tree {
    root: Box<Option<Node>>
}

impl Tree {
    pub fn new() -> Tree{
        Tree {
            root: Box::new(None)
        }
    }

    pub fn insert(mut self, value: usize) -> Tree {
        let mut node = Node::new(value);
        self = self.clone().insert_node(&mut node, Box::new(None));
        self
    }

    fn fix_tree(mut self, node: &mut Node) {
        if self.root.is_none() {
            self.root = Box::new(Some(node.clone()));
            node.colour = false; 
        }
        else {
            let uncle = node.clone().get_uncle();
            if uncle.is_none() || !uncle.clone().unwrap().colour {
                let nl_child = node.clone().is_left_child();
                if nl_child == node.parent.clone().unwrap().is_left_child() {
                    if nl_child {
                        self.right_rotate(node.parent.clone().unwrap());
                    }
                    else {
                        self.left_rotate(node.parent.clone().unwrap());
                    }
                }
                else {
                    {
                        node.parent.clone().unwrap().colour = !node.clone().parent.unwrap().colour;
                        node.parent.clone().unwrap().parent.unwrap().colour = !node.clone().parent.unwrap().parent.unwrap().colour;
                        if nl_child {
                            self.right_rotate(node.parent.clone().unwrap().parent.unwrap());
                        }
                        else {
                            self.left_rotate(node.parent.clone().unwrap().parent.unwrap());
                        }
                    }
                }
            }
            else {
                uncle.unwrap().colour = false;
                node.parent.clone().unwrap().colour = false;
                node.parent.clone().unwrap().parent.unwrap().colour = true;
            }
        }
    }

    fn left_rotate(mut self, mut node: Node) {
        if node.right.is_none() {return;}
        let mut temp = node.right.unwrap();
        node.right = temp.clone().left;
        if !temp.clone().left.is_none() {
            temp.clone().left.unwrap().parent = temp.clone().parent;
        }
        temp.parent = node.clone().parent;
        if temp.clone().parent.is_none() {
            self.root = Box::new(Some(temp.clone()));
        }
        else if !node.clone().parent.unwrap().left.is_none() && node.clone().is_left_child() {
            node.clone().parent.unwrap().left = Box::new(Some(temp.clone()));
        }
        else {
            node.clone().parent.unwrap().right = Box::new(Some(temp.clone()));
        }
        temp.left = Box::new(Some(node.clone()));
        node.parent = Box::new(Some(temp));
    }

    fn right_rotate(mut self, mut node: Node) {
        if node.left.is_none() {return;}
        let mut temp = node.left.unwrap();
        node.left = temp.clone().right;
        if !temp.clone().right.is_none() {
            temp.clone().right.unwrap().parent = temp.clone().parent;
        }
        temp.parent = node.clone().parent;
        if temp.clone().parent.is_none() {
            self.root = Box::new(Some(temp.clone()));
        }
        else if !node.clone().parent.unwrap().left.is_none() && node.clone().is_left_child() {
            node.clone().parent.unwrap().left = Box::new(Some(temp.clone()));
        }
        else {
            node.clone().parent.unwrap().right = Box::new(Some(temp.clone()));
        }
        temp.right = Box::new(Some(node.clone()));
        node.parent = Box::new(Some(temp));
    }

    fn insert_node(mut self, inserting: &mut Node, mut current: Box<Option<Node>>) -> Tree {
        if self.root.is_none() {
            println!("inserting root");
            self.root = Box::new(Some(inserting.clone()));
            println!("{:?}",self.root);
            return self;
        }
        else if current.clone().is_none() {
            current = self.root.clone();
        }
        if inserting.value >= current.clone().unwrap().value {
            println!("bigger than {:?}", current.clone());
            if current.clone().unwrap().right.is_none() {
                println!("inserting bigger");
                current.clone().unwrap().right = Box::new(Some(inserting.clone()));
                println!("{:?} {}",current.clone().unwrap().right, current.clone() == current);
                //inserting.parent = current.clone();
                self
            }
            else {
                self.insert_node(inserting, current.unwrap().right)
            }
        }
        else {
            if current.clone().unwrap().left.is_none() {
                current.clone().unwrap().left = Box::new(Some(inserting.clone()));
                inserting.parent = current.clone();
                self
            }
            else {
                self.insert_node(inserting, current.unwrap().left)
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn create_tree() {
        let mut tree = Tree::new();
        assert_eq!(tree.root.is_none(), true);
        tree = tree.clone().insert(10);
        assert_ne!(tree.root.is_none(), true);
        tree = tree.clone().insert(15);
        assert_ne!(tree.root.clone().unwrap().right.is_none(), true);
    }
}
