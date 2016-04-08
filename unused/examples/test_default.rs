#[derive(Debug)]
#[derive(Default)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}


fn main() {
    let options: SomeOptions = Default::default();
    println!("{:#?}", options)
}