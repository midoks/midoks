


fn main() {
    let  raw_str = "EEEE don`t work here: \x3f \u{211D}";

    println!("Hello, {}!",raw_str);
    println!("Hello, world!");
}
