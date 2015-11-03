extern crate radix_trie;
extern crate petgraph;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use petgraph::graph::Graph;
use petgraph::graph::Neighbors;
use petgraph::graph::NodeIndex;
use radix_trie::Trie;
use std::collections::HashMap;
use std::rc::Rc;

struct PathComponent<'a> {
    position: (i32, i32),
    trie: &'a Trie<String, ()>,
    character: char,
    previous: Option<Rc<PathComponent<'a>>>
}


impl <'a, 'b> PathComponent<'a> {
    fn iter(&'b self) -> PathComponentIterator<'a, 'b> {
        PathComponentIterator {current: &self}
    }

    fn characters_so_far(&self) -> String {
        self.iter().map(|pc| pc.character).collect::<String>()
    }

    fn positions_so_far(&'a self) -> Vec<(i32,i32)> {
        self.iter().map(|pc| pc.position).collect::<Vec<(i32,i32)>>()
    }
}

struct PathComponentIterator<'a: 'b, 'b> {
    current: &'b PathComponent<'a>
}

impl <'a: 'b, 'b> Iterator for PathComponentIterator<'a, 'b> {
    type Item = &'b PathComponent<'a>;

    fn next(&mut self) -> Option<&'b PathComponent<'a>> {
        match self.current.previous {
            Some(ref pc) => {self.current = pc; return Some(pc)},
            None => None
        }
    }
}

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

fn build_grid() -> Result<[[char; 4]; 4], std::io::Error> {
    // This is just exactly the same thing as reading in the example board.
    // I don't really care about spending much time reading in boards right
    // now, so I'm going to leave this hard coded for the time being.

    Ok([['a','t','g','c'],
        ['l','r','j','e'],
        ['j','r','f','g'],
        ['m','h','e','s']])
}

fn build_to_visit<'a>(
    grid: &[[char; 4]; 4],
    trie: &'a Trie<String, ()>,
    current_path: Rc<PathComponent<'a>>,
    graph: &Graph<(), (), petgraph::Undirected>,
    current_node: &NodeIndex,
    node_indices_to_positions: &'a HashMap<NodeIndex, (i32,i32)>
    ) -> Vec<Rc<PathComponent<'a>>> {
        graph.neighbors(*current_node)
            .map(|neighbor| {
                 let position = *node_indices_to_positions.get(&neighbor).expect("should be impossible");
                 let sub_trie: &Trie<String, ()>;

                 match trie.get_node(&current_path.characters_so_far()) {
                     Some(an_trie) => sub_trie = an_trie,
                     None => return None
                 }

                 Some(Rc::new(PathComponent {
                     position: position,
                     character: grid[position.0 as usize][position.1 as usize],
                     trie: sub_trie,
                     previous: Some(current_path.clone())
                 }))})
            .filter(|val| !val.is_none())
            .map(|val| val.expect("this will always work because we filtered the nones out"))
            .collect()
}

fn main() {
    let mut graph: Graph<(), (), petgraph::Undirected> = Graph::new_undirected();
    let mut positions_to_node_indices: HashMap<(i32,i32), NodeIndex> = HashMap::new();
    let mut node_indices_to_positions: HashMap<NodeIndex, (i32,i32)> = HashMap::new();
    let trie: Trie<String, ()>;
    let grid: [[char; 4]; 4];

    for position in positions(4,4) {
        let node = graph.add_node(());

        positions_to_node_indices.insert(position, node);
        node_indices_to_positions.insert(node, position);
    }

    for tuhpl in neighboring_indices(4,4) {
        match tuhpl {
            ((i1, j1),(i2,j2)) => {
                println!("({},{}) ({},{})", i1, j1, i2, j2);

                let msg = "the construction of the position to node index map is broken.";
                let node1: &NodeIndex = positions_to_node_indices.get(&(i1,j1)).expect(msg);
                let node2: &NodeIndex = positions_to_node_indices.get(&(i2,j2)).expect(msg);

                graph.add_edge(*node1, *node2, ());
            }
        }
    }

    match build_trie() {
        Ok(trie1) => trie = trie1,
        Err(str) => panic!("error building trie: {}", str)
    }

    match build_grid() {
        Ok(grid1) => grid = grid1,
        Err(str) => panic!("error building grid: {}", str)
    }

    {
        let mut position_iterator = positions(4,4).into_iter();

        while let Some((i,j)) = position_iterator.next() {
            println!("starting from ({}, {})", i, j);
            let current_char = grid[i as usize][j as usize];
            let mut current_node: &NodeIndex = positions_to_node_indices.get(&(i,j))
                .expect("if this is reached the whole program is hopelessly buggy.");
            let neighbors: Neighbors<()> = graph.neighbors(*current_node);
            let mut current_path: Rc<PathComponent> = Rc::new(PathComponent {
                character: current_char,
                position: (i,j),
                trie: &trie,
                previous: None
            });

            assert!(graph.neighbors(*current_node).count() != 0);

            let mut to_visit: Vec<Rc<PathComponent>> = build_to_visit(&grid, &trie, current_path, &graph, &current_node, &node_indices_to_positions);

            assert!(!to_visit.is_empty());

            while !to_visit.is_empty() {
                let thing = to_visit.pop();
                match thing {
                    Some(inner_thing) => {
                        println!("\t({}, {})", inner_thing.position.0, inner_thing.position.1);
                        current_path = inner_thing;
                        current_node = positions_to_node_indices.get(&current_path.position).expect("again should be right by construction.");
                        let mut to_potentially_visit = build_to_visit(&grid, &trie, current_path, &graph, &current_node, &node_indices_to_positions);
                        while let Some(pc) = to_potentially_visit.pop() {
                            if pc.positions_so_far().iter().any(|previous_pos| *previous_pos == pc.position) {
                                continue;
                            }
                            to_visit.push(pc.clone());
                        }
                    },
                    None => panic!("at the disco!")
                }
            }
        }
    }
}
