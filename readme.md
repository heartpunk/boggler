Boggler
=======

Boggler is a pretty simple mostly zero copy boggle solver in Rust. The approach it takes is pretty much the following:

* For each position on the board, do the following.
  * Initialize an ancestor tree with the current position, and the corresponding character. As it is the root, it will not point to any previous node.
  * Put a path corresponding to each neighbor of the current root position followed by the neighbor itself on the stack of paths to explore.
  * While there are still paths to explore, do the following with each path on the stack:
    * Is this path the beginning of a word? Check the trie. If not, short-circuit this path.
    * Does this path overlap itself? If so, short-circuit this path.

The core code reads about as close to the previous pseudocode description as it could. There's a fair bit of other boilerplate (reading wordlist in, debugging functions, etcetera). While I could streamline some of this, I wouldn't learn too much from it, so I've moved on.

Interesting Notes
-----------------

```<a: b, b>``` can be used in lifetime introductions to specify that lifetime ```b``` is at least as long as ```a```.

Rusts rules about borrowing are [stricter than you might think they need to be](http://manishearth.github.io/blog/2015/05/17/the-problem-with-shared-mutability/), and so in some cases where you can convince yourself what you're doing is safe, you'll need to use something like an ```Rc``` or ```RefCell```, or build your own wrapping datastructure with an ```unsafe``` ```impl``` somewhere.

The bright side is, once you learn not to fight the borrow checker, you really can minimize copies! I'm not sure if this lives up to what zero-copy is meant to be, but I'm pretty happy with it.
