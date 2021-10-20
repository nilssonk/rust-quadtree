# rust-quadtree
A simple implementation of a [Quadtree](https://en.wikipedia.org/wiki/Quadtree) in Rust.

## Features
* Data to be inserted should implement the *BoxBounded\<T>* trait which enables AABB querying.
* In an attempt to increase reference locality, the nodes themselves only carry a list of indices into a central StableVec which stores the inserted data.
* The subdivision limit, i.e. the number of elements a single node can contain before a node subdivision is triggered, is compile-time configurable through *const generics*.
* A visitation interface is included for convenient tree traversal.
