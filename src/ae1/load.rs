use rayon::prelude::*;
use ae1::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;

fn load_file<P: AsRef<Path>>(file: P) -> (Vec<NodeInfo>, Vec<EdgeInfo>) {
    let mut buffer = String::new();

    let mut file = File::open(file).expect("File could not be opened");
    file.read_to_string(&mut buffer)
        .expect("Could not read file");
    let lines: Vec<&str> = buffer
        .lines()
        .skip_while(|l| l.starts_with('#') || l.is_empty())
        .collect();
    let node_count: usize = lines[0].parse().expect("Node Count could not be parsed");
    let nodes = lines[2..node_count + 2]
        .par_iter()
        .map(|l| {
            let mut raw_node_data = l.split(' ');
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
            NodeInfo::new(osm_id, lat, long, height)
        })
        .collect();

    let edges = lines[node_count + 2..]
        .par_iter()
        .map(|l| {
            let mut raw_node_data = l.split(' ');

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
            EdgeInfo::new(source, dest, length, speed)

        })
        .collect();

    (nodes, edges)
}
pub fn load_graph<P: AsRef<Path>>(file: P) -> Graph<EdgeInfo> {
    use std::time::Instant;
    let start = Instant::now();
    let (nodes, edges) = load_file(file);
    let file_loaded = Instant::now();
    let g = Graph::new(nodes, edges);
    let graph_created = Instant::now();
    println!("file loading time:   {:?}",
             file_loaded.duration_since(start));
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
