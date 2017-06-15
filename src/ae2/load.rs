
use rayon::prelude::*;
use ae2::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;

fn load_file<P: AsRef<Path>>(file: P) -> (Vec<ChNodeInfo>, Vec<ChEdgeInfo>) {
    let mut buffer = String::new();

    let mut file = File::open(file).expect("File could not be opened");
    file.read_to_string(&mut buffer).expect(
        "Could not read file",
    );
    let lines: Vec<&str> = buffer
        .lines()
        .skip_while(|l| l.starts_with('#') || l.is_empty())
        .collect();
    let node_count: usize = lines[0].parse().expect("Node Count could not be parsed");
    let nodes = lines[2..node_count + 2]
        .par_iter()
        .map(|l| {
            let mut raw_node_data = l.split(' ');
            let id: NodeId = raw_node_data
                .next()
                .map(str::parse)
                .expect("No Node ID Data")
                .expect("Node ID not parse-able");
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
            let level: Level = raw_node_data
                .next()
                .map(str::parse)
                .expect("No Level Data")
                .expect("Level not parse-able");
            ChNodeInfo::new(id, osm_id, lat, long, height, level)
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
                .unwrap_or(0);
            let edge_a: Option<EdgeId> = raw_node_data
                .next()
                .map(str::parse)
                .expect("No edge_a found")
                .ok();
            let edge_b: Option<EdgeId> = raw_node_data
                .next()
                .map(str::parse)
                .expect("No edgeb found")
                .ok();
            ChEdgeInfo::new(source, dest, length, speed, edge_a, edge_b)

        })
        .collect();

    (nodes, edges)
}
pub fn load_graph<P: AsRef<Path>>(file: P) -> ChGraph {
    use std::time::Instant;
    let start = Instant::now();
    let (nodes, edges) = load_file(file);
    let file_loaded = Instant::now();
    let g = ChGraph::new(nodes, edges);
    let graph_created = Instant::now();
    println!(
        "file loading time:   {:?}",
        file_loaded.duration_since(start)
    );
    println!(
        "graph creation time: {:?}",
        graph_created.duration_since(file_loaded)
    );
    g

}
