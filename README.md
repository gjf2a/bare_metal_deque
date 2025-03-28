# bare_metal_deque

## Deque for use with `no-std` Rust

The `BareMetalDeque` represents a fixed-size double-ended queue analogous to [VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html). It is implemented internally as a ring buffer.

There are numerous other implementations of this concept available. None 
quite met my own needs. Here is the combination of features that distinguishes this particular implementation:

Key features:
* Runs in `no-std` Rust projects.
* No other dependencies.
* No `unsafe` code.
* Can be indexed (mutably and otherwise).
* Can be iterated.
* Implements the `Copy` and `Clone` traits.
* Expects its object type to implement the `Default`, `Copy`, and `Clone` traits.