%title: Rust Tutorial
%author: Leif Harald Karlsen
%date: 2017-05-09





->  \_\_\_\_            \_     \_\_\_\_\_      \_             \_       \_  <-
-> |  \_ \\ \_   \_ \_\_\_| |\_  |\_   \_|   \_| |\_ \_\_\_  \_ \_\_(\_) \_\_ \_| | <-
-> | |\_) | | | / \_\_| \_\_|   | || | | | \_\_/ \_ \\| '\_\_| |/ \_\\ | | <-
-> |  \_ <| |\_| \\\_\_ \\ |\_    | || |\_| | || (\_) | |  | | (\_| | | <-
-> |\_| \\\_\\\\\_\_,\_|\_\_\_/\\\_\_|   |\_| \\\_\_,\_|\\\_\_\\\_\_\_/|\_|  |\_|\\\_\_,\_|\_| <-



-> by Leif Harald Karlsen <-
 


-> Code available at *https://github.com/leifhka/rust-talk*

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
 
Rust is developed by Mozilla Research and is heavily influenced
by C, C++ and Haskell.
^

Features (from rust-lang.org):
* zero-cost abstractions
^
* move semantics
^
* guaranteed memory safety
^
* threads without data races
^
* trait-based generics
^
* pattern matching
^
* type inference
^
* minimal runtime
^
* efficient C bindings


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

_Ownership:_

*let x = val* - *x* now owns *val*
^
*let y = x* - then does a bit-wise move of *val* to *y* making *x* unusable
^

_Borrowing:_

*let y = \&x* - *y* now borrows *x*
^
*let y =\&mut x* - *y* now borrows *x* and can mutate it
^

At any point in time we can have:
^
* Any number of immutable borrows ( *\&x* )
^
or
* Exactly one mutable borrow ( *\&mut x* )
^ 
but never both.

^
_Lifetimes:_

* No reference can outlive what it refers to.
^
* The compiler ensures this, so there are no dangling pointers at runtime.
^
* Sometimes, we have to help the compiler by explicitly stating relative lifetimes.

---

-> # Pointer types <-

* *&x* - borrowed immutable reference, data on stack
^
* *&mut x* - (unique) borrowed mutable reference, data on stack
^
* *Box::new(x)* - (unique) owned reference, data on heap
^
* *Rc::new(x)* - shared owned reference, data on heap, reference counting GC
^
* *Weak::new()* - same as Rc, but is not reference counted (allows cycles)
^
* *Arc::new(x)* - thread safe version of Rc

---

-> # Quirks <-

* One cannot implement other people's traits for other people's types.
^
* There are no higher kinded polymorphism (so no Foldable, Functor, Monad traits).
^
* Functions on arrays needs to be implemented for each size.

---

-> # More info <-
 
* Main page: [https://www.rust-lang.org/]
* The Rust Book: [https://doc.rust-lang.org/book/]
* Rust By Example: [http://rustbyexample.com/]

 

-> *Thank you!* <-
