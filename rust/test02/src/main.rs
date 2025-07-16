#[repr(C)]
struct A {
    a: u8,
    b: u32,
    c: u16,
}

fn main() {
    let raw_str = "EEEE don`t work here: \x3f \u{211D}";

    println!("Hello, {}!", raw_str);
    println!("Hello, world!");

    println!("{:?}", std::mem::size_of::<A>()); // 8
    let _v = A { a: 1, b: 2, c: 3 };

    for number in 1..5 {
        println!("{number}")
    }

    for ch in 'a'..='z' {
        println!("{ch}")
    }

    let s1 = String::from("Hello world");
    let s2 = s1.clone();

    println!("{s1}");
    println!("{s2}");

    let a = 10u32;
    let b = a;

    println!("{a}");
    println!("{b}");
    println!("{a}");
}
