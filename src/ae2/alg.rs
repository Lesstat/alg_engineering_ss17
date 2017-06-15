use super::{ChGraph, Length, NodeId};

use std::usize;
use std::cmp::Ordering;
use std::collections::BTreeSet;

impl ChGraph {
    pub fn dijkstra(&self) -> ChDijkstra {
        ChDijkstra {
            s_dist: vec![usize::MAX; self.node_count()],
            t_dist: vec![usize::MAX; self.node_count()],
            s_touched: Default::default(),
            t_touched: Default::default(),
            graph: self,
        }
    }
}

pub struct ChDijkstra<'a> {
    s_dist: Vec<Length>,
    t_dist: Vec<Length>,
    s_touched: BTreeSet<NodeId>,
    t_touched: BTreeSet<NodeId>,
    graph: &'a ChGraph,
}

#[derive(PartialEq, Eq, Debug)]
struct NodeCost {
    node: NodeId,
    cost: usize,
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> ChDijkstra<'a> {
    pub fn distance(&mut self, s: NodeId, t: NodeId) -> Length {
        use std::collections::BinaryHeap;

        for &node in &self.s_touched {
            self.s_dist[node] = usize::MAX;
        }
        self.s_touched.clear();

        let mut s_heap = BinaryHeap::new();
        s_heap.push(NodeCost { node: s, cost: 0 });
        self.s_dist[s] = 0;
        self.s_touched.insert(s);

        for &node in &self.t_touched {
            self.t_dist[node] = usize::MAX;
        }
        self.t_touched.clear();

        let mut t_heap = BinaryHeap::new();
        t_heap.push(NodeCost { node: t, cost: 0 });
        self.t_dist[t] = 0;
        self.t_touched.insert(t);

        let mut min_candidate = usize::MAX;
        let mut s_empty = false;
        let mut t_empty = false;
        let mut s_bigger = false;
        let mut t_bigger = false;

        loop {
            if s_empty && t_empty {
                return usize::MAX;
            }
            if s_bigger && t_bigger {
                return min_candidate;
            }
            if let Some(NodeCost { node, cost }) = s_heap.pop() {

                if cost > self.s_dist[node] {
                    continue;
                } else if cost > min_candidate {
                    s_bigger = true;
                }
                if self.t_dist[node] != usize::MAX {
                    let candidate = self.t_dist[node] + self.s_dist[node];
                    if candidate < min_candidate {
                        min_candidate = candidate;
                    }
                }

                for edge in self.graph.outgoing_edges_for(node) {
                    if self.graph.level[edge.endpoint] >= self.graph.level[node] {
                        let next = NodeCost {
                            node: edge.endpoint,
                            cost: cost + edge.weight,
                        };

                        if next.cost < self.s_dist[next.node] {


                            self.s_dist[next.node] = next.cost;
                            self.s_touched.insert(next.node);
                            s_heap.push(next);

                        }
                    }
                }
            } else {
                s_empty = true;
            }


            if let Some(NodeCost { node, cost }) = t_heap.pop() {

                if cost > self.t_dist[node] {
                    continue;
                } else if cost > min_candidate {
                    t_bigger = true;
                }


                if self.s_dist[node] != usize::MAX {
                    let candidate = self.t_dist[node] + self.s_dist[node];
                    if candidate < min_candidate {
                        min_candidate = candidate;
                    }
                }
                for edge in self.graph.ingoing_edges_for(node) {
                    if self.graph.level[edge.endpoint] >= self.graph.level[node] {

                        let next = NodeCost {
                            node: edge.endpoint,
                            cost: cost + edge.weight,
                        };
                        if next.cost < self.t_dist[next.node] {
                            self.t_dist[next.node] = next.cost;
                            self.t_touched.insert(next.node);
                            t_heap.push(next);
                        }
                    }
                }
            } else {
                t_empty = true
            }
        }

    }
}
