fn add2(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    let x = 100;
    let y = 100;
    // &str is a slice (&[u8]) that always points to a valid UTF-8 sequence.
    let s1 : &str = "Hello World";
    // String is heap allocated, growable and not null terminated.
    let s2 : String = "Hello World";
    println!("{}", add2(x, y));
    println!("Hello World");
}