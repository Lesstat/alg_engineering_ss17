use super::{Edge, Graph, NodeId};

use std::collections::VecDeque;
use std::time::Instant;

impl<E: Edge> Graph<E> {
    pub fn count_components(&self) -> usize {
        let start = Instant::now();
        let mut visited = vec![false; self.node_info.len()];
        let mut queue = VecDeque::<NodeId>::new();
        let mut count = 0;
        for id in 0..visited.len() {
            if !visited[id] {
                queue.push_front(id);
            } else {
                continue;
            }
            while let Some(n) = queue.pop_front() {
                if !visited[n] {
                    visited[n] = true;
                    queue.extend(self.outgoing_edges_for(n)
                                     .iter()
                                     .map(|e| e.get_dest_id())
                                     .filter(|&n| !visited[n]));
                }
            }
            count += 1;
        }

        let end = Instant::now();
        println!("Counting Components took {:?}", end.duration_since(start));
        count
    }
}
