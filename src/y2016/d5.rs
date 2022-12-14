pub fn solution_1(input: &str) -> String {
    let mut ctx = md5::Context::new();
    ctx.consume(input);
    let mut i = 1;
    let mut result = String::new();

    while result.len() < 8 {
        let mut ctx = ctx.clone();
        ctx.consume(i.to_string());
        let hash = ctx.compute();
        if hash[0] == 0x00 && hash[1] == 0x00 && hash[2] <= 0x0F {
            result.push(match hash[2] & 0x0F {
                n @ 0x00..=0x09 => (b'0' + n) as char,
                n => (b'a' + (n - 0x0A)) as char,
            });
        }
        i += 1;
    }
    result
}

pub fn solution_2(input: &str) -> String {
    let mut ctx = md5::Context::new();
    ctx.consume(input);
    let mut i = 1;
    let mut result = [0xFF; 8];
    let mut set = 0;
    while set < 8 {
        let mut ctx = ctx.clone();
        ctx.consume(i.to_string());
        let hash = ctx.compute();
        if hash[0] == 0x00 && hash[1] == 0x00 && hash[2] <= 0x0F {
            let i = (hash[2] & 0x0F) as usize;
            if i < result.len() && result[i] == 0xFF {
                set += 1;
                result[i] = (hash[3] & 0xF0) >> 4
            }
        }
        i += 1;
    }
    result
        .iter()
        .map(|byte| match byte {
            n @ 0x00..=0x09 => (b'0' + n) as char,
            n => (b'a' + (n - 0x0A)) as char,
        })
        .collect()
}
