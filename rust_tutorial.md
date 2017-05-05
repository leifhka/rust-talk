%title: Rust Tutorial
%author: Leif Harald Karlsen
%date: 2016-05-09

# Content

* Placing Rust among other languages
  * Why does Rust exist?
  * Language overview
^
* Simple features
  * Hello World
  * Declarations
  * Type inference
  * Functions
  * Structs and types
  * Methods
  * Traits
^
* Advanced features
  * Ownership and borrowing
  * Life times
  * Box
  * Unsafe
 

---

# Why does Rust exist?


_One normally has to pick one of:_

* No Runtime: (C/C++)
^
  * Low resource consumption, 
^
  * but manual memory management and no memory safety
^
* Runtime: (Java)
^
  * Automatic memory management and memory safety,
^
  * but high resource consumption
^
* No Runtime + cleverness: (Rust)
^
  * Low resource consumption, 
^
  * and automatic memory management and memory safety.

---

# Language overview

Rust:
* is compiled systems programming language,
^
* is strictly and satically typed,
^
* has a struct and trait based type system,
^
* is comparable to C/C++ in speed,
^
* is memory safe,
^
* heavily influenced by C and Haskell

---


