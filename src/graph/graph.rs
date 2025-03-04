use std::{collections::{HashMap, HashSet}, fmt::Debug, hash::Hash};

type NodeID = usize;

pub struct Graph<T: PartialEq + Eq + Hash + Debug> {
    id_counter: usize,
    id_dict: HashMap<T, NodeID>,
    nodes_dict: HashMap<NodeID, Node>,
}

struct Node {
    id: NodeID,
    children: Vec<NodeID>,
}

impl Node {
    fn new(id: NodeID) -> Self {
        Self {
            id,
            children: Vec::new(),
        }
    }

    fn add_edge(&mut self, id: NodeID) -> bool {
        let ret = self.children.contains(&id);

        self.children.push(id);

        ret
    }
}

impl <T:  PartialEq + Eq + Hash + Debug> Graph<T> {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            id_dict: HashMap::new(),
            nodes_dict: HashMap::new(),
        }
    }


    // 使用するノードを登録する
    pub fn add_node(&mut self, u: T) -> Result<(), String> {
        if self.id_dict.contains_key(&u) {
            return Err(format!("node {:#?} is already added", u));
        }

        let new_id = self.id_counter;
        self.id_counter += 1;
        self.id_dict.insert(u, new_id);
        self.nodes_dict.insert(new_id, Node::new(new_id));

        Ok(())
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

        let node = self.nodes_dict.get_mut(&from_id).unwrap(); // add_node メソッドを介してしか追加されずその際に Node は作られている
        return Ok(node.add_edge(to_id));
    }

    fn has_cycle_dfs(
        &self,
        node: NodeID,
        visited: &mut HashSet<NodeID>,
        rec_stack: &mut Vec<NodeID>,
        cycle: &mut Vec<NodeID>,
    ) -> bool {
        if let Some(pos) = rec_stack.iter().position(|&x| x == node) {
            // サイクル発見: `rec_stack` からサイクル部分を取り出す
            *cycle = rec_stack[pos..].to_vec();
            cycle.push(node);
            return true;
        }

        if visited.contains(&node) {
            return false;
        }

        visited.insert(node);
        rec_stack.push(node);

        if let Some(n) = self.nodes_dict.get(&node) {
            for &neighbor in &n.children {
                if self.has_cycle_dfs(neighbor, visited, rec_stack, cycle) {
                    return true;
                }
            }
        }

        rec_stack.pop(); // 探索が終わったら戻す
        false
    }

    pub fn detect_cycle(&self) -> Option<Vec<NodeID>> {
        let mut visited = HashSet::new();
        let mut rec_stack = Vec::new();
        let mut cycle = Vec::new();

        for &node in self.nodes_dict.keys() {
            if !visited.contains(&node) {
                if self.has_cycle_dfs(node, &mut visited, &mut rec_stack, &mut cycle) {
                    return Some(cycle);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, Node};

    #[test]
    fn test_node_add_edge() {
        {
            // standard case
            let mut n = Node::new(0);
            assert_eq!(n.add_edge(1), false);
        }
        {
            // children duplication
            let mut n = Node::new(0);
            assert_eq!(n.add_edge(1), false);
            assert_eq!(n.add_edge(1), true);
            assert_eq!(n.add_edge(1), true);
        }
        {
            // self cyclic (but no error)
            let mut n = Node::new(0);
            assert_eq!(n.add_edge(0), false);
            assert_eq!(n.add_edge(0), true);
        }
    }

    #[test]
    fn test_graph_add_node() {
        {
            // standard case
            let mut g = Graph::new();
            assert_eq!(g.add_node("node1"), Ok(()))
        }
        {
            // node duplication
            let mut g = Graph::new();
            let _ = g.add_node("node1");
            assert_eq!(g.add_node("node1").is_err(), true);
        }
    }

    #[test]
    fn test_graph_add_edge() {
        {
            // not existing node
            let mut g = Graph::new();
            assert_eq!(g.add_edge(&0, &0).is_err(), true);
        }
        {
            // not existing node2
            let mut g = Graph::new();
            let _ = g.add_node(0);
            assert_eq!(g.add_edge(&0, &1).is_err(), true);
        }
        {
            // standard case
            let mut g = Graph::new();
            let _ = g.add_node(0);
            let _ = g.add_node(1);
            assert_eq!(g.add_edge(&0, &1), Ok(false));
        }
        {
            // edge duplication
            let mut g = Graph::new();
            let _ = g.add_node(0);
            let _ = g.add_node(1);
            assert_eq!(g.add_edge(&0, &1), Ok(false));
            assert_eq!(g.add_edge(&0, &1), Ok(true));
        }
        {
            // cyclic
            let mut g = Graph::new();
            let _ = g.add_node(0);
            let _ = g.add_node(1);
            assert_eq!(g.add_edge(&0, &1), Ok(false));
            assert_eq!(g.add_edge(&1, &0), Ok(false));
        }
    }

    #[test]
    fn test_detect_cycle_no_cycle() {
        let mut g = Graph::new();
        let _ = g.add_node(0);
        let _ = g.add_node(1);
        let _ = g.add_node(2);
        let _ = g.add_edge(&0, &1);
        let _ = g.add_edge(&1, &2);

        assert_eq!(g.detect_cycle(), None);
    }

    #[test]
    fn test_detect_cycle_single_cycle() {
        let mut g = Graph::new();
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
        let mut g = Graph::new();
        let _ = g.add_node(0);
        let _ = g.add_node(1);
        let _ = g.add_node(2);
        let _ = g.add_node(3);
        let _ = g.add_node(4);
        let _ = g.add_edge(&0,&1);
        let _ = g.add_edge(&1,&2);
        let _ = g.add_edge(&2,&0); // サイクル1: 0 → 1 → 2 → 0
        let _ = g.add_edge(&3,&4);
        let _ = g.add_edge(&4,&3); // サイクル2: 3 → 4 → 3

        let cycle = g.detect_cycle().unwrap();
        println!("{:#?}", cycle);
        assert!(cycle.len() >= 3); // いずれかのサイクルが見つかること
        assert_eq!(cycle.first(), cycle.last());
    }

    #[test]
    fn test_detect_cycle_self_loop() {
        let mut g = Graph::new();
        let _ = g.add_node(0);
        let _ = g.add_edge(&0, &0); // 自己ループ

        let cycle = g.detect_cycle().unwrap();
        assert_eq!(cycle, vec![0, 0]); // 自己ループのサイクル
    }
}
