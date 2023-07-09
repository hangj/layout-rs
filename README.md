# layout-lib

View the data layout of a struct.

# Usage

```rust
use layout_lib::Layout;

#[derive(Layout)]
struct A<T> {
    b: u8,
    c: u64,
    d: T,
}

fn main() {
    let layout = A::<Vec<i32>>::get_layout();
    println!("{}", layout);
}
```

The output will be like this

```console
example::A<alloc::vec::Vec<i32>> (size: 40, align: 8)
|  field   | offset |  size  |
| -------- | ------ | ------ |
| c        | 0      | 8      |
| d        | 8      | 24     |
| b        | 32     | 1      |
```

As you can see, the first field of struct A in the layout is *c*, which is not the first declared field(*b*).
That is because Rust does not guarantee the order of the fields in the layout be the same as the order in which the fields are specified in the declaration of the type. see [The Default Representation](https://doc.rust-lang.org/reference/type-layout.html#the-default-representation)

