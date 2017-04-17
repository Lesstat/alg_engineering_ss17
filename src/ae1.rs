pub use self::load::load_graph;

pub type NodeId = usize;
pub type OsmNodeId = usize;
pub type Latitude = f64;
pub type Longitude = f64;
pub type Length = usize;
pub type Speed = usize;
pub type Height = usize;

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

#[derive(PartialEq,Debug)]
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
#[derive(Clone,PartialEq,Debug)]
struct NodeOffset(usize);

pub struct Graph<E: Edge> {
    node_info: Vec<NodeInfo>,
    node_offsets: Vec<NodeOffset>,
    edges: Vec<E>,
}

impl<E: Edge> Graph<E> {
    pub fn new(node_info: Vec<NodeInfo>, mut edges: Vec<E>) -> Graph<E> {
        edges.sort_by(|a, b| {
                          let ord = a.get_source_id().cmp(&b.get_source_id());
                          match ord {
                              ::std::cmp::Ordering::Equal => a.get_dest_id().cmp(&b.get_dest_id()),
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

mod load {

    use ae1::*;

    use std::path::Path;
    use std::fs::File;
    use std::io::Read;

    fn load_file<P: AsRef<Path>>(file: P) -> (Vec<NodeInfo>, Vec<EdgeInfo>) {
        let mut buffer = String::new();

        let mut file = File::open(file).expect("File could not be opened");
        file.read_to_string(&mut buffer)
            .expect("Could not read file");
        let mut line_iter = buffer
            .lines()
            .skip_while(|l| l.starts_with('#') || l.is_empty());
        let node_count: usize = line_iter
            .next()
            .map(str::parse)
            .expect("did not find node count")
            .expect("Node Count could not be parsed");
        let edge_count: usize = line_iter
            .next()
            .map(str::parse)
            .expect("did not find edge count")
            .expect("edge Count could not be parsed");


        let mut nodes = Vec::with_capacity(node_count);
        let mut edges = Vec::with_capacity(edge_count);
        for node_id in 0..node_count {
            let mut raw_node_data = line_iter
                .next()
                .map(|l| l.split(' '))
                .expect(&format!("No more node data for {}", node_id));
            raw_node_data.next(); // id is not necessary
            let osm_id: OsmNodeId = raw_node_data
                .next()
                .map(str::parse)
                .expect("No OSM ID Data")
                .expect("OSM ID not parse-able");
            let lat: Latitude = raw_node_data
                .next()
                .map(str::parse)
                .expect("No Latitude Data")
                .expect("Latitude not parse-able");
            let long: Longitude = raw_node_data
                .next()
                .map(str::parse)
                .expect("No Longitude Data")
                .expect("Longitude not parse-able");
            let height: Height = raw_node_data
                .next()
                .map(str::parse)
                .expect("No Height Data")
                .expect("Height not parse-able");
            nodes.push(NodeInfo::new(osm_id, lat, long, height))

        }
        for edge_id in 0..edge_count {
            let mut raw_node_data = line_iter
                .next()
                .map(|l| l.split(' '))
                .expect(&format!("No more node data for {}", edge_id));
            let source: NodeId = raw_node_data
                .next()
                .map(str::parse)
                .expect("No source Id found")
                .expect("Source id not parse-able");
            let dest: NodeId = raw_node_data
                .next()
                .map(str::parse)
                .expect("No destination Id found")
                .expect("Destination id not parse-able");
            let length: Length = raw_node_data
                .next()
                .map(str::parse)
                .expect("No length found")
                .expect("Length id not parse-able");
            raw_node_data.next(); //ignore type

            let speed: Speed = raw_node_data
                .next()
                .map(str::parse)
                .expect("No speed found")
                .expect("Speed id not parse-able");
            edges.push(EdgeInfo::new(source, dest, length, speed))

        }

        assert_eq!(None, line_iter.next(), "file still contains data");

        (nodes, edges)
    }
    pub fn load_graph<P: AsRef<Path>>(file: P) -> Graph<EdgeInfo> {
        use std::time::Instant;
        let start = Instant::now();
        let (nodes, edges) = load_file(file);
        let file_loaded = Instant::now();
        let g = Graph::new(nodes, edges);
        let graph_created = Instant::now();
        println!("file loading time: {:?}", file_loaded.duration_since(start));
        println!("graph creation time: {:?}",
                 graph_created.duration_since(file_loaded));


        g

    }

    #[test]
    #[ignore]
    fn load_test() {
        let (nodes, edges) = load_file("/home/flo/workspaces/rust/graphdata/saarland.graph");
        assert_eq!(nodes.len(), 595294);
        assert_eq!(edges.len(), 1241741);
    }

}
