This repo contains practice code working with closures in Rust. The examples in [The Book](https://doc.rust-lang.org/book/ch13-01-closures.html) were followed closely.

The code in `main.rs` implements two kinds of Cacher structs that have fields for a closure (a simulated expensive calculation in this case), and values. The caller specifies a closure when calling `Cacher::new`, and a Cacher struct is returned.

In the first example, which uses a `match` expression when calling `.value(u32)` on an instance of Cacher, a conditional getter/setter is implemented which either **gets** the existing `Some(v)` value for that arg, or **sets** it. The drawback is that this can only be done for one value per Cacher instance. Calling `cacher_1.value(1)` (calculation takes place here and value is set) followed by `cacher_1.value(2)` (calculation is skipped here) will result in a return of the same value from the first method call.

In the second example - `Cacher2`, a closure is stored in the calculation field of the struct similar to the first Cacher implementation. The field `values` corresponds to a HashMap of K:V pairs that are either get/set via conditional logic concerning whether or not the closure has been called on the argument already.

The third example - `Cacher3`, improves upon `Cacher2` by using generic trait bounds to allow for different types in its closure's function definitions. This allows for a more broad application of the Cacher struct.

#### You can clone, compile, and run this repo by running:

```bash
git clone git@github.com:leetie/closure-cacher.git && cd closure-cacher && cargo run
```
