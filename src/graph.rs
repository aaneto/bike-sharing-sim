use petgraph::{
    Graph,
    Undirected,
    dot::{Config, Dot}
};
use std::{
    fmt::Debug,
    io::Write,
    fs::File,
};

pub fn plot_graph_to_file<A: Debug, B: Debug>(g: Graph<A, B, Undirected>) {
    let text = format!("{:?}", Dot::with_config(&g, &[Config::NodeIndexLabel, Config::EdgeNoLabel]));

    let err_msg = "Cannot create graph file";
    File::create("graph.viz")
        .expect(err_msg)
        .write_all(&text.as_bytes())
        .expect(err_msg);
}