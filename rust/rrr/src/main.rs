use hotwatch::{Event, EventKind, Hotwatch};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch
        .watch(
            "/Users/midoks/Documents/GitHub/midoks/rust/one",
            |event: Event| {
                if let EventKind::Modify(_) = event.kind {
                    println!("War has changed.");
                }
            },
        )
        .expect("failed to watch file!");

    loop {
        sleep(Duration::from_secs(2));
    }
}
