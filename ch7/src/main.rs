//! 文件和存储
use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::{Serialize};

use std::io::prelude::*;
use std::fs::File;
use std::env;

const BYTES_PER_LINE: usize = 16;

/// 文件跟内存不同，文件比内存更加整体和格式化。
fn main() {
    println!("第七章 文件和存储");
    // file_format();

    let arg1 = env::args().nth(1);

    // let fname = arg1.expect("usage: fview FILENAME");
    let fname = match arg1 {
        Some(f) => f,
        None => "./src/main.rs".to_string(),
    };

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    // read_exact: 来自 Read trait，把内容读入buffer直到buffer满
    while let Ok(_) = f.read_exact(&mut buffer) {
        println!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".   "),
                0xff => print!("##  "),
                _ => print!("{:02x} ", byte),
            }
        }

        println!("");
        pos += BYTES_PER_LINE;
    }
}

/// 创建文件格
fn file_format() {
    #[derive(Serialize)]
    struct City {
        name: String,
        population: usize, 
        latitude: f64,
        longitude: f64,
    }

    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33
    };

    let as_json = to_json(&calabar).unwrap();
    let as_cbor = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();

    println!("json:\n{}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("bincode:\n{:?}\n", &as_bincode);
}

/// 多行字符串的写法: r##, b means binary
fn hex_dump() {
    const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello World!");
}"#;
}


/// 文件，File，是操作系统提供的一个抽象，他是一个包含名字和字节层级的 Facade。
/// 文件也提供了安全性能，比如权限。
/// std::fs::File 是 Rust 提供的关于文件系统的模块
/// std::path::{Path, PathBuf} 分别对应 str 和 String，不过提供了一些路径分隔符相关的操作
fn file_ops() {

}
