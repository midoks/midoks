use std::fmt::{self, Formatter, Write};

// 自定义 Formatter
struct MyFormatter;
impl MyFormatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        println!("{}", s);
        Ok(())
    }
}

// FormatterSink 定义
struct FormatterSink<'a, 'b: 'a> {
    f: &'a mut Formatter<'b>,
}

// 为 FormatterSink 实现 Write trait
impl<'a, 'b: 'a> Write for FormatterSink<'a, 'b> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.f.write_str(s)
    }
}

fn main() {
    let mut formatter = MyFormatter;
    let mut sink = FormatterSink { f: &mut formatter };
    write!(sink, "Formatted: {}", 42).unwrap();
}
