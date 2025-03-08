use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use super::core::{GraphCore, NodeID};

pub struct Graph<T: PartialEq + Eq + Hash + Debug> {
    id_counter: usize,
    id_dict: HashMap<T, NodeID>,
    core: GraphCore,
}

impl<T: PartialEq + Eq + Hash + Debug> Graph<T> {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            id_dict: HashMap::new(),
            core: GraphCore::new(),
        }
    }

    pub fn get_node_by_id(&self, id: &NodeID) -> Option<&T> {
        let mut ret: Option<&T> = None;
        for (k, v) in self.id_dict.iter() {
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
    pub fn add_node(&mut self, u: T) -> Result<(), String> {
        if self.id_dict.contains_key(&u) {
            return Err(format!("node {:#?} is already added", u));
        }

        let new_id = self.id_counter;
        self.id_counter += 1;
        self.id_dict.insert(u, new_id);

        self.core.add_node(new_id)
    }

    // すでにエッジが登録されている場合 false が返される (ただし，複数のエッジとして登録はされる)
    pub fn add_edge(&mut self, u_from: &T, u_to: &T) -> Result<bool, String> {
        let from_id = *self
            .id_dict
            .get(&u_from)
            .ok_or(format!("node {:#?} is not added", u_from))?;
        let to_id = *self
            .id_dict
            .get(&u_to)
            .ok_or(format!("node {:#?} is not added", u_to))?;

        return self.core.add_edge(from_id, to_id);
    }

    pub fn detect_cycle(&self) -> Option<Vec<&T>> {
        let inner_ret = self.core.detect_cycle();

        match inner_ret {
            Some(v) => {
                let mut ret = Vec::<&T>::new();

                for i in v.iter() {
                    ret.push(self.get_node_by_id(i).unwrap())
                }

                Some(ret)
            }
            None => None,
        }
    }

    pub fn traverse<F>(&self, start: &T, mut f: F)
    where
        F: FnMut(&T),
    {
        if let Some(&start_id) = self.id_dict.get(start) {
            let mut visited = HashSet::new();
            self.traverse_recursive(start_id, &mut visited, &mut f);
        }
    }

    fn traverse_recursive<F>(&self, node_id: NodeID, visited: &mut HashSet<NodeID>, f: &mut F)
    where
        F: FnMut(&T),
    {
        if visited.contains(&node_id) {
            return;
        }
        visited.insert(node_id);

        if let Some(node_value) = self.get_node_by_id(&node_id) {
            f(node_value);
        }

        if let Some(node) = self.core.nodes_dict.get(&node_id) {
            for &child_id in &node.children {
                self.traverse_recursive(child_id, visited, f);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test_graph_add_node() {
        {
            // standard case
            let mut g = Graph::new();
            assert_eq!(g.add_node(0), Ok(()))
        }
        {
            // node duplication
            let mut g = Graph::new();
            let _ = g.add_node(0);
            assert_eq!(g.add_node(0).is_err(), true);
        }
    }

    #[test]
    fn test_detect_cycle_no_cycle() {
        let mut g = Graph::new();
        let _ = g.add_node("A");
        let _ = g.add_node("B");
        let _ = g.add_node("C");
        let _ = g.add_edge(&"A", &"B");
        let _ = g.add_edge(&"B", &"C");

        assert_eq!(g.detect_cycle(), None);
    }

    #[test]
    fn test_detect_cycle_single_cycle() {
        let mut g = Graph::new();
        let _ = g.add_node("A");
        let _ = g.add_node("B");
        let _ = g.add_node("C");
        let _ = g.add_edge(&"A", &"B");
        let _ = g.add_edge(&"B", &"C");
        let _ = g.add_edge(&"C", &"A"); // A → B → C → A のサイクル

        let cycle = g.detect_cycle().unwrap();
        assert!(cycle.len() >= 3);
        assert_eq!(cycle.first(), cycle.last()); // サイクルは閉じているはず
    }

    #[test]
    fn test_detect_cycle_multiple_cycles() {
        let mut g = Graph::new();
        let _ = g.add_node("A");
        let _ = g.add_node("B");
        let _ = g.add_node("C");
        let _ = g.add_node("X");
        let _ = g.add_node("Y");
        let _ = g.add_edge(&"A", &"B");
        let _ = g.add_edge(&"B", &"C");
        let _ = g.add_edge(&"C", &"A"); // サイクル1: A → B → C → A
        let _ = g.add_edge(&"X", &"Y");
        let _ = g.add_edge(&"Y", &"X"); // サイクル2: X → Y → X

        let cycle = g.detect_cycle().unwrap();
        assert!(cycle.len() >= 3);
        assert_eq!(cycle.first(), cycle.last());
    }

    #[test]
    fn test_detect_cycle_self_loop() {
        let mut g = Graph::new();
        let _ = g.add_node("A");
        let _ = g.add_edge(&"A", &"A"); // 自己ループ

        let cycle = g.detect_cycle().unwrap();
        assert_eq!(cycle, vec![&"A", &"A"]); // A → A の自己ループ
    }

    #[test]
    fn test_detect_cycle_with_numbers() {
        let mut g = Graph::new();
        let _ = g.add_node(1);
        let _ = g.add_node(2);
        let _ = g.add_node(3);
        let _ = g.add_edge(&1, &2);
        let _ = g.add_edge(&2, &3);
        let _ = g.add_edge(&3, &1); // 1 → 2 → 3 → 1 のサイクル

        let cycle = g.detect_cycle().unwrap();
        assert!(cycle.len() >= 3);
        assert_eq!(cycle.first(), cycle.last());
    }
}
