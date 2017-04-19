extern crate rayon;
#[macro_use]
extern crate heapsize_derive;
extern crate heapsize;
extern crate rand;

use rand::Rng;
use heapsize::HeapSizeOf;

use std::time::Instant;

mod ae1;

use ae1::NodeId;

fn ae1_main() {
    let graph = ae1::load_graph("/home/flo/workspaces/rust/graphdata/bw.graph");
    println!("Size of graph: {} MB",
             graph.heap_size_of_children() / 1048576);

    println!("#Connected components: {}", graph.count_components());
    let tries = 1000;
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
    for try in 0..tries {
        graph.dijkstra(sources[try], destinations[try]);
    }
    let end = Instant::now();
    println!("{} dijkstras took {:?}", tries, end.duration_since(start));
    println!("average run: {} seconds",
             end.duration_since(start).as_secs() as f64 / tries as f64);



}

fn main() {
    ae1_main();
}
