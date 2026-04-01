use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread;

fn cached_squares(inputs: Vec<i32>) -> Vec<i32> {
    let cache: Arc<Mutex<HashMap<i32, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut seen = HashSet::new();
    let mut handles = Vec::new();

    for &x in inputs.iter() {
        if seen.insert(x) {
            let cache = Arc::clone(&cache);
            handles.push(thread::spawn(move || {
                let square = x * x;
                let mut map = cache.lock().unwrap();
                map.insert(x, square);
            }));
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let map = cache.lock().unwrap();
    inputs.iter().map(|x| *map.get(x).unwrap()).collect()
}
