use usize_graph::graph::Graph;

mod usize_graph;

fn main() {
    let mut g = Graph::new();
    let _ = g.add_node(2);
    let _ = g.add_node(3);
    let _ = g.add_node(0);
    let _ = g.add_node(1);
    let _ = g.add_node(4);
    let _ = g.add_edge(0, 1);
    let _ = g.add_edge(1, 2);
    let _ = g.add_edge(2, 0); // サイクル1: 0 → 1 → 2 → 0
    let _ = g.add_edge(3, 4);
    let _ = g.add_edge(4, 3); // サイクル2: 3 → 4 → 3

    let cycle = g.detect_cycle().unwrap();

    for c in cycle {
        let u = *g.get_node(c).unwrap();
        println!("{}", u);
    }
}
