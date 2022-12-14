use std::str::Split;

#[derive(Clone, Debug)]
enum Instruction {
    Hlf(u8),
    Tpl(u8),
    Inc(u8),
    Jmp(i32),
    Jie(u8, i32),
    Jio(u8, i32),
}

impl Instruction {
    fn from_str(input: &str) -> Self {
        fn read_reg(split: &mut Split<&str>) -> u8 {
            match split.next().unwrap() {
                "a" => 0,
                "b" => 1,
                _ => panic!(),
            }
        }
        fn read_offset(split: &mut Split<&str>) -> i32 {
            split.next().unwrap().parse().unwrap()
        }
        let (instr, args) = input.split_once(' ').unwrap();
        let mut args = args.split(", ");
        match instr {
            "hlf" => Self::Hlf(read_reg(&mut args)),
            "tpl" => Self::Tpl(read_reg(&mut args)),
            "inc" => Self::Inc(read_reg(&mut args)),
            "jmp" => Self::Jmp(read_offset(&mut args)),
            "jie" => Self::Jie(read_reg(&mut args), read_offset(&mut args)),
            "jio" => Self::Jio(read_reg(&mut args), read_offset(&mut args)),
            _ => panic!(),
        }
    }
}

#[derive(Default)]
struct VM {
    registers: [u32; 2],
    line: usize,
    program: Vec<Instruction>,
}

impl VM {
    fn from_str(input: &str) -> Self {
        Self {
            program: input
                .lines()
                .map(Instruction::from_str)
                .collect(),
            ..Default::default()
        }
    }

    fn run_one(&mut self) {
        self.line = match self.program[self.line] {
            Instruction::Hlf(reg) => {
                self.registers[reg as usize] /= 2;
                self.line + 1
            }
            Instruction::Tpl(reg) => {
                self.registers[reg as usize] *= 3;
                self.line + 1
            }
            Instruction::Inc(reg) => {
                self.registers[reg as usize] += 1;
                self.line + 1
            }
            Instruction::Jmp(offset) => (self.line as isize + offset as isize) as usize,
            Instruction::Jie(reg, offset) => {
                if self.registers[reg as usize] % 2 == 0 {
                    (self.line as isize + offset as isize) as usize
                } else {
                    self.line + 1
                }
            }
            Instruction::Jio(reg, offset) => {
                if self.registers[reg as usize] == 1 {
                    (self.line as isize + offset as isize) as usize
                } else {
                    self.line + 1
                }
            }
        }
    }

    fn run(&mut self) {
        while self.line < self.program.len() {
            self.run_one();
        }
    }
}

pub fn solution_1(input: &str) -> String {
    let mut vm = VM::from_str(input);
    vm.run();
    vm.registers[1].to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut vm = VM::from_str(input);
    vm.registers[0] = 1;
    vm.run();
    vm.registers[1].to_string()
}
