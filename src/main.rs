extern crate rayon;
#[macro_use]
extern crate heapsize_derive;
extern crate heapsize;

use heapsize::HeapSizeOf;
mod ae1;


fn ae1_main() {
    let graph = ae1::load_graph("/home/flo/workspaces/rust/graphdata/bw.graph");
    println!("Size of graph: {} MB",
             graph.heap_size_of_children() / 1048576);

}

fn main() {
    ae1_main();
}
