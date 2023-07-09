use layout_lib::Layout;

#[derive(Layout)]
struct A<T> {
    b: u8,
    c: u64,
    d: T,
}

fn main() {
    let layout = A::<String>::get_layout();
    println!("{}", layout);
}