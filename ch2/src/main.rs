use num::complex::Complex;
use std::convert::TryInto;
use regex::Regex;
use std::time::Duration;
use std::vec;
use clap::{App, Arg};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;

fn main() {
    println!("Hi Ch2");
    // 命令行参数
    // command_line_args();

    // 数字
    number();
    // 控制流
    flow_control();
    // 函数
    function_def();
    // 第二章项目
    project_ch2();
    // 函数进阶
    advance_function();
    // 项目2
    project_grep();
    // Array
    array_ex();
    // Slice
    slice_ex();
    // Vector
    vector_ex();
    // 第三方库使用
    third_party();
}

// 1. 数字
fn number() {
    // 1.1 整数和浮点数
    fn number_integer_float() {
        // 三种不同的指定类型的方法
        let twenty = 20; // 类型推断
        let twenty_one: i32 = 21;
        let twenty_two = 21_i32;
        // 跟 Python 一样的
        let any = 100_100_100;

        let addition = twenty + twenty_one + twenty_two;
        println!(
            "{} + {} + {} = {}",
            twenty, twenty_one, twenty_two, addition
        );

        let forty_twos = [42.0, 42f32, 42.0_f32];

        println!("{:02}", forty_twos[0]);
    }

    // 1.2 整数的不同进制
    fn number_base() {
        let three = 0b11; // 二进制
        let thirty = 0o36; // 8进制
        let three_hundred = 0x12C; // 16进制

        println!("base 10: {} {} {}", three, thirty, three_hundred);
        println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
        println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
        println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
    }

    // 1.3 数字的比较
    fn number_compare() {
        let a = 10;
        let b = 12u16;

        if a < (b as i32) {
            // 需要转换类型，否则无法比较，编译器不开心 ：<
            println!("hoo");
        }
        // 但是类型转换不是很好的做法，有时候还会出现意想不到的结果：比如
        // 300_i32 as i8 返回 44。。。降级会出现问题。

        /* use std::convert::TryInto; 这句将 TryInto Trait 引入了空间，b就可以使用try_into了。
        // 使用 try_into() 更加安全，就是写起来很啰嗦。。。
        // 以后我们在具体讲 trait 的事情，但是 Rust 的一个特点之一就是可以通过引入不同的trait到
        命名空间来增加类型的函数。
        */
        let b_ = b.try_into().unwrap();
        if a < b_ {
            println!("hoo, hoo");
        }

        // 浮点数的精度和比较
        let a = 0.1f32;
        let b = 0.2;
        let c = a + b;
        let diff = (c - 0.3).abs();
        assert!(diff <= f32::EPSILON);
    }

    // 1.4 有理数、复数和其他的数字
    fn number_others() {
        let a = Complex::new(2.1, -1.2);
        let b = Complex::new(11.1, 22.2);
        let res = a + b;

        println!("{} + {}i", res.re, res.im);
    }

    // 调用例子
    number_integer_float();
    number_base();
    number_compare();
    number_others();
}

// 2. 控制流
fn flow_control() {
    // 2.1 for 循环
    fn flow_for() {
        /*
        1. for item in collection    所有权转移，Ownership
        2. for item in &collection   只读借用，Borrow
        3. for item in &mut collection  读写借用，Borrow
         */
        let vec = [1, 2, 3];

        for item in &vec {
            println!("{}", item)
        }

        // 当我们不关心变量名的时候
        for _ in 0..10 {
            ////////////////////////////////
        }

        // 这种是不推荐的方式
        for i in 0..vec.len() {
            // continue 可以跳过后面的代码
            if i % 2 == 0 {
                continue;
            }
        }
    }

    fn flow_while() {
        println!("Flow - while loop");
        let mut samples = vec![];

        while samples.len() < 10 {
            let sample = 1;
            samples.push(sample);
        }
    }

    fn flow_loop() {
        // loop 比 while true 更好
        let mut a = 0;
        loop {
            if a < 10 {
                a += 1;
                continue;
            }
            break;
        }
        println!("Finish loop.")
    }

    // 2.2 if 分支
    fn flow_condition() {
        let a = 1;
        if a < 0 {
            //
        } else if a > 2 {
            //
        } else {
            //
        }
    }

    // 2.3 模式匹配
    fn flow_pattern_match() {
        let needle = 42;
        let haystack = [1, 1, 2, 3, 5, 14, 42, 132];

        for item in &haystack {
            let result = match item {
                42 | 132 => "hit", // 注意这里是逗号不是分号
                _ => "miss",
            };

            if result == "hit" {
                println!("{} : {}", item, result);
            }
        }
    }

    flow_for();
    flow_while();
    flow_loop();
    flow_condition();
    flow_pattern_match();
}

// 3. 函数
fn function_def() {
    // 类型注释是必须的，这一点我很奇怪，为什么编译器可以类型推断，却强迫写类型签名？
    fn add(i: i32, j: i32) -> i32 {
        i + j
    }

    let a = add(1, 2);
    println!("{}", a);
}

// Project 1:
fn project_ch2() {
    fn calculate_mandelbrot(
        max_iters: usize,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        width: usize,
        height: usize,
    ) -> Vec<Vec<usize>> {
        let mut rows = Vec::with_capacity(width);

        for img_y in 0..height {
            let mut row: Vec<usize> = Vec::with_capacity(height);
            for img_x in 0..width {
                let x_percent = img_x as f64 / width as f64;
                let y_percent = img_y as f64 / height as f64;
                let cx = x_min + (x_max - x_min) * x_percent;
                let cy = y_min + (y_max - y_min) * y_percent;
                // compute
                let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
                // println!("{} {} {} ", x_percent, y_percent, escaped_at);
                row.push(escaped_at);
            }

            rows.push(row);
        }

        rows
    }

    fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
        let mut z = Complex { re: 0.0, im: 0.0 };
        let c = Complex::new(cx, cy);

        for i in 0..=max_iters {
            if z.norm() > 2.0 {
                return i;
            }
            z = z * z + c
        }
        // 发散了，没找到解答
        max_iters
    }

    fn render_mandelbrot(escaped_vals: Vec<Vec<usize>>) {
        for row in escaped_vals {
            let mut line = String::with_capacity(row.len());
            for column in row {
                let val = match column {
                    0..=2 => ' ',
                    3..=5 => '.',
                    6..=10 => '。',
                    11..=30 => '*',
                    31..=100 => '+',
                    101..=200 => 'X',
                    201..=400 => '$',
                    401..=700 => '#',
                    _ => '%',
                };
                // println!("{}", column);
                line.push(val)
            }
            println!("{}", line);
        }
    }

    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 50);

    render_mandelbrot(mandelbrot);
}

fn advance_function() {
    // 生命周期的表达方式
    // 'a 'b 是 生命周期变量， &'a 赋予 变量i a生命周期
    // 多数情况下，rust编译器会推断生命周期，不需要显式的写出来
    fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
        *i + *j
    }

    // 泛型 和 Trait
    fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
        i + j
    }

    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let duration = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", duration);
}

// Project 2: simple grep
fn project_grep() {
    let search_term = "picture";
    let quote = "\
    Every face, ever shop, bedroom window, public-house, and 
    dark square is a picture feverishly turned--in search of what?
    It is the same with books.
    What do we seek through millions of pages?";

    // 只有闭包可以捕捉上下文的变量
    // 普通函数不行，这点跟Python不同
    let v1 = || {
        println!("This is v1 of grep");
        for line in quote.lines() {
            if line.contains(search_term) {
                println!("{}", line);
            }
        }
    };

    let v2 = || {
        println!("This is v2 of grep");
        let mut line_num: usize = 1;
        for line in quote.lines() {
            if line.contains(search_term) {
                println!("{}: {}", line_num, line)
            }
            line_num += 1;
        }
    };

    let v3 = || {
        // 同时访问 index 和 内容
        // Python: for idx, item in enumerate(string)
        println!("This is v3 of grep");
        for (i, line) in quote.lines().enumerate() {
            if line.contains(search_term) {
                println!("{}: {}", i + 1, line);
            }
        }
    };

    v1();
    v2();
    v3();
}

// Array
// Rust中的Array就是一个连续的相同类型数据的内存空间
// Array 的类型很有意思的，需要两个部分：[type; length]
// 换句话说，Array的长度是固定的，而且是固定在类型中，不同长度的Array的类型都是不同的
fn array_ex() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3]; // [0, 0, 0]
    let blank2: [u8; 3] = [0; 3];

    // 嵌套的类型
    let arrays = [one, two, blank1, blank2];

    // 遍历方法：.iter or &items
    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n+10);
        }
        
        // 不推荐这种，但是可以
        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t {:?} {}", a, sum);
    }
}

// Slices，切片
// 切片是动态长度的类似array的数据结构
// 切片的类型只有一个部分
fn slice_ex() {
    let a = [1,2,3];
    let a_s = &a[0..2];  // Python: a[0:2]

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

// Vector
// Vector Vec<T> 是大小可变的列表
fn vector_ex() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
    haha,
    bedroom, twenty_one, twenty_two,
    good.
    ";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2*ctx_lines+1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

// Third party library ( crate )
fn third_party() {
    let re = Regex::new("picture").unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

// command line args
fn command_line_args() {
    let args = App::new("ch2")
        .version("0.1")
        .about("chapter 2")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    println!("{}", pattern);
}

// 文件读写
fn file_read() {
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new(); // heap allocation

    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);
        line.truncate(0);  // buffer重置
    }

    // 或者直接读取buffer
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => ()
        }
    }
}

// io
fn io_read() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let pattern = Regex::new("test").unwrap();
    process_lines(reader, pattern);
}