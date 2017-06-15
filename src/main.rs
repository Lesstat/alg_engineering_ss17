#![feature(test)]
extern crate test;


extern crate rayon;
#[macro_use]
extern crate heapsize_derive;
extern crate heapsize;
extern crate rand;

extern crate porter_stemmer;

use rand::Rng;
use heapsize::HeapSizeOf;

use std::time::Instant;

mod ae1;
mod ae2;
mod ae4;

use ae1::NodeId;

#[allow(dead_code)]
fn ae1_main() {
    let graph = ae1::load_graph("/home/flo/workspaces/rust/graphdata/bw.graph");
    println!(
        "Size of graph: {} MB",
        graph.heap_size_of_children() / 1048576
    );

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
    println!(
        "average run: {} seconds",
        end.duration_since(start).as_secs() as f64 / tries as f64
    );



}
#[allow(dead_code)]
fn ae2_main() {

    let graph = ae2::load_graph("/home/flo/workspaces/rust/graphdata/bw.ch");
    println!(
        "Size of graph: {} MB",
        graph.heap_size_of_children() / 1048576
    );

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
    let mut dijkstra = graph.dijkstra();
    for try in 0..tries {
        dijkstra.distance(sources[try], destinations[try]);
    }
    let end = Instant::now();
    println!("{} dijkstras took {:?}", tries, end.duration_since(start));
    println!(
        "average run: {} seconds",
        end.duration_since(start).as_secs() as f64 / tries as f64
    );
}

#[allow(dead_code)]
fn ae1_eq_ae2() {
    use std::usize;

    let graph1 = ae1::load_graph("/home/flo/workspaces/rust/graphdata/saarland.graph");
    let graph2 = ae2::load_graph("/home/flo/workspaces/rust/graphdata/saarland.ch");

    let tries = 40;
    let mut sources = Vec::<NodeId>::with_capacity(tries);
    let mut destinations = Vec::<NodeId>::with_capacity(tries);
    let mut rng = rand::thread_rng();
    for _ in 0..tries {
        let source: NodeId = rng.gen();
        sources.push(source % graph1.node_count());
        let dest: NodeId = rng.gen();
        destinations.push(dest % graph1.node_count());
    }
    let mut dijk1 = graph1.dijkstra();
    let mut dijk2 = graph2.dijkstra();
    for try in 0..tries {
        println!("try {}", try);

        let d2 = dijk2.distance(sources[try], destinations[try]);
        let (d1, _) = dijk1.distance(sources[try], destinations[try]).unwrap_or((
            usize::MAX,
            Default::default(),
        ));

        assert_eq!(d1, d2, "form {} to {}", sources[try], destinations[try])
    }
}

fn ae4_main() {
    use std::env;
    use std::io::{BufRead, stdin};

    let mut args = env::args();
    let path = args.nth(1).expect("expect file argument");
    let movies = ae4::load_movies(path).expect("movies could not be loaded");
    let index = ae4::build_inverted_index(&movies);
    let naive = match args.next() {
        Some(_) => true,
        None => false,
    };
    println!("Inverted index is build");
    if naive {
        println!("naive search is active too");
    }
    println!("Please enter Query");
    let stdin = stdin();
    let mut handle = stdin.lock();

    loop {
        let mut buf = String::new();
        handle.read_line(&mut buf).expect(
            "reading from stdin failed",
        );
        println!("looking for key {}", buf);
        let ind_dur = ae4::query_index(&index, &movies, &buf);

        if naive {
            let naiv_dur = ae4::naive_query(&movies, &buf);
            println!("vec duration: {:?}", naiv_dur);

        }

        println!("query duration: {:?}", ind_dur);


    }


}
fn main() {
    ae4_main();
}
