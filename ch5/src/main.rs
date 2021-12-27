use std::mem::transmute;

fn main() {
    println!("第五章 数据！");
    listing_5_1();
    listing_5_2();
    endianness();
    floating_f32();
    deconstruct_f32();
    fix_point_float();
    mock_rand_ex();
    project_v1();
    project_v2();
    project_v3();
}

fn listing_5_1() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
    /*
    a: 1100001111000011 50115
    b: 1100001111000011 -15421
     */
}

fn listing_5_2() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("{}", b);
    assert_eq!(a, b);
}

fn endianness() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };

    println!("{} vs {}", a, b);
}

fn floating_f32() {
    // x xxxxxxxx xx..22..xx
    // sign exponent significand(mantissa)
    // n = (-1)^sign * mantissa * 2 ^ exponent
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let sign_bit = n_bits >> 31;
    let exponent_ = n_bits >> 23;
    let exponent_ = exponent_ & 0xff;
    let exponent = (exponent_ as i32) - 127;

    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    println!("{} {} {}", sign_bit, exponent, mantissa);
}

fn deconstruct_f32() {
    fn to_parts(n: f32) -> (u32, u32, u32) {
        let bits = n.to_bits();

        let sign = (bits >> 31) & 1; // 符号位，第一bit
        let exponent = (bits >> 23) & 0xff; // 中间8位
        let fraction = bits & 0x7fffff; // 最后23位

        (sign, exponent, fraction)
    }

    fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
        let signed_1 = (-1.0_f32).powf(sign as f32);

        let exponent = (exponent as i32) - 127;
        let exponent = (2 as f32).powf(exponent as f32);

        let mut mantissa = 1.0;

        for i in 0..23 {
            let mask = 1 << i;
            let one_at_bit_i = fraction & mask;
            if one_at_bit_i != 0 {
                let i_ = i as f32;
                let weight = 2_f32.powf(i_ - 23.0);
                mantissa += weight;
            }
        }

        (signed_1, exponent, mantissa)
    }

    fn from_parts(
        // <10>
        sign: f32,
        exponent: f32,
        mantissa: f32,
    ) -> f32 {
        sign * exponent * mantissa
    }

    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field    |  as bits | as real number");
    println!("sign     |        {:01b} | {}", sign, sign_);
    println!("exponent | {:08b} | {}", exp, exp_);
    println!("mantissa | {:023b} | {}", frac, mant);
}

// std::convert ::From
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-127)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}

fn fix_point_float() {
    // n.0 是啥？
    let a = Q7(20);
    println!("{}", a.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.0));
    }
}

fn mock_rand_ex() {
    fn mock_rand(n: u8) -> f32 {
        // 类似某种 hash
        let base: u32 = 0b0_0111110_000000000000000000000000;
        let large_n = (n as u32) << 15;
        let f32_bits = base | large_n;
        let m = f32::from_bits(f32_bits);

        2.0 * (m - 0.5)
    }

    println!("{}", mock_rand(0xff));
}

/// 感受一下 函数 也是 数据
/// CHIP-8 模拟器
fn project_v1() {
    struct CPUV1 {
        current_operation: u16, // 16bit wide
        registers: [u8; 2],     // 我们只有两个1字节的寄存器
    }

    impl CPUV1 {
        fn read_opcode(&self) -> u16 {
            self.current_operation
        }

        /// ADD: 0x4
        fn add_xy(&mut self, x: u8, y: u8) {
            self.registers[x as usize] += self.registers[y as usize];
        }

        fn run(&mut self) {
            // loop {
            let opcode = self.read_opcode();

            // decode 操作码
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
            // }
        }
    }

    let mut cpu = CPUV1 {
        current_operation: 0,
        registers: [0, 0],
    };

    cpu.current_operation = 0x8014;
    // 8: 意味着需要两个寄存器 | 0，1代表两个寄存器 | 4 代表加法
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();
    println!("5 + 10 = {}", cpu.registers[0]);
}

fn project_v2() {
    /// 16个1字节（8bit）寄存器，一个Program Counter，4kb 内存。其中前512kb保留做系统内存，0x0 ~ 0x100
    /// 注意，opcpde仍然是16bit宽，但是我们的内存时8bit宽，我们需要连续读入两个连续的内存获得opcode
    struct CPUV2 {
        registers: [u8; 16], // 我们有16个1字节的寄存器, 我们可以用 0x0 ~ 0xF 代表他们
        position_in_memory: usize, // PC, Program Counter
        memory: [u8; 0x1000], // 4kb 内存
    }

    impl CPUV2 {
        fn read_opcode(&self) -> u16 {
            let p = self.position_in_memory;
            let op_byte1 = self.memory[p] as u16;
            let op_byte2 = self.memory[p + 1] as u16;

            op_byte1 << 8 | op_byte2
        }

        /// ADD: 0x4
        fn add_xy(&mut self, x: u8, y: u8) {
            let arg1 = self.registers[x as usize];
            let arg2 = self.registers[y as usize];

            let (val, overflow) = arg1.overflowing_add(arg2);
            self.registers[x as usize] = val;

            if overflow {
                self.registers[0xF] = 1;
            } else {
                self.registers[0xF] = 0;
            }
        }

        fn run(&mut self) {
            loop {
                let opcode = self.read_opcode();
                self.position_in_memory += 2; // 更新PC位置

                // decode 操作码
                let c = ((opcode & 0xF000) >> 12) as u8;
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                let d = ((opcode & 0x000F) >> 0) as u8;

                println!("{} {} {} {}", c, x, y, d);

                match (c, x, y, d) {
                    (0, 0, 0, 0) => {
                        return;
                    } // 注意这里，会直接跳出循环
                    (0x8, _, _, 0x4) => self.add_xy(x, y),
                    _ => todo!("opcode {:04x}", opcode),
                }
            }
        }
    }

    let mut cpu = CPUV2 {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    // opcode: 0x8014
    mem[0] = 0x80;
    mem[1] = 0x14;
    // opcode: 0x8024
    mem[2] = 0x80;
    mem[3] = 0x24;
    // opcode: 0x8034
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run(); // 思考：为什么这里loop会退出？

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}

/// 加入stack实现函数调用
/// 1. 如何把函数读入内存？
/// 2. 如何试实现函数调用？
/// 调用函数需要三步：把当前PC存入Stack；增加SP；把PC指向函数入口地址
/// 函数返回需要三步：递减SP；取回之前的PC地址；把PC指向之前的PC地址
/// 函数会把返回值存入寄存器
fn project_v3() {
    /// 为什么 我们可以直接修改 memory 的值？
    struct CPUV3 {
        registers: [u8; 16], // 我们有16个1字节的寄存器, 我们可以用 0x0 ~ 0xF 代表他们
        position_in_memory: usize, // PC, Program Counter
        memory: [u8; 0x1000], // 4kb 内存
        stack: [u16; 16],    // 思考：为什么stack的宽度是16bit，而不是8？
        stack_pointer: usize,
    }

    impl CPUV3 {
        fn read_opcode(&self) -> u16 {
            let p = self.position_in_memory;
            let op_byte1 = self.memory[p] as u16;
            let op_byte2 = self.memory[p + 1] as u16;

            op_byte1 << 8 | op_byte2
        }

        /// ADD: 0x4
        fn add_xy(&mut self, x: u8, y: u8) {
            let arg1 = self.registers[x as usize];
            let arg2 = self.registers[y as usize];

            let (val, overflow) = arg1.overflowing_add(arg2);
            self.registers[x as usize] = val;

            if overflow {
                self.registers[0xF] = 1;
            } else {
                self.registers[0xF] = 0;
            }
        }

        /// add_twice
        /// 0x8014   add 
        /// 0x8014   add
        /// 0x00EE   ret
        fn load_add_function(&mut self, addr: u16) {
            let mem = &mut self.memory;
            let addr = addr as usize;
            mem[addr    ] = 0x80;
            mem[addr + 1] = 0x14;
            mem[addr + 2] = 0x80;
            mem[addr + 3] = 0x14;
            mem[addr + 4] = 0x00;
            mem[addr + 5] = 0xEE;
        }


        /// 调用函数需要三步：把当前PC存入Stack；增加SP；把PC指向函数入口地址
        /// * `addr` 函数入口
        fn call(&mut self, addr: u16) {
            let sp = self.stack_pointer;
            let stack = &mut self.stack;

            if sp > stack.len() {
                panic!("Stack Overflow")
            }

            stack[sp] = self.position_in_memory as u16;
            self.stack_pointer += 1;
            self.position_in_memory = addr as usize;
        }

        /// 函数返回需要三步：递减SP；取回之前的PC地址；把PC指向之前的PC地址
        fn ret(&mut self) {
            if self.stack_pointer == 0 {
                panic!("Stack underflow...")
            }

            self.stack_pointer -= 1;
            let call_addr = self.stack[self.stack_pointer];
            self.position_in_memory = call_addr as usize;
        }

        fn run(&mut self) {
            loop {
                let opcode = self.read_opcode();
                self.position_in_memory += 2; // 更新PC位置

                // decode 操作码
                let c = ((opcode & 0xF000) >> 12) as u8;
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                let d = ((opcode & 0x000F) >> 0) as u8;

                let nnn = opcode & 0x0FFF; // 0x2000
                println!("{} {} {} {}", c, x, y, d);

                match (c, x, y, d) {
                    (0, 0, 0, 0) => {
                        return;
                    } // 注意这里，会直接跳出循环
                    (0, 0, 0xE, 0xE) => self.ret(),
                    (0x2, _, _, _) => self.call(nnn),
                    (0x8, _, _, 0x4) => self.add_xy(x, y),
                    _ => todo!("opcode {:04x}", opcode),
                }
            }
        }
    }

    let mut cpu = CPUV3 {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    /// add_twice
    /// 0x8014   add 
    /// 0x8014   add
    /// 0x00EE   ret
    fn load_add_function(mem: &mut [u8; 4096], addr: u16) {
        let addr = addr as usize;
        mem[addr    ] = 0x80;
        mem[addr + 1] = 0x14;
        mem[addr + 2] = 0x80;
        mem[addr + 3] = 0x14;
        mem[addr + 4] = 0x00;
        mem[addr + 5] = 0xEE;
    }

    // 把函数写入内存,0x100
    // let mem = &mut cpu.memory;
    // 这两种都可以
    // cpu.load_add_function( 0x100);
    load_add_function(&mut cpu.memory, 0x100);
    println!("DEBUG: function memory {:?}", &cpu.memory[0x100..0x106]);

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // opcode: 0x2100, 调用地址入口在0x100的函数
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    // opcode: 0x2100, 调用地址入口在0x100的函数
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    // opcode: 0x0000, HALT
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    // // opcode: 0x8014, ADD reg1 and reg2 to reg1 
    // mem[0x000] = 0x80; mem[0x001] = 0x14;
    // // opcode: 0x8014, ADD reg1 and reg2 to reg1
    // mem[0x000] = 0x80; mem[0x001] = 0x14;
    // // return
    // mem[0x0] = 0x80; mem[0x001] = 0x14;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
