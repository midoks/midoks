// https://www.runoob.com/rust/rust-lifetime.html
// 生命周期（Lifetimes）是 Rust 中一个独特的概念，用于确保引用始终有效，防止悬垂引用（dangling references）。这是 Rust 内存安全保证的核心部分。
// 基本概念
// 生命周期是 Rust 中引用有效的作用域。Rust 编译器通过生命周期分析来确保所有引用都是有效的。

fn main() {
    let s = "abc".to_string();

    let a = &s;

    println!("{}!", a);
}
