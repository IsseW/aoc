use md5;
use std::fmt::Write;

pub fn solution_1(input: &str) -> String {
    let mut ctx = md5::Context::new();
    ctx.consume(input);
    let mut i = 1;
    loop {
        let mut ctx = ctx.clone();
        ctx.consume(i.to_string());
        let hash = ctx.compute();
        if hash[0] == 0x00 && hash[1] == 0x00 && hash[2] <= 0x0F {
            break i;
        }
        i += 1;
    }
    .to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut ctx = md5::Context::new();
    ctx.consume(input);
    let mut i = 1;
    let mut buffer = String::new();
    loop {
        let mut ctx = ctx.clone();
        write!(buffer, "{}", i).unwrap();
        ctx.consume(buffer.as_str());
        buffer.clear();
        let hash = ctx.compute();
        if hash[0] == 0x00 && hash[1] == 0x00 && hash[2] == 0x00 {
            break i;
        }
        i += 1;
    }
    .to_string()
}
