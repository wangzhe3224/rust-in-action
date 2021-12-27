//! 第四章 生命周期、所有权和借用
//! 这章主要涉及 Rust 编译器最特别，最强大的部分：Borrow Checker
//! Borrow Checking 与三个相关关联的概念相关：
//! - ownership
//! - lifetime
//! - borrow
//! 在 rust 中，一个值如果生命周期结束 或者 不再作用域了，其内存就会被清理
//! Rust 中有两种方式改变值的所有权：赋值 或者 函数调用。
//! 为了减少所有权带来的困扰，我们可以：
//! 借用
//! 拷贝
//! 重构代码，减少long live数据
//! 使用特殊的处理所有权的类型
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("第四章 生命周期、所有权和借用");
    // ex1();
    method_1_use_ref();
    method_2_reduce_long_live_objects();
    method_3_clone();
    method_4_gc();
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct CubeSatV2 {
    id: u64,
    mailbox: MailBox, 
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>
}
type Message = String;

struct GroundStation;


fn ex1() {

    fn check_status(sat_id: i64) -> StatusMessage {
        StatusMessage::Ok
    }

    let sat_a = 0 ;
    let sat_b = 1 ;
    let sat_c = 2 ;

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // wait.....
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}

/// 对于 primitive 变量，Rust默认都实现了 Copy trait，所以他们是 Copy 语义
/// 而自定义变量，默认没有实现 Copy trait，默认是 Move 语义，所以这里出现编译错误
/// 我们可以调整 check status 函数，交换所有权
/// 但是。。这样很麻烦，传递传递去。。而且我们还需要其他返回值。。
fn ex2() {
    fn check_status(sat_id: CubeSat) -> CubeSat {
        // StatusMessage::Ok
        println!("{:?}, {:?}", sat_id, StatusMessage::Ok);
        sat_id
    }

    let sat_a = CubeSat{ id: 0 };
    let sat_b = CubeSat{ id: 1 };
    let sat_c = CubeSat{ id: 2 };

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // wait.....
    let a_status = check_status(a_status);
    let b_status = check_status(b_status);
    let c_status = check_status(c_status);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}

fn method_1_use_ref() {
    impl GroundStation {
        // 这里 self 我们只需要 read only access
        fn send(&self, to: &mut CubeSatV2, msg: Message) {
            to.mailbox.messages.push(msg);
        }
    }   

    impl CubeSatV2 {
        // 这里 self 我们需要 write access 因为我们需要更新数据
        fn recv(&mut self) -> Option<Message> {
            self.mailbox.messages.pop()
        }
    }

    let base = GroundStation{};
    let mut sat_a = CubeSatV2 {
        id: 0,
        mailbox: MailBox {
            messages: vec![]
        }
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("Hello there!"));
    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);

}

fn method_2_reduce_long_live_objects() {
    struct MailBoxV2 {
        messages: Vec<MessageV2>
    }

    #[derive(Debug)]
    struct MessageV2 {
        to: u64,
        content: String,
    }

    struct GroundStationV2;

    impl MailBoxV2 {
        fn post(&mut self, msg: MessageV2) {
            self.messages.push(msg);
        }

        fn deliver(&mut self, recipient: &CubeSat) -> Option<MessageV2> {
            for i in 0..self.messages.len() {
                if self.messages[i].to == recipient.id {
                    let msg = self.messages.remove(i);
                    return Some(msg)
                } 
            }
            None
        }
    }

    impl GroundStationV2 {
        fn connect(&self, sat_id: u64) -> CubeSat {
            CubeSat{
                id: sat_id
            }
        }

        fn send(&self, mailbox: &mut MailBoxV2, msg: MessageV2) {
            mailbox.post(msg)
        }
    }

    impl CubeSat {
        fn recv(&self, mailbox: &mut MailBoxV2) -> Option<MessageV2> {
            mailbox.deliver(&self)
        }
    }

    fn fetch_sat_ids() -> Vec<u64> {
        vec![1,2,3]
    }

    let mut mail = MailBoxV2 { messages: vec![] };
    let base = GroundStationV2{};
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = MessageV2 {
            to: sat_id,
            content: String::from("Hi there. "),
        };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}

fn method_3_clone() {
    #[derive(Debug, Clone, Copy)]
    struct CubeSatV3 {
        id: u64,
    }

    #[derive(Debug, Clone, Copy)]
    enum StatusMessageV3 {
        Ok,
    }

    fn check_status(sat: CubeSatV3) -> StatusMessageV3 {
        StatusMessageV3::Ok
    }

    let sat_a = CubeSatV3 { id: 0 };
    let a_status = check_status(sat_a.clone());
    // println!("a: {:?}", a_status.clone());
    println!("a: {:?}", a_status);

    let a_status = check_status(sat_a.clone());
    // println!("a: {:?}", a_status.clone());
    println!("a: {:?}", a_status);
}

/// 是的，Rust也可以有GC。。。
/// Rc 是不可变的
/// RefCell是可变的
fn method_4_gc() {
    /*
    base: RefCell { value: GroundStationV4 { radio_freq: 87.65 } }
    base2: GroundStationV4 { radio_freq: 75.31 }
    base: RefCell { value: <borrowed> }  这里很有意思，base的值编了。。
    base: RefCell { value: GroundStationV4 { radio_freq: 75.31 } }
     */
    #[derive(Debug)]
    struct GroundStationV4 {
        radio_freq: f64,
    }

    let base = Rc::new(RefCell::new(
        GroundStationV4{
            radio_freq: 87.65,
        }
    ));

    println!("base: {:?}", base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base2: {:?}", base_2);
        println!("base: {:?}", base);
    }

    println!("base: {:?}", base);
}