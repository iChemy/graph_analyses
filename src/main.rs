use graph::graph::Graph;
use usize_graph::graph::UsizeGraph;

mod graph;
mod usize_graph;

fn main() {
    {
        let mut g = UsizeGraph::new();
        let _ = g.add_node(2);
        let _ = g.add_node(3);
        let _ = g.add_node(0);
        let _ = g.add_node(1);
        let _ = g.add_node(4);
        let _ = g.add_edge(&0, &1);
        let _ = g.add_edge(&1, &2);
        let _ = g.add_edge(&2, &0); // サイクル1: 0 → 1 → 2 → 0
        let _ = g.add_edge(&3, &4);
        let _ = g.add_edge(&4, &3); // サイクル2: 3 → 4 → 3

        let cycle = g.detect_cycle().unwrap();

        for c in cycle {
            let u = *g.get_node_by_id(&c).unwrap();
            println!("{}", u);
        }
    }

    {
        let mut g = Graph::<&str>::new();
        let node1 = "ndoe1";
        let node2 = "node2";

        let _ = g.add_node(&node1);
        let _ = g.add_node(&node2);

        let _ = g.add_edge(&node1, &node2);
        let _ = g.add_edge(&node2, &node1);

        let cycle = g.detect_cycle().unwrap();

        for c in cycle {
            println!("{}", c);
        }
    }

    {
        let mut graph = Graph::new();
        let node_a = "A";
        let node_b = "B";
        let node_c = "C";

        let _ = graph.add_node(node_a);
        let _ = graph.add_node(node_b);
        let _ = graph.add_node(node_c);

        let _ = graph.add_edge(&node_a, &node_b);
        let _ = graph.add_edge(&node_a, &node_c);

        graph.traverse(&"A", |node| {
            println!("Visited node: {}", node);
        });
    }
}
