//! 这里可以生成模块自己的doc
use rand::prelude::*;
use std::fmt;
use std::fmt::Display;

fn main() {
    println!("第三章 复合数据结构");
    type_aliases();
    struct_file();
    rust_oo();
    error_handle();
    enum_type();
    more_enum();
    trait_exp();
}

// 类型别名，type aliases
fn type_aliases() {
    type File = String; // File 是一个新的类型

    fn open(f: &mut File) -> bool {
        true
    }

    fn close(f: &mut File) -> bool {
        true
    }

    #[allow(dead_code)]
    fn read(f: &File, save_to: &mut Vec<u8>) {
        unimplemented!();
    }

    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    close(&mut f1);
}

#[derive(Debug)]
/// 三个斜杠也是文档的意思，这些内容会出现在文档首页
pub struct File {
    /// pub 会把这个struct变为共有，暴露给其他模块使用
    /// 而这里的内容会出现在文档的次级目录里面
    name: String,
    /// 这里是data的文档
    // 两个斜杠不是文档。。。不会出现在doc
    data: Vec<u8>, // A vector of plain bytes, 1 byte = 8 bit
}

// 使用 Struct model File
fn struct_file() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    // 借用(Borrow)，而不是移动(Move)
    let f1_name = &f1.name;
    let f1_len = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, &f1_len);

    // 增加一些函数
    fn open(f: &mut File) -> bool {
        true
    }

    fn close(f: &mut File) -> bool {
        true
    }

    fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = f.data.clone();
        let read_length = tmp.len();

        // 为什么要reserve？不可以直接append吗？Vec的长度可变
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }

    let mut f2 = File {
        name: String::from("f2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };
    let mut buffer = vec![];

    open(&mut f2);
    let f2_len = read(&f2, &mut buffer);
    close(&mut f2);

    // 把文件内容读入内存
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", f2.name, f2_len);
    println!("{}", text);
}

// Rust的”面向对象“
fn rust_oo() {
    impl File {
        pub fn new(name: &str) -> File {
            File {
                name: String::from(name),
                data: Vec::new(),
            }
        }

        pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
            let mut f = File::new(name);
            // 为啥这里不是 &f.data? 因为我们需要所有权，不是只读借用，需要更改内部数据
            f.data = data.clone();
            f
        }
    }

    let f3 = File::new("f3.txt");
    let f3_name = &f3.name;
    println!("{:?}", f3)
}

// 错误处理
fn error_handle() {
    fn one_in(denominator: u32) -> bool {
        thread_rng().gen_ratio(1, denominator)
    }

    impl File {
        // 返回一个 Sum Type Result：OK 或者 Error
        // 很有意思，虽然这个 Impl 分布在不同的函数中，他们确实一体的。。
        fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
            let mut tmp = self.data.clone();
            let read_len = tmp.len();
            save_to.reserve(read_len);
            save_to.append(&mut tmp); // 为啥需要一个mut ref？

            Ok(read_len)
        }
    }

    fn open(f: File) -> Result<File, String> {
        if one_in(100_000) {
            let err_msg = String::from("Interrupted by signal!");
            Err(err_msg)
        } else {
            Ok(f)
        }
    }
}

// Sum Type: enum
fn enum_type() {
    #[derive(Debug)]
    enum Event {
        Update,
        Delete,
        Unknown,
    }

    type Message = String;

    fn parse_log(line: &str) -> (Event, Message) {
        // 这里的类型注释是必要的，不过如果不写，编译器会提示要求我们指明类型，还是很友好的
        let parts: Vec<_> = line.splitn(2, ' ').collect();

        if parts.len() == 1 {
            return (Event::Unknown, String::from(line));
        }

        let event = parts[0];
        let rest = String::from(parts[1]);

        match event {
            "UPDATE" | "update" => (Event::Update, rest),
            "DELETE" | "delete" => (Event::Delete, rest),
            _ => (Event::Unknown, rest),
        }
    }

    let log = "BEGINE xxxx\nUPDATE sss\nDELETE cccc\ndelete ddd";

    for line in log.lines() {
        let parse_res = parse_log(line);
        println!("{:?}", parse_res);
    }
}

// enum 还可以内嵌在 struct 中，enum 还可以带有参数
fn more_enum() {
    enum Suit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
    }

    enum Card {
        // 带人的牌
        King(Suit),
        Queen(Suit),
        Jack(Suit),
        Ace(Suit),
        // 普通数字牌，每种 usize 张
        Pip(Suit, usize),
    }

    enum FileStatus {
        Open,
        Closed,
    }

    struct File {
        name: String,
        data: Vec<u8>,
        state: FileStatus,
    }
}

// trait
fn trait_exp() {
    #[derive(Debug)]
    struct File2;

    trait Read {
        fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
    }

    impl Read for File2 {
        fn read(self: &File2, save_to: &mut Vec<u8>) -> Result<usize, String> {
            Ok(0)
        }
    }

    let f = File2 {};
    let mut buffer = vec![];
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} bytes read from {:?}", n_bytes, f);

    // 例子：Display trait
    enum FileStatus2 {
        Open,
        Closed,
    }

    impl Display for FileStatus2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                FileStatus2::Open => write!(f, "Open"),
                FileStatus2::Closed => write!(f, "Closed"),
            }
        }
    }

    let f_s = FileStatus2::Open;
    println!("{}", f_s);
}
