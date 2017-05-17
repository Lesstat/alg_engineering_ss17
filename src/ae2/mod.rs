mod load;
mod alg;

pub use self::load::load_graph;

pub type NodeId = usize;
pub type EdgeId = usize;
pub type OsmNodeId = usize;
pub type Latitude = f64;
pub type Longitude = f64;
pub type Length = usize;
pub type Speed = usize;
pub type Height = usize;
pub type Level = usize;

#[derive(HeapSizeOf)]
pub struct ChNodeInfo {
    id: NodeId,
    osm_id: OsmNodeId,
    lat: Latitude,
    long: Longitude,
    height: Height,
    level: Level,
}

impl ChNodeInfo {
    fn new(id: NodeId,
           osm_id: OsmNodeId,
           lat: Latitude,
           long: Longitude,
           height: Height,
           level: Level)
           -> ChNodeInfo {
        ChNodeInfo {
            id,
            osm_id,
            lat,
            long,
            height,
            level,
        }
    }
}

#[derive(PartialEq,Debug,HeapSizeOf)]
pub struct ChEdgeInfo {
    source: NodeId,
    dest: NodeId,
    length: Length,
    speed: Speed,
    edge_a: Option<EdgeId>,
    edge_b: Option<EdgeId>,
}

impl ChEdgeInfo {
    fn new(source: NodeId,
           dest: NodeId,
           length: Length,
           speed: Speed,
           edge_a: Option<EdgeId>,
           edge_b: Option<EdgeId>)
           -> ChEdgeInfo {
        ChEdgeInfo {
            source: source,
            dest: dest,
            length: length,
            speed: speed,
            edge_a: edge_a,
            edge_b: edge_b,
        }
    }
}

#[derive(HeapSizeOf,Debug, Eq, PartialEq)]
pub struct HalfEdge {
    endpoint: NodeId,
    weight: Length,
}



#[derive(Clone,PartialEq,Debug,HeapSizeOf)]
struct NodeOffset {
    in_start: usize,
    out_start: usize,
}
impl NodeOffset {
    pub fn new(in_start: usize, out_start: usize) -> NodeOffset {
        NodeOffset {
            in_start: in_start,
            out_start: out_start,
        }
    }
}

#[derive(HeapSizeOf)]
pub struct ChGraph {
    node_info: Vec<ChNodeInfo>,
    node_offsets: Vec<NodeOffset>,
    out_edges: Vec<HalfEdge>,
    in_edges: Vec<HalfEdge>,
    level: Vec<Level>,
}

enum OffsetMode {
    In,
    Out,
}
impl ChGraph {
    pub fn new(mut node_info: Vec<ChNodeInfo>, mut edges: Vec<ChEdgeInfo>) -> ChGraph {
        //node_info.sort_by(|a, b| a.level.cmp(&b.level));
        //ChGraph::map_node_id_to_edges(&node_info, &mut edges);
        let level = node_info.iter().map(|n| n.level).collect();


        let node_count = node_info.len();
        let (node_offset, in_edges, out_edges) = ChGraph::calc_node_offsets(node_count, edges);
        ChGraph {
            node_info: node_info,
            node_offsets: node_offset,
            out_edges: out_edges,
            in_edges: in_edges,
            level: level,
        }

    }
    fn map_node_id_to_edges(node_info: &Vec<ChNodeInfo>, edges: &mut Vec<ChEdgeInfo>) {
        use std::collections::BTreeMap;

        let mut mapping = BTreeMap::new();
        for (i, n) in node_info.iter().enumerate() {
            mapping.insert(n.id, i);
        }
        for e in edges.iter_mut() {
            e.source = *mapping.get(&e.source).expect("id must be present");
            e.dest = *mapping.get(&e.dest).expect("id must be present");
        }

    }

    pub fn outgoing_edges_for(&self, id: NodeId) -> &[HalfEdge] {
        &self.out_edges[self.node_offsets[id].out_start..self.node_offsets[id + 1].out_start]
    }

    pub fn ingoing_edges_for(&self, id: NodeId) -> &[HalfEdge] {
        &self.in_edges[self.node_offsets[id].in_start..self.node_offsets[id + 1].in_start]
    }

    fn calc_node_offsets(node_count: usize,
                         mut edges: Vec<ChEdgeInfo>)
                         -> (Vec<NodeOffset>, Vec<HalfEdge>, Vec<HalfEdge>) {
        use std::cmp::Ordering;

        fn calc_offset_inner(edges: &Vec<ChEdgeInfo>,
                             node_offsets: &mut Vec<NodeOffset>,
                             mode: OffsetMode) {

            let mut last_id = 0;
            for (index, edge) in edges.iter().enumerate() {

                let cur_id = match mode {
                    OffsetMode::In => edge.dest,
                    OffsetMode::Out => edge.source,
                };
                for node_offset in &mut node_offsets[last_id + 1..cur_id + 1] {
                    match mode {
                        OffsetMode::In => {
                            node_offset.in_start = index;
                        }
                        OffsetMode::Out => {
                            node_offset.out_start = index;
                        }
                    }

                }
                last_id = cur_id;
            }

            for node_offset in &mut node_offsets[last_id + 1..] {
                match mode {
                    OffsetMode::In => {
                        node_offset.in_start = edges.len();
                    }
                    OffsetMode::Out => {
                        node_offset.out_start = edges.len();
                    }
                }
            }
        }

        let mut node_offsets = vec![NodeOffset::new(0,0); node_count +1];

        edges.sort_by(|a, b| {
                          let ord = a.dest.cmp(&b.dest);
                          match ord {
                              Ordering::Equal => a.source.cmp(&b.source),
                              _ => ord,
                          }
                      });
        calc_offset_inner(&edges, &mut node_offsets, OffsetMode::In);
        let in_edges = ChGraph::create_half_edges(&edges, OffsetMode::In);


        edges.sort_by(|a, b| {
                          let ord = a.source.cmp(&b.source);
                          match ord {
                              Ordering::Equal => a.dest.cmp(&b.dest),
                              _ => ord,
                          }
                      });
        calc_offset_inner(&edges, &mut node_offsets, OffsetMode::Out);
        let out_edges = ChGraph::create_half_edges(&edges, OffsetMode::Out);

        (node_offsets, in_edges, out_edges)
    }
    fn create_half_edges(edges: &Vec<ChEdgeInfo>, mode: OffsetMode) -> Vec<HalfEdge> {
        match mode {

            OffsetMode::In => {
                edges
                    .iter()
                    .map(|e| {
                             HalfEdge {
                                 endpoint: e.source,
                                 weight: e.length,
                             }
                         })
                    .collect()
            }

            OffsetMode::Out => {
                edges
                    .iter()
                    .map(|e| {
                             HalfEdge {
                                 endpoint: e.dest,
                                 weight: e.length,
                             }
                         })
                    .collect()
            }
        }

    }

    pub fn node_count(&self) -> usize {
        self.node_offsets.len()
    }
}
