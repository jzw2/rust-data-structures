pub struct BinarySearchTree<T> {
    data: Vec<Node<T>>,
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
        unimplemented!()
    }

    fn delete(&mut self, key: usize) {
        unimplemented!()
    }
}

fn main() {

}
