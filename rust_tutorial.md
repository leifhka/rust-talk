%title: Rust Tutorial
%author: Leif Harald Karlsen
%date: 2016-05-09





->  \_\_\_\_            \_     \_\_\_\_\_      \_             \_       \_  <-
-> |  \_ \\ \_   \_ \_\_\_| |\_  |\_   \_|   \_| |\_ \_\_\_  \_ \_\_(\_) \_\_ \_| | <-
-> | |\_) | | | / \_\_| \_\_|   | || | | | \_\_/ \_ \\| '\_\_| |/ \_\\ | | <-
-> |  \_ <| |\_| \\\_\_ \\ |\_    | || |\_| | || (\_) | |  | | (\_| | | <-
-> |\_| \\\_\\\\\_\_,\_|\_\_\_/\\\_\_|   |\_| \\\_\_,\_|\\\_\_\\\_\_\_/|\_|  |\_|\\\_\_,\_|\_| <-


-> by Leif Harald Karlsen <-
 
---


-> # Content <-

* Placing Rust among other languages
  * Why does Rust exist?
  * Language overview
^
* Simple features
  * Declarations
  * Structs and types
  * Functions and methods
  * Traits
  * Borrowing
^
* Advanced features
  * More on ownership and borrowing
  * Lifetimes
  * Box
  * Unsafe

---

-> # Why does Rust exist? <-


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
^

*Rust is a safe systems programming language*

---

-> # Language overview <-

Rust:
* is compiled systems programming language,
^
* is strictly and statically typed,
^
* has a struct and trait based type system,
^
* is comparable to C/C++ in speed,
^
* is memory safe,
^
* heavily influenced by C and Haskell

---

-> #Rules <-

_Borrowing:_

At any point in time we can have:
^
* Any number of immutable borrows ( *\&x* )
^
* but no mutable borrows ( *\&mut x* )
^
or
* Exactly one mutable borrow ( *\&mut x* )
^
* but no immutable borrows ( *\&x* )
^ 

_Lifetimes:_

* No reference can outlive what it refers to.
^
* The compiler ensures this, so there are no dangling pointers at runtime.
^
* Sometimes, we have to help the compiler by explicitly stating relative lifetimes.
