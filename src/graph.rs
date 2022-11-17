use std::collections::HashSet;

#[allow(dead_code)]
struct Graph<Data> {
    arena: Vec<Node<Data>>,
}

#[allow(dead_code)]
struct Node<Data> {
    id: usize,
    edges: HashSet<usize>,
    data: Data,
}

#[allow(dead_code)]
impl<Data> Node<Data> {
    fn new(id: usize, data: Data) -> Self {
        let edges = HashSet::new();
        Self { id, edges, data }
    }

    fn add_connection(&mut self, to: usize) {
        self.edges.insert(to);
    }
}
