use std::rc::Rc;

#[derive(PartialEq,Clone,Debug)]
struct Node {
    value: usize,
    left: Rc<Option<Node>>,
    right: Rc<Option<Node>>,
    parent: Rc<Option<Node>>,
    colour: bool
}

impl Node {
    fn new(_value: usize) -> Node {
        Node {
            value: _value,
            left: Rc::new(None),
            right: Rc::new(None),
            parent: Rc::new(None),
            colour: true
        }
    }

    

    fn get_uncle(self) -> Rc<Option<Node>> {
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
                    return Rc::new(None);
                }
            }
            else {
                return Rc::new(None);
            }
        }
        else {
            return Rc::new(None);
        }
    }

    fn is_left_child(self) -> bool {
        self.clone().parent.unwrap().left.is_some() && self.clone() == self.clone().parent.unwrap().left.unwrap()
    }

    fn is_right_child(self) -> bool {
        self.clone().parent.unwrap().right.is_some() && self.clone() == self.clone().parent.unwrap().right.unwrap()
    }
}

#[derive(Clone,Debug)]
struct Tree {
    root: Rc<Option<Node>>
}

impl Tree {
    pub fn new() -> Tree{
        Tree {
            root: Rc::new(None)
        }
    }

    pub fn insert(mut self, value: usize) -> Tree {
        let mut node = Node::new(value);
        self = self.clone().insert_node(&mut node, Rc::new(None));
        println!("{:?}", self);
        self
    }

    fn fix_tree(mut self, node: &mut Node) {
        if self.root.is_none() {
            self.root = Rc::new(Some(node.clone()));
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
            self.root = Rc::new(Some(temp.clone()));
        }
        else if !node.clone().parent.unwrap().left.is_none() && node.clone().is_left_child() {
            node.clone().parent.unwrap().left = Rc::new(Some(temp.clone()));
        }
        else {
            node.clone().parent.unwrap().right = Rc::new(Some(temp.clone()));
        }
        temp.left = Rc::new(Some(node.clone()));
        node.parent = Rc::new(Some(temp));
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
            self.root = Rc::new(Some(temp.clone()));
        }
        else if !node.clone().parent.unwrap().left.is_none() && node.clone().is_left_child() {
            node.clone().parent.unwrap().left = Rc::new(Some(temp.clone()));
        }
        else {
            node.clone().parent.unwrap().right = Rc::new(Some(temp.clone()));
        }
        temp.right = Rc::new(Some(node.clone()));
        node.parent = Rc::new(Some(temp));
    }

    fn insert_node(mut self, inserting: &mut Node, mut current: Rc<Option<Node>>) -> Tree {
        if self.root.is_none() {
            self.root = Rc::new(Some(inserting.clone()));
            return self;
        }
        else if current.clone().is_none() {
            current = self.root.clone();
        }
        if inserting.value >= current.clone().unwrap().value {
            if current.clone().take().unwrap().right.is_none() {
                let mut curclo = current.clone().unwrap();
                curclo.right = Rc::new(Some(inserting.clone()));
                current = Rc::new(Some(curclo));
                if current.clone().unwrap().is_left_child() {

                }
                inserting.parent = current.clone();
                self
            }
            else {
                self.insert_node(inserting, current.unwrap().right)
            }
        }
        else {
            if current.clone().unwrap().left.is_none() {
                current.clone().unwrap().left = Rc::new(Some(inserting.clone()));
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
