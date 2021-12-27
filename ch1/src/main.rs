use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
            println!("{}", &region);
    }
}

fn basics() {
    let data = "\
    common name,length (cm)
Little penguin,33
Yellow-eyed penguin,65
Invalid,data
    ";

    let records = data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }

    fn add(i: i32, j: i32) -> i32 {
        i + j
    }

    let e = add(add(1, 2), add(3, 4));
    let a = 1.1 + 2 as f64;
    let b = ((a / 100_000.0).round() as i64) * 100_000;
    let c = 1.1 + 2_f64;
}

fn test_inc_counter() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{} micro second", (1.0 / count as f64) * 1e6);
}

fn control() {
    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}

fn pattern_matching () {
    let needle = 42;
    let haystack = [1, 1, 2, 4, 14, 42];

    for item in &haystack {
        let result = match item {
            42 | 2 => "hit!",
            _ => "miss"
        };

        if result == "hit!" {
            println!("{}: {}", item, result)
        }
    }
}

fn main() {
    // greet_world();
    // basics();
    // control();

    // for _ in 0..=10 {
    //     println!("Hi 11 times..")
    // }
    // test_inc_counter();

    pattern_matching();
}