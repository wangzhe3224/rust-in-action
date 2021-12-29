//! 指针的类型是 usize （int）
//! reference, pointer, address的区别
//! reference，引用，是Rust的一个抽象，通常是一个指针或者一个指针+一个整数
//! pointer，指针，是高级语言，比如C，的一个抽象，是一个地址，改地址是内存中的一个位置
//! address，地址，是内存中的一个Byte的起始位置
//! pointer比address多了一个信息，就是类型，比如我们会说一直指向i32的指针，那么这个指针不仅包含了起始地址，
//! 他还会告诉编译器，需要encode 4个字节来获取正确的i32数值。但是，尽管如此，程序员需要保证在运行时，指针
//! 指向的内存空间的类型是正确的
//! reference，则不同，ref可以确保在运行时也指向合法的数据，而且，ref还可以确保内存区域是usize对齐的
//! 这样CPU就会很开心的读取数据。
use std::mem::size_of;
use std::borrow::Cow; // 只能指针类型，可以直接读取指向内容而不拷贝，Copy on Write？
use std::ffi::CStr;   // C-like string
use std::os::raw::c_char;  // c_char alias for i8 type


fn main() {
    println!("第六章 内存");
    // listing_1();

    // 创造一个 raw pointer
    let a: i64 = 42;
    let a_ptr = &a as *const i64;

    println!("a: {} ({:p})", a, a_ptr);
    // 通过指针从内存中读取数据的过程成为：dereference a point
    let a_addr: usize = unsafe {
        std::mem::transmute(a_ptr)
    };
    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);

    // 任何数值都可以被编译器看做一个指针
    let ptr = 42 as *const Vec<String>;
    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }

    stack_and_heap();

    // 虚拟内存
    // virtual_memory();
    vm_2();
}


fn listing_1() {
    static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
    static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

    fn func() {
        let a = 42;
        let b = &B;
        let c = &C;

        // {:p} 打印指针指向的地址
        println!("a: {}, b: {:p}, c: {:p}", a, b, c);
    }

    func();

    fn func2() {
        let a: usize = 42;  // usize 就是内存地址的最小单位
        let b: &[u8; 10] = &B;
        let c: Box<[u8]> = Box::new(C);  // Boxed byte slice，这里C的所有权转移到了变量c

        println!("a (an unsigned integer):");
        println!("  location: {:p}", &a);
        println!("      size: {:?}", size_of::<usize>());
        println!("     value: {:?}", a);
        println!("");

        println!("b (a reference to B):");
        println!("  location: {:p}", &b);
        println!("      size: {:?}", size_of::<&[u8;10]>());
        println!(" points to: {:p}", b);
        println!("");

        println!("c (a Box for C):");
        println!("  location: {:p}", &c);
        println!("      size: {:?}", size_of::<Box<[u8]>>());
        println!(" points to: {:p}", c);
        println!("");

        println!("B (an array of 10 bytes):");
        println!("  location: {:p}", &B);
        println!("      size: {:?}", size_of::<&[u8;10]>());
        println!("     value: {:?}", B);
        println!("");

        println!("C (an array of 11 bytes):");
        println!("  location: {:p}", &C);
        println!("      size: {:?}", size_of::<&[u8;11]>());
        println!("     value: {:?}", C);
        println!("");
    }

    func2();

    /// *const T 和 *mut T 分别是 raw pointer
    /// &T 和 &mut T 分别是 reference
    fn func3() {
        let a = 42;
        let b: String;
        let c: Cow<str>;

        unsafe {
            // reference 不能被直接cast成*mut T
            let b_ptr = &B as *const u8 as *mut u8;
            b = String::from_raw_parts(b_ptr, 10, 10);

            let c_ptr = &C as *const u8 as *const c_char;
            c = CStr::from_ptr(c_ptr).to_string_lossy();
        }

        println!("a: {}, b: {}, c: {}", a, b, c);
    }

    func3();
}

/// Raw Pointer 大多数时间不是最优解
/// Rust提供了很多Wrapper types来包裹 Row Pointer，让他们更加安全
fn rust_pointer_family() {

}

/// 堆 和 栈
/// 实现了 Size trait 的类型通常可以stack allocated
fn stack_and_heap() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let r1 = *a + *b + *c;

    std::mem::drop(a);
    let d = Box::new(1);
    let r2 = *d + *b + *c;

    println!("{} {}", r1, r2);
}


/// 一些关于虚拟内存的名词
/// Page，分页，一个固定大小内存区块，通常为4kb（64位系统）
/// Word，词，指针的大小，与CPU寄存器的大小（宽度）相关，Rust中就是 usize 和 isize 表示词的长度的类型
/// Page Fault，页错误，当CPU无法在物理存储中找到一个地址的时候，会抛出页错误信号，然后OS会尝试从硬盘读取
/// Swapping，交换，把内存中的内容临时放到硬盘中
/// Virtual Memory，虚拟内存，OS提供给应用程序的内存空间
/// Real Memory，真实内存，OS看到的当前可用的物理内存
/// Page Table，OS维护的一个数据结构，用来把虚拟内存地址翻译成真是内存地址
/// Segment, 段，虚拟内存的一个区块。为了方便在虚拟内存和物理内存地址之间翻译，虚拟内存会按照段大小作为最小分割
/// Segment Fault，段错误，CPU会抛出段错误，如果请求一个非法的内存地址
/// MMU，一个帮助CPU进行内存地址翻译的硬件组件
fn virtual_memory(){
    let mut n_nonzero = 0;

    for i in 1..10_000 {
        let ptr = i as *const u8;
        println!("{:p}", ptr);
        // Segment Fault，因为0x1不是一个合法的内存区域
        let byte_at_addr = unsafe { *ptr };
        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}

fn vm_2() {

    static GLOBAL: i32 = 1000;

    fn noop() -> *const i32 {
        let noop_local = 12345;
        &noop_local as *const i32
    }

    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);

    let fn_int = noop();

    println!("GLOBAL:    {:p}", &GLOBAL as *const i32);
    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const i32);
    println!("boxed_int: {:p}", Box::into_raw(boxed_int));
    println!("boxed_str: {:p}", Box::into_raw(boxed_str));
    println!("fn_int:    {:p}", fn_int);
}