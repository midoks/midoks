

#[repr(C)]
struct A{
    a: u8,
    b: u32,
    c: u16,
}

fn main() {
    let  raw_str = "EEEE don`t work here: \x3f \u{211D}";

    println!("Hello, {}!",raw_str);
    println!("Hello, world!");


    println!("{:?}", std::mem::size_of::<A>()); // 8
    let v = A{a:1,b:2,c:3};
}
