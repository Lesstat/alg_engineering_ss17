pub type NodeId = usize;


pub trait Edge {
    fn get_source_id(&self) -> NodeId;
    fn get_dest_id(&self) -> NodeId;
}

#[derive(PartialEq,Debug)]
struct EdgeTest {
    source: NodeId,
    dest: NodeId,
}

impl Edge for EdgeTest {
    fn get_source_id(&self) -> NodeId {
        self.source
    }
    fn get_dest_id(&self) -> NodeId {
        self.dest
    }
}
impl EdgeTest {
    fn new(source: NodeId, dest: NodeId) -> EdgeTest {
        EdgeTest {
            source: source,
            dest: dest,
        }
    }
}
#[derive(Clone,PartialEq,Debug)]
struct NodeOffset(usize);

pub struct Graph<E: Edge> {
    nodes: Vec<NodeOffset>,
    edges: Vec<E>,
}

impl<E: Edge> Graph<E> {
    pub fn new(node_count: usize, mut edges: Vec<E>) -> Graph<E> {
        edges.sort_by(|a, b| {
                          let ord = a.get_source_id().cmp(&b.get_source_id());
                          match ord {
                              ::std::cmp::Ordering::Equal => a.get_dest_id().cmp(&b.get_dest_id()),
                              _ => ord,
                          }
                      });

        Graph {
            nodes: Graph::calc_node_offsets(node_count, &edges),
            edges: edges,
        }

    }

    pub fn outgoing_edges_for(&self, id: NodeId) -> &[E] {
        &self.edges[self.nodes[id].0..self.nodes[id + 1].0]
    }

    fn calc_node_offsets(node_count: usize, edges: &Vec<E>) -> Vec<NodeOffset> {

        let mut nodes = vec![NodeOffset(0); node_count +1];
        let mut last_source = 0;
        for (index, edge) in edges.iter().enumerate() {
            let cur_source = edge.get_source_id();
            for id in last_source..cur_source {
                nodes[id + 1].0 = index;
            }
            last_source = cur_source;
        }


        for id in (last_source + 1)..(node_count + 1) {
            nodes[id].0 = edges.len();
        }
        nodes

    }
}

#[test]
fn graph_creation() {
    let g = Graph::new(5,
                       vec![EdgeTest::new(0, 1),
                            EdgeTest::new(0, 2),
                            EdgeTest::new(2, 3),
                            EdgeTest::new(0, 3),
                            EdgeTest::new(2, 4)]);
    let exp = vec![NodeOffset(0),
                   NodeOffset(3),
                   NodeOffset(3),
                   NodeOffset(5),
                   NodeOffset(5),
                   NodeOffset(5)];
    assert_eq!(g.nodes.len(), exp.len());
    assert_eq!(g.nodes, exp);

    assert_eq!(g.outgoing_edges_for(0).len(), 3);
    assert_eq!(g.outgoing_edges_for(2),
               &[EdgeTest::new(2, 3), EdgeTest::new(2, 4)]);
}
