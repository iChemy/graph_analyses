use std::collections::HashMap;

use crate::graph::core::{GraphCore, NodeID};

pub struct UsizeGraph {
    id_counter: usize,
    usize_id_dict: HashMap<usize, NodeID>,
    core: GraphCore,
}

impl UsizeGraph {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            usize_id_dict: HashMap::new(),
            core: GraphCore::new(),
        }
    }

    pub fn get_node_by_id(&self, id: &NodeID) -> Option<&usize> {
        let mut ret: Option<&usize> = None;
        for (k, v) in self.usize_id_dict.iter() {
            if v == id {
                if ret.is_none() {
                    ret = Some(k);
                } else {
                    panic!("NodeID duplication")
                }
            }
        }

        ret
    }

    // 使用するノードを登録する
    pub fn add_node(&mut self, u: usize) -> Result<(), String> {
        if self.usize_id_dict.contains_key(&u) {
            return Err(format!("node {} is already added", u));
        }

        let new_id = self.id_counter;
        self.id_counter += 1;
        self.usize_id_dict.insert(u, new_id);

        self.core.add_node(new_id)
    }

    // すでにエッジが登録されている場合 false が返される (ただし，複数のエッジとして登録はされる)
    pub fn add_edge(&mut self, u_from: &usize, u_to: &usize) -> Result<bool, String> {
        let from_id = *self
            .usize_id_dict
            .get(&u_from)
            .ok_or(format!("node {} is not added", u_from))?;
        let to_id = *self
            .usize_id_dict
            .get(&u_to)
            .ok_or(format!("node {} is not added", u_to))?;

        self.core.add_edge(from_id, to_id)
    }

    pub fn detect_cycle(&self) -> Option<Vec<NodeID>> {
        self.core.detect_cycle()
    }
}

#[cfg(test)]
mod tests {
    use super::UsizeGraph;

    #[test]
    fn test_graph_add_edge() {
        {
            // not existing node
            let mut g = UsizeGraph::new();
            assert_eq!(g.add_edge(&0, &0).is_err(), true);
        }
        {
            // not existing node2
            let mut g = UsizeGraph::new();
            let _ = g.add_node(0);
            assert_eq!(g.add_edge(&0, &1).is_err(), true);
        }
        {
            // standard case
            let mut g = UsizeGraph::new();
            let _ = g.add_node(0);
            let _ = g.add_node(1);
            assert_eq!(g.add_edge(&0, &1), Ok(false));
        }
        {
            // edge duplication
            let mut g = UsizeGraph::new();
            let _ = g.add_node(0);
            let _ = g.add_node(1);
            assert_eq!(g.add_edge(&0, &1), Ok(false));
            assert_eq!(g.add_edge(&0, &1), Ok(true));
        }
        {
            // cyclic
            let mut g = UsizeGraph::new();
            let _ = g.add_node(0);
            let _ = g.add_node(1);
            assert_eq!(g.add_edge(&0, &1), Ok(false));
            assert_eq!(g.add_edge(&1, &0), Ok(false));
        }
    }

    #[test]
    fn test_detect_cycle_no_cycle() {
        let mut g = UsizeGraph::new();
        let _ = g.add_node(0);
        let _ = g.add_node(1);
        let _ = g.add_node(2);
        let _ = g.add_edge(&0, &1);
        let _ = g.add_edge(&1, &2);

        assert_eq!(g.detect_cycle(), None);
    }

    #[test]
    fn test_detect_cycle_single_cycle() {
        let mut g = UsizeGraph::new();
        let _ = g.add_node(0);
        let _ = g.add_node(1);
        let _ = g.add_node(2);
        let _ = g.add_edge(&0, &1);
        let _ = g.add_edge(&1, &2);
        let _ = g.add_edge(&2, &0); // 0 → 1 → 2 → 0 のサイクル

        let cycle = g.detect_cycle().unwrap();
        assert!(cycle.len() >= 3); // 最低 3 つのノードを含む
        assert_eq!(cycle.first(), cycle.last()); // サイクルは閉じているはず
    }

    #[test]
    fn test_detect_cycle_multiple_cycles() {
        let mut g = UsizeGraph::new();
        let _ = g.add_node(0);
        let _ = g.add_node(1);
        let _ = g.add_node(2);
        let _ = g.add_node(3);
        let _ = g.add_node(4);
        let _ = g.add_edge(&0, &1);
        let _ = g.add_edge(&1, &2);
        let _ = g.add_edge(&2, &0); // サイクル1: 0 → 1 → 2 → 0
        let _ = g.add_edge(&3, &4);
        let _ = g.add_edge(&4, &3); // サイクル2: 3 → 4 → 3

        let cycle = g.detect_cycle().unwrap();
        println!("{:#?}", cycle);
        assert!(cycle.len() >= 3); // いずれかのサイクルが見つかること
        assert_eq!(cycle.first(), cycle.last());
    }

    #[test]
    fn test_detect_cycle_self_loop() {
        let mut g = UsizeGraph::new();
        let _ = g.add_node(0);
        let _ = g.add_edge(&0, &0); // 自己ループ

        let cycle = g.detect_cycle().unwrap();
        assert_eq!(cycle, vec![0, 0]); // 自己ループのサイクル
    }
}
