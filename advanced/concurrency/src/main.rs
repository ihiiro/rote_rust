use std::time::Duration;
use std::thread;
use std::sync::{mpsc, Mutex, Arc}; // Arc: atomic reference counter and is thread-safe

fn main() {
    println!();
    // span new threads
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread 1 says: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread says: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![10, 11, 12, 13, 14, 15];

    let handle2 = thread::spawn(move || { //move keyword gives closure ownership of used values
        println!("spawned thread 2 says: {:?}", v);
    });

    // blocks any other thread from continuing until this thread finishes
    handle.join().unwrap();
    handle2.join().unwrap();

    // destructure returned tuple
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec!["hi", "there!", "How", "are", "you?"];
        for val in vals {
            tx1.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let vals = vec!["I", "am", "fine", "Thank you!"];
        for val in vals {
            tx.send(val).unwrap();
        }
    });

    println!();

    for received in rx {
        println!("main thread received `{}` from another thread", received);
    }

    // mutexes (Mutual Exclusion)
    println!();
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    //
    println!();
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            *counter.lock().unwrap() += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter = {}", counter.lock().unwrap());
}
