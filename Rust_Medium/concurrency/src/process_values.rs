use std::sync::mpsc;
use std::thread;

fn process_values(values: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for value in values {
            tx.send(value * 2).unwrap();
        }
    });

    let mut results = Vec::new();
    for received in rx {
        results.push(received);
    }

    handle.join().unwrap();
    results
}
