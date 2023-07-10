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

The output will be something like this

```console
example::A<alloc::vec::Vec<i32>> (size: 40, align: 8)
|  field   | offset |  size  |    type    |
| -------- | ------ | ------ | ---------- |
| c        | 0      | 8      | u64 (align: 8) |
| d        | 8      | 24     | alloc::vec::Vec<i32> (align: 8) |
| b        | 32     | 1      | u8 (align: 1) |
```

As you can see, the first field of struct A in the layout is *c*, which is not the first declared field(*b*).
That is because Rust does not guarantee the order of the fields in the layout be the same as the order in which the fields are specified in the declaration of the type. see [The Default Representation](https://doc.rust-lang.org/reference/type-layout.html#the-default-representation)


# The offset calculation
The offset of the field is simply calculated by this macro

```rust
#[macro_export]
macro_rules! offset_of_struct {
    ($struct_name: ty, $field_name: ident) => {
        {
            let p = 0 as *const $struct_name;
            unsafe {&(*p).$field_name as *const _ as usize}
        }
    };
}
```

```rust
let offset = offset_of_struct!(A<Vec<i32>>, b); // 32
```