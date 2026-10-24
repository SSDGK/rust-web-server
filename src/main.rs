use rust_web_server::ThreadPool;
use std::{thread, time};
fn main() {
    let tp = ThreadPool::new(3);
    for _ in 0..9 {
        tp.execute(|| {
            thread::sleep(time::Duration::from_millis(2));
        });
    }
    thread::sleep(time::Duration::from_millis(30));
}