extern crate radix_trie;
extern crate petgraph;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use radix_trie::Trie;
use std::collections::HashMap;

fn positions(x: i32, y: i32) -> Vec<(i32,i32)> {
    assert!(x > 0);
    assert!(y > 0);

    (0..x).flat_map(|i| (0..y).map(|j| (i,j)).collect::<Vec<(i32,i32)>>()).collect()
}

fn neighboring_indices(x: i32, y: i32) -> Vec<((i32,i32),(i32,i32))> {
    assert!(x > 0);
    assert!(y > 0);

    let pairs: Vec<((i32,i32),(i32,i32))> = positions(x,y).into_iter()
        .flat_map(|pair|
          match pair {
              (i,j) => vec![
                  (pair, (i-1,   j)),
                  (pair, (i-1, j-1)),
                  (pair, (i,   j-1))]
          })
        .filter(|pair_o_pairs|
          match *pair_o_pairs {
              ((_,_), (i2,j2)) => !( i2 < 0 || j2 < 0 )
          })
        .collect();

    assert!(!pairs.is_empty());

    pairs
}

fn build_trie() -> Result<Trie<String, ()>, std::io::Error> {
    let f = try!(File::open("./words"));
    let reader = BufReader::new(f);
    let mut trie: Trie<String, ()> = Trie::new();

    for line in reader.lines() {
        trie.insert(try!(line), ());
    }

    Ok(trie)
}

fn main() {
    let mut graph: Graph<(), ()> = Graph::new();
    let mut positions_to_node_indices: HashMap<(i32,i32), NodeIndex> = HashMap::new();
    let mut node_indices_to_positions: HashMap<NodeIndex, (i32,i32)> = HashMap::new();
    let trie: Trie<String, ()>;

    for position in positions(5,5) {
        let node = graph.add_node(());

        positions_to_node_indices.insert(position, node);
        node_indices_to_positions.insert(node, position);
    }

    for tuhpl in neighboring_indices(5,5) {
        let edge_index: petgraph::graph::EdgeIndex;
        match tuhpl {
            ((i1, j1),(i2,j2)) => {
                println!("({},{}) ({},{})", i1, j1, i2, j2);

                let res1: Option<&NodeIndex> = positions_to_node_indices.get(&(i1,j1));
                let res2: Option<&NodeIndex> = positions_to_node_indices.get(&(i2,j2));

                match (res1,res2) {
                    (Some(&index1), Some(&index2)) => {
                        edge_index = graph.add_edge(index1, index2, ());
                    },
                    _ => panic!("what the fuck")
                }
            }
        }
    }

    match build_trie() {
        Ok(trie1) => trie = trie1,
        Err(str) => panic!("error building trie: {}", str)
    }
}
