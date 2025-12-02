use std::thread;
use std::time::Duration;

pub fn init() {
    spawn_bounded_channel();
    spawn_unbounded_channel();
}

fn spawn_bounded_channel() {
    let (tx, rx) = std::sync::mpsc::sync_channel::<String>(10);
    #[cfg(feature = "hotpath")]
    let (tx, rx) = hotpath::channel!((tx, rx), label = "demo-bounded", capacity = 10, log = true);

    thread::spawn(move || {
        let mut counter = 0u64;
        loop {
            let msg = format!("Message {}", counter);
            if tx.send(msg).is_err() {
                break;
            }
            counter += 1;
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        while let Ok(_msg) = rx.recv() {
            thread::sleep(Duration::from_millis(150));
        }
    });
}

fn spawn_unbounded_channel() {
    let (tx, rx) = std::sync::mpsc::channel::<u64>();
    #[cfg(feature = "hotpath")]
    let (tx, rx) = hotpath::channel!((tx, rx), label = "demo-unbounded", log = true);

    thread::spawn(move || {
        let mut counter = 0u64;
        loop {
            if tx.send(counter).is_err() {
                break;
            }
            counter += 1;
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        while let Ok(_value) = rx.recv() {
            thread::sleep(Duration::from_millis(80));
        }
    });
}
