%title: Rust Tutorial
%author: Leif Harald Karlsen
%date: 2017-05-09





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
  * Functions and methods
  * Structs, types and traits
  * High-level functionality
^
* Advanced features
  * Ownership and borrowing
  * Lifetimes
  * Box and Rc
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
^
* introudces alot of zero-cost abstractions

---

-> # Basic Rust <-

* *let x = val* - introduces a new variable *x* with value *val*
^
* *x: t* - states that *x* has type *t*
^
* *mut x* - denotes a mutable variable *x*
^
* *\&x* - denotes a reference to *x*
^
* *&mut x* - denotes a mutable reference to *x*
^
* *fn f(x: t) -> r { body }* - defines a function *f* from *t* to *r*
^
* *struct Pair { x: t1, y: t2 }* - defines a C-like struct
^
* *struct Pair(t1, t2)* - defines a tuple struct
^
* *enum E { X, Y, Z }* - defines an enum/union type
^
* *impl S { fs }* - defines methods *fs* for type *Pair*
^
* *s.f(x)* - calls a method *f* with argument *x* on *s*
^
* *impl T for S { fs }* - implements trait *T* for type *S*
^
* *|x| (x+1)* - defines a lambda function with argument *x*

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
