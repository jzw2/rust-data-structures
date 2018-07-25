pub struct BinarySearchTree<T> {
    data: Vec<Node<T>>,

    root: Option<usize>,
}

struct Node<T> {
    key: usize,
    value: T,
    left: Option<usize>,
    right: Option<usize>,
}



impl<T> BinarySearchTree<T> {

    fn new() {
        unimplemented!()
    }
    fn find(&self, key: usize) -> Option<&T> {
        unimplemented!()
    }



    fn insert(&mut self, key: usize, value: T) {
        let new_node = Node {key: key, value: value, left: None, right: None};

        self.data.push(new_node);

        let node_index = self.data.len() - 1;



        match self.root {
            Some(root_index) => {
                let mut current_node_index = root_index;

                loop {
                    let current_node = &self.data[current_node_index];
                    if key < current_node.key {
                        match current_node.left {
                            Some(num) => current_node_index = num,
                            None => break,
                        };
                    } else if key > current_node.key {
                        match current_node.right {
                            Some(num) => current_node_index = num,
                            None => break,
                        }
                    } else {
                        panic!();
                    }
                }

                let current_node = &mut self.data[current_node_index];

                if key < current_node.key {
                    current_node.left = Some(node_index);
                } else if key > current_node.key {
                    current_node.right = Some(node_index);
                }
            }
            None => {
                self.root = Some(0);
                return;
            }
        };



    }

    fn delete(&mut self, key: usize) {
        unimplemented!()
    }
}

fn main() {

}
