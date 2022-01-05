#[macro_use]
extern crate crossbeam;

use std::thread;
use crossbeam::{channel::unbounded, internal::SelectHandle};

fn main() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(42)
          .unwrap();
    });

    select!{
        recv(rx) -> msg => println!("{:?}", msg),
    }
}


enum Work {
    // order, args
    Task((usize, i32)),
    Finished
}

fn worker_demo() {
    let n_thread = 2;
    let (todo_tx, todo_rx) = unbounded();
    let (results_tx, results_rx) = unbounded();
    
    let tasks = [11, 22, 33, 44];

    for (id, arg) in tasks.iter().enumerate() {
        todo_tx.send(Work::Task((id, *arg))).unwrap();
    }

    for _ in 0..n_thread {
        todo_tx.send(Work::Finished).unwrap();
    }

    for _ in 0..n_thread {
        let todo = todo_rx.clone();
        let results = results_tx.clone();

        thread::spawn(move || {
            loop {
                let task = todo.recv();
                let result = match task {
                    Err(_) => break,
                    Ok(Work::Finished) => break,
                    Ok(Work::Task((id, arg))) => (id, arg),
                };

                results.send(result).unwrap();
            }
        });
    };
}