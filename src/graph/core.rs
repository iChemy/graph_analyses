use std::collections::{HashMap, HashSet};

pub type NodeID = usize;

pub struct GraphCore {
    pub nodes_dict: HashMap<NodeID, Node>,
}

pub struct Node {
    id: NodeID,
    pub children: HashSet<NodeID>,
}

impl Node {
    fn new(id: NodeID) -> Self {
        Self {
            id,
            children: HashSet::new(),
        }
    }

    fn add_edge(&mut self, id: NodeID) -> bool {
        let ret = self.children.contains(&id);

        self.children.insert(id);

        ret
    }
}

impl GraphCore {
    pub fn new() -> Self {
        Self {
            nodes_dict: HashMap::new(),
        }
    }

    // 使用するノードを登録する
    pub fn add_node(&mut self, new_id: NodeID) -> Result<(), String> {
        let old_node = self.nodes_dict.insert(new_id, Node::new(new_id));

        match &old_node {
            Some(n) => Err(format!("duplication: node {} is already added", n.id)),
            None => Ok(()),
        }
    }

    // すでにエッジが登録されている場合 false が返される (ただし，複数のエッジとして登録はされる)
    pub fn add_edge(&mut self, from_id: NodeID, to_id: NodeID) -> Result<bool, String> {
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

    /// グラフの探索を行い、各ノードで `visit` 関数を実行する
    pub fn traverse<F>(&self, start: NodeID, mut visit: F)
    where
        F: FnMut(NodeID),
    {
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![start];

        while let Some(node) = stack.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            // 渡された関数を実行
            visit(node);

            if let Some(n) = self.nodes_dict.get(&node) {
                for &neighbor in &n.children {
                    stack.push(neighbor);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

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
}
