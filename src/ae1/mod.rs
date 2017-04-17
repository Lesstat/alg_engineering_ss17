mod load;

pub use self::load::load_graph;

pub type NodeId = usize;
pub type OsmNodeId = usize;
pub type Latitude = f64;
pub type Longitude = f64;
pub type Length = usize;
pub type Speed = usize;
pub type Height = usize;

#[derive(HeapSizeOf)]
pub struct NodeInfo {
    osm_id: OsmNodeId,
    lat: Latitude,
    long: Longitude,
    height: Height,
}

impl NodeInfo {
    fn new(osm_id: OsmNodeId, lat: Latitude, long: Longitude, height: Height) -> NodeInfo {
        NodeInfo {
            osm_id: osm_id,
            lat: lat,
            long: long,
            height: height,
        }
    }
}

pub trait Edge {
    fn get_source_id(&self) -> NodeId;
    fn get_dest_id(&self) -> NodeId;
    fn get_travel_time(&self) -> f64;
}

#[derive(PartialEq,Debug,HeapSizeOf)]
pub struct EdgeInfo {
    source: NodeId,
    dest: NodeId,
    length: Length,
    speed: Speed,
}

impl Edge for EdgeInfo {
    fn get_source_id(&self) -> NodeId {
        self.source
    }
    fn get_dest_id(&self) -> NodeId {
        self.dest
    }
    fn get_travel_time(&self) -> f64 {
        (self.length as f64) / (self.speed as f64)
    }
}
impl EdgeInfo {
    fn new(source: NodeId, dest: NodeId, length: Length, speed: Speed) -> EdgeInfo {
        EdgeInfo {
            source: source,
            dest: dest,
            length: length,
            speed: speed,
        }
    }
}
#[derive(Clone,PartialEq,Debug,HeapSizeOf)]
struct NodeOffset(usize);

#[derive(HeapSizeOf)]
pub struct Graph<E: Edge> {
    node_info: Vec<NodeInfo>,
    node_offsets: Vec<NodeOffset>,
    edges: Vec<E>,
}

impl<E: Edge> Graph<E> {
    pub fn new(node_info: Vec<NodeInfo>, mut edges: Vec<E>) -> Graph<E> {
        use std::cmp::Ordering;
        edges.sort_by(|a, b| {
                          let ord = a.get_source_id().cmp(&b.get_source_id());
                          match ord {
                              Ordering::Equal => a.get_dest_id().cmp(&b.get_dest_id()),
                              _ => ord,
                          }
                      });

        let node_count = node_info.len();
        Graph {
            node_info: node_info,
            node_offsets: Graph::calc_node_offsets(node_count, &edges),
            edges: edges,
        }

    }

    pub fn outgoing_edges_for(&self, id: NodeId) -> &[E] {
        &self.edges[self.node_offsets[id].0..self.node_offsets[id + 1].0]
    }

    fn calc_node_offsets(node_count: usize, edges: &[E]) -> Vec<NodeOffset> {

        let mut node_offsets = vec![NodeOffset(0); node_count +1];
        let mut last_source = 0;
        for (index, edge) in edges.iter().enumerate() {
            let cur_source = edge.get_source_id();
            for node_offset in &mut node_offsets[last_source + 1..cur_source + 1] {
                node_offset.0 = index;
            }
            last_source = cur_source;
        }

        for node_offset in &mut node_offsets[last_source + 1..node_count + 1] {
            node_offset.0 = edges.len();
        }
        node_offsets

    }
}

#[test]
fn graph_creation() {
    let g = Graph::new(vec![NodeInfo::new(23, 3.4, 2.3, 12),
                            NodeInfo::new(27, 4.4, 2.3, 12),
                            NodeInfo::new(53, 6.4, 1.3, 12),
                            NodeInfo::new(36, 3.8, 2.4, 12),
                            NodeInfo::new(78, 9.2, 2.3, 12)],
                       vec![EdgeInfo::new(0, 1, 1, 1),
                            EdgeInfo::new(0, 2, 1, 1),
                            EdgeInfo::new(2, 3, 1, 1),
                            EdgeInfo::new(0, 3, 1, 1),
                            EdgeInfo::new(2, 4, 1, 1)]);
    let exp = vec![NodeOffset(0),
                   NodeOffset(3),
                   NodeOffset(3),
                   NodeOffset(5),
                   NodeOffset(5),
                   NodeOffset(5)];
    assert_eq!(g.node_offsets.len(), exp.len());
    assert_eq!(g.node_offsets, exp);

    assert_eq!(g.outgoing_edges_for(0).len(), 3);
    assert_eq!(g.outgoing_edges_for(2),
               &[EdgeInfo::new(2, 3, 1, 1), EdgeInfo::new(2, 4, 1, 1)]);
}
