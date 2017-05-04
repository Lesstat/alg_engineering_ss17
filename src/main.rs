extern crate rayon;
#[macro_use]
extern crate heapsize_derive;
extern crate heapsize;
extern crate rand;

use rand::Rng;
use heapsize::HeapSizeOf;

use std::time::Instant;

mod ae1;
mod ae2;

use ae1::NodeId;

fn ae1_main() {
    let graph = ae1::load_graph("/home/flo/workspaces/rust/graphdata/bw.graph");
    println!("Size of graph: {} MB",
             graph.heap_size_of_children() / 1048576);

    println!("#Connected components: {}", graph.count_components());
    let tries = 100;
    let mut sources = Vec::<NodeId>::with_capacity(tries);
    let mut destinations = Vec::<NodeId>::with_capacity(tries);
    let mut rng = rand::thread_rng();
    for _ in 0..tries {
        let source: NodeId = rng.gen();
        sources.push(source % graph.node_count());
        let dest: NodeId = rng.gen();
        destinations.push(dest % graph.node_count());
    }
    let start = Instant::now();
    let mut dijkstra = graph.dijkstra();
    for try in 0..tries {
        dijkstra.distance(sources[try], destinations[try]);
    }
    let end = Instant::now();
    println!("{} dijkstras took {:?}", tries, end.duration_since(start));
    println!("average run: {} seconds",
             end.duration_since(start).as_secs() as f64 / tries as f64);



}
fn ae2_main() {

    let graph = ae2::load_graph("/home/flo/workspaces/rust/graphdata/bw.ch");
    println!("Size of graph: {} MB",
             graph.heap_size_of_children() / 1048576);

    let tries = 100;
    let mut sources = Vec::<NodeId>::with_capacity(tries);
    let mut destinations = Vec::<NodeId>::with_capacity(tries);
    let mut rng = rand::thread_rng();
    for _ in 0..tries {
        let source: NodeId = rng.gen();
        sources.push(source % graph.node_count());
        let dest: NodeId = rng.gen();
        destinations.push(dest % graph.node_count());
    }
    let start = Instant::now();
    let mut dijkstra = graph.dijkstra();
    for try in 0..tries {
        dijkstra.distance(sources[try], destinations[try]);
    }
    let end = Instant::now();
    println!("{} dijkstras took {:?}", tries, end.duration_since(start));
    println!("average run: {} seconds",
             end.duration_since(start).as_secs() as f64 / tries as f64);
}

fn ae1_from_to(s: ae1::NodeId, t: ae1::NodeId) -> ae1::Length {

    use std::usize;

    let graph = ae1::load_graph("/home/flo/workspaces/rust/graphdata/saarland.graph");
    let mut dijk = graph.dijkstra();
    dijk.distance(s, t).unwrap_or(usize::MAX)
}

fn ae2_from_to(s: ae2::NodeId, t: ae2::NodeId) -> ae2::Length {

    let graph = ae2::load_graph("/home/flo/workspaces/rust/graphdata/saarland.ch");
    let mut dijk = graph.dijkstra();
    dijk.distance(s, t)
}
fn main() {
    let d1 = 9173; //ae1_from_to(5, 500);
    let d2 = ae2_from_to(5, 500);

    println!("d1: {}, d2: {}", d1, d2);

    assert_eq!(d1, d2)

}
