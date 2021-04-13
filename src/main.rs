use crypto;
struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn main() {
    let m; let x=6;
    {
	let y = 8;
	let f = Foo{x: &x, y: &y};
	m = f.x;
    }
    println!("Hello, world!{:?}", m);
}
