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
use std::collections::HashSet;
use std::rc::Rc;

struct PathComponent<'a> {
    position: (i32, i32),
    trie: &'a Trie<String, ()>,
    character: char,
    previous: Option<Rc<PathComponent<'a>>>
}


impl <'a, 'b> PathComponent<'a> {
    fn iter(&'b self) -> PathComponentIterator<'a, 'b> {
        PathComponentIterator {current: &self, done: false}
    }

    fn characters_so_far(&self) -> String {
        let mut chars = self.iter().map(|pc| pc.character).collect::<Vec<char>>();
        chars.reverse();
        chars.into_iter().collect()
    }

    fn positions_so_far(&self) -> Vec<(i32,i32)> {
        self.iter().map(|pc| pc.position).collect::<Vec<(i32,i32)>>()
    }
}

struct PathComponentIterator<'a: 'b, 'b> {
    current: &'b PathComponent<'a>,
    done: bool
}

impl <'a: 'b, 'b> Iterator for PathComponentIterator<'a, 'b> {
    type Item = &'b PathComponent<'a>;

    fn next(&mut self) -> Option<&'b PathComponent<'a>> {
        match self.current.previous {
            Some(ref pc) => {
                let ret = self.current;
                self.current = pc;
//                println!("off by one?");
//                print_position(&ret.position);
//                println!("");
//                print_position(&self.current.position);
//                println!("");
//                println!("i hope not");
                return Some(ret)
            },
            None => {
                if !self.done {
                    self.done = true;
                    return Some(self.current);
                }
                else {
                    return None
                }
            }
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
                  //  (i-1, j-1) (i-1, j) (i-1, j+1)
                  //  (i  , j-1) (i  , j) (i  , j+1)
                  //  (i+1, j-1) (i+1, j) (i+1, j+1)
                  (pair, (i  , j-1)),
                  (pair, (i+1, j-1)),
                  (pair, (i+1, j  )),
                  (pair, (i+1, j+1)),
                  (pair, (i  , j+1)),
                  (pair, (i-1, j+1)),
                  (pair, (i-1, j  )),
                  (pair, (i-1, j-1)),
                  ]
//                  (pair, (i+1, j-1)),
//                  (pair, (i+1, j+1)),
//                //(pair, (i  , j  )), you might think you want this entry, for symmetry. but think.
//                  (pair, (i  , j-1)),
//                  (pair, (i  , j+1))]
          })
        .filter(|pair_o_pairs|
          match *pair_o_pairs {
              ((_,_), (i2,j2)) => !(j2 > ( y - 1 ) || i2 > ( x - 1 ) || i2 < 0 || j2 < 0 )
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
        print!("path to ({}, {}) is", current_path.position.0, current_path.position.1);
        let mut in_order_positions = current_path.positions_so_far();
        in_order_positions.reverse();
//        for position in in_order_positions {
//            print!(" {},{}", position.0, position.1);
//        }
//        println!("");
//        print!("neighbors of ({}, {}) are ", current_path.position.0, current_path.position.1);
        let ret = graph.neighbors(*current_node)
            .map(|neighbor| {
                let position = *node_indices_to_positions.get(&neighbor).expect("should be impossible");
                let sub_trie: &Trie<String, ()>;
                let this_char: char = grid[position.0 as usize][position.1 as usize];

//                print!("({}, {}) ", position.0, position.1);

                let mut maybe_word = current_path.characters_so_far();
                maybe_word.push(this_char);

//               println!("");
//               print_position(&position);
//               println!(" might get filtered.");

//                println!("maybe_word = {}", maybe_word);
//                println!("trie.get_descendant succeeded? {}", trie.get_descendant(&maybe_word).is_some());
                match trie.get_descendant(&maybe_word) {
                    Some(an_trie) => sub_trie = an_trie,
                    None => return None
                }

//               print_position(&position);
//               println!(" didn't get filtered for not being a word-path");

                let mut sub_vec = current_path.positions_so_far();
                sub_vec.reverse();
                sub_vec.pop();

                if sub_vec.iter().any(|pos| *pos == position) {
                    return None
                }

//               print_position(&position);
//               println!(" didn't get filtered for overlappingness");

                Some(Rc::new(PathComponent {
                    position: position,
                    character: this_char,
                    trie: sub_trie,
                    previous: Some(current_path.clone())
                }))})
            .filter(|val| !val.is_none())
            .map(|val| val.expect("this will always work because we filtered the nones out"))
        .collect();
        println!("\n");
        ret
}

fn print_position(position: &(i32, i32)) {
    print!("{},{}", position.0, position.1)
}

fn print_path_from_tuples(tuples: Vec<(i32,i32)>) {
    for position in tuples {
        print_position(&position);
        print!(" ");
    };
}

fn print_path_from_pc(path: Rc<PathComponent>) {
    let mut local_positions = path.positions_so_far();
    local_positions.reverse();
    print_path_from_tuples(local_positions);
}

fn print_path_stack(stack: &Vec<Rc<PathComponent>>) {
    let mut local_stack = stack.clone();
    local_stack.reverse();
    for frame in local_stack {
        print_path_from_pc(frame);
        println!("");
    }
    println!("====\n");
}

fn main() {
    let mut graph: Graph<(), (), petgraph::Undirected> = Graph::new_undirected();
    let mut edges: HashSet<((i32,i32), (i32,i32))> = HashSet::new();
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
                let msg = "the construction of the position to node index map is broken.";
                let node1: &NodeIndex = positions_to_node_indices.get(&(i1,j1)).expect(msg);
                let node2: &NodeIndex = positions_to_node_indices.get(&(i2,j2)).expect(msg);

                if !(edges.contains(&((i1,j1), (i2,j2))) ||
                     edges.contains(&((i2,j2), (i1,j1)))) {
                    edges.insert(((i1,j1), (i2,j2)));
                    graph.add_edge(*node1, *node2, ());
                }
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
            println!("");
            println!("starting from ({}, {})", i, j);
            let current_char = grid[i as usize][j as usize];
            let current_node: &NodeIndex = positions_to_node_indices.get(&(i,j))
                .expect("if this is reached the whole program is hopelessly buggy.");
            let neighbors: Neighbors<()> = graph.neighbors(*current_node);
            let mut current_path: Rc<PathComponent> = Rc::new(PathComponent {
                character: current_char,
                position: (i,j),
                trie: &trie,
                previous: None
            });

            let neighbor_count = graph.neighbors(*current_node).count();
            assert!(neighbor_count != 0);

            let mut to_visit: Vec<Rc<PathComponent>> = build_to_visit(&grid, &trie, current_path, &graph, &current_node, &node_indices_to_positions);

            print_path_stack(&to_visit);

            assert!(neighbor_count >= to_visit.len());

//            print_path_stack(&to_visit);
//            println!("stack should not be empty"); // this is lies

            while !to_visit.is_empty() {
                println!("yay!");
                let thing = to_visit.pop();
//                println!("just popped actual");
//                print_path_stack(&to_visit);
                match thing {
                    Some(inner_thing) => {
                        let maybe_word = inner_thing.characters_so_far();
                        match trie.get(&maybe_word) {
                            Some(_) => {
                                if maybe_word.len() >= 3 {
                                    println!("******************************************************************************************************");
                                    println!("{}", maybe_word);
                                    println!("******************************************************************************************************");
                                }
                            },
                            None => println!("didn't find a word")
                        }
                        //println!("\t({}, {})", inner_thing.position.0, inner_thing.position.1);
                        current_path = inner_thing;
                        current_node = positions_to_node_indices.get(&current_path.position).expect("again should be right by construction.");
                        let mut to_potentially_visit = build_to_visit(&grid, &trie, current_path, &graph, &current_node, &node_indices_to_positions);
//                        println!("number of potential nodes to visit next is {}", to_potentially_visit.len());
                        while let Some(pc) = to_potentially_visit.pop() {
//                            println!("just popped potential");
//                            print_path_stack(&to_visit);
//                            let mut sub_vec = pc.positions_so_far();
//                            sub_vec.pop();
//                            println!("the following path should be a proper subset of the path below it.");
//                            print_path_from_tuples(sub_vec.clone());
//                            println!(""); // must always println after print_path*
//                            print_path_from_tuples(pc.positions_so_far());
//                            println!("\n*******"); // must always println after print_path*
//                            if sub_vec.iter().any(|previous_pos| *previous_pos == pc.position) {
//                                println!("think this is dead code");
//                                continue;
//                            }

                            to_visit.push(pc.clone());
//                            println!("just pushed potential to actual");
//                            print_path_stack(&to_visit);
                        }
                    },
                    None => panic!("at the disco!")
                }
            }
        }
    }
}
