use hotwatch::{Event, EventKind, Hotwatch};
use std::{thread::sleep, time::Duration};

use std::env;
use std::path::Path;

fn main() {
    let cur_dir = env::current_dir();
    println!("当前工作路径1:{:?}", cur_dir);
    println!("当前工作路径2:{:?}", cur_dir.expect("不存在").display());

    let mut path = cur_dir.expect("不存在").display();

    println!("当前工作路径1:{:?}", path);

    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch
        .watch(path, |event: Event| {
            if let EventKind::Modify(_) = event.kind {
                println!("War has changed.");
            }
        })
        .expect("failed to watch file!");

    loop {
        sleep(Duration::from_secs(2));
    }
}
