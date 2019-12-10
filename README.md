# refcells

![](https://github.com/vova616/refcells/workflows/Rust/badge.svg)
    
A library that allows you to get N mutable elements from slices without changing the underlying type, AKA runtime borrow check for slices

## Usage

Use latest nightly
Didn't yet publish to crates.io so

Add this to your `Cargo.toml`:

```toml
[dependencies]
refcells = {version="0.1.3", git = "https://github.com/vova616/refcells"}
```

example:

```rust
use refcells::RefCells;

fn main() {
    let mut arr = [10,20,30,40,50];
    //we going to take 3 different elements from a slice
    //if you are doing something in a loop then this should be inside the loop and not outside
    let view = RefCells::<_, 3>::new(&mut arr);
    let a = view.get_mut(0).unwrap();
    let b = view.get_mut(1).unwrap();
    let c = view.get_mut(2).unwrap();

    *a += *b + *c;
    *b += 2;
    *c += 3;
    
    println!("{:?}", &arr[..])
}
```
