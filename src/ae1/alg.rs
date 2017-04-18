use super::{Edge, Graph, NodeId};

use std::collections::{BTreeSet, VecDeque};
use std::time::Instant;

type Component = BTreeSet<NodeId>;
impl<E: Edge> Graph<E> {
    pub fn count_components(&self) -> usize {
        let start = Instant::now();
        let mut visited = vec![false; self.node_info.len()];
        let mut components = Vec::<Component>::new();
        for id in 0..visited.len() {
            if !visited[id] {
                let mut comp = self.dfs(id, &mut visited);
                for component in &mut components {
                    if !component.is_disjoint(&comp) {
                        component.append(&mut comp);
                        break;
                    }
                }
                if !comp.is_empty() {
                    components.push(comp);
                }
            } else {
                continue;
            }
        }

        let end = Instant::now();
        println!("Counting Components took {:?}", end.duration_since(start));
        components.len()
    }

    fn dfs(&self, start: NodeId, visited: &mut Vec<bool>) -> Component {
        let mut result = BTreeSet::new();
        let mut queue = VecDeque::<NodeId>::new();
        queue.push_front(start);
        while let Some(n) = queue.pop_front() {
            if !visited[n] {
                visited[n] = true;
                result.insert(n);
                queue.extend(self.outgoing_edges_for(n)
                                 .iter()
                                 .map(|e| e.get_dest_id())
                                 .filter(|&n| !visited[n]));
            }
        }
        result
    }
}

#[test]
fn count() {
    use super::{EdgeInfo, NodeInfo};
    let g = Graph::new(vec![NodeInfo::new(1, 2.3, 3.4, 0),
                            NodeInfo::new(2, 2.3, 3.4, 0),
                            NodeInfo::new(3, 2.3, 3.4, 0),
                            NodeInfo::new(4, 2.3, 3.4, 0)],
                       vec![EdgeInfo::new(0, 1, 3, 3),
                            EdgeInfo::new(0, 2, 3, 3),
                            EdgeInfo::new(2, 3, 3, 3),
                            EdgeInfo::new(4, 0, 3, 3)]);
    assert_eq!(g.count_components(), 1)
}
