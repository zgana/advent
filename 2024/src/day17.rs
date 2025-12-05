use core::panic;
use std::fmt;
use std::fs;
use std::io::{self, Write};

use anyhow::Result;
use regex::{Match, Regex};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn from_u8(c: u8) -> Self {
        use Opcode::*;
        match c {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => panic!("could not parse {c}"),
        }
    }

    fn as_digit(&self) -> u8 {
        use Opcode::*;
        match self {
            Adv => 0,
            Bxl => 1,
            Bst => 2,
            Jnz => 3,
            Bxc => 4,
            Out => 5,
            Bdv => 6,
            Cdv => 7,
        }
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operand({})", self)
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Opcode::*;
        let repr = match self {
            Adv => "Adv",
            Bxl => "Bxl",
            Bst => "Bst",
            Jnz => "Jnz",
            Bxc => "Bxc",
            Out => "Out",
            Bdv => "Bdv",
            Cdv => "Cdv",
        };
        write!(f, "{}", repr)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Operand(u8);

impl Operand {
    fn from_u8(value: u8) -> Self {
        if !(0..8).contains(&value) {
            panic!("value should be in 0..8; got {value}");
        }
        Self(value)
    }

    fn as_lit(&self) -> u64 {
        self.0 as u64
    }

    fn as_combo(&self, comp: &Computer) -> u64 {
        match self.0 {
            0..=3 => self.0 as u64,
            4 => comp.a,
            5 => comp.b,
            6 => comp.c,
            _ => panic!("should never find {:?}", self),
        }
    }
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operand({})", self)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

impl Instruction {
    fn new(opcode_u8: u8, operand_u8: u8) -> Self {
        let opcode = Opcode::from_u8(opcode_u8);
        let operand = Operand::from_u8(operand_u8);
        Self { opcode, operand }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instruction({}[{}])", self.opcode, self.operand)
    }
}

fn register_from(m: Option<Match>) -> u64 {
    m.unwrap()
        .as_str()
        .parse()
        .expect("should be able to parse register")
}

#[derive(Clone, Debug, Default)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    prog: Vec<Instruction>,
    ptr: usize,
    out: Vec<u64>,
}

impl Computer {
    fn new(text: &str) -> Self {
        let re = Regex::new(
            r"
Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: ([\d,]+)"
                .trim(),
        )
        .unwrap();
        let caps = re.captures(text).expect("should be able to parse text");
        let a = register_from(caps.get(1));
        let b = register_from(caps.get(2));
        let c = register_from(caps.get(3));

        let prog_vals: Vec<u8> = caps
            .get(4)
            .unwrap()
            .as_str()
            .split(',')
            .map(|s| s.parse().expect("should be able to parse prog"))
            .collect();
        let mut prog = Vec::new();
        for (v1, v2) in prog_vals
            .iter()
            .step_by(2)
            .zip(prog_vals.iter().skip(1).step_by(2))
        {
            prog.push(Instruction::new(*v1, *v2));
        }

        Self {
            a,
            b,
            c,
            prog,
            ..Default::default()
        }
    }

    fn step(&mut self) -> bool {
        use Opcode::*;

        // unpack current instruction
        if self.ptr >= self.prog.len() {
            return false;
        }

        let Instruction { opcode, operand } = self
            .prog
            .get(self.ptr)
            .expect("instruction ptr should be in bounds");

        // process opcode
        match opcode {
            Adv => {
                let o = operand.as_combo(self) as u32;
                let denom = 2u64.pow(o);
                self.a /= denom;
            }
            Bxl => {
                let o = operand.as_lit();
                self.b ^= o;
            }
            Bst => {
                let o = operand.as_combo(self);
                self.b = o % 8;
            }
            Jnz => {
                if self.a != 0 {
                    let o = operand.as_lit() as usize;
                    self.ptr = o.wrapping_sub(1);
                }
            }
            Bxc => {
                self.b ^= self.c;
            }
            Out => {
                let o = operand.as_combo(self);
                let value = o % 8;
                self.out.push(value);
            }
            Bdv => {
                let o = operand.as_combo(self) as u32;
                let denom = 2u64.pow(o);
                self.b = self.a / denom;
            }
            Cdv => {
                let o = operand.as_combo(self) as u32;
                let denom = 2u64.pow(o);
                self.c = self.a / denom;
            }
        };

        // advance instruction ptr and return
        self.ptr = self.ptr.wrapping_add(1);
        true
    }

    fn run(&self) -> Self {
        let mut out = self.clone();
        while out.step() {}
        out
    }

    fn run_verbose(&self) -> Self {
        let mut out = self.clone();
        println!(
            "prog = {} = [\n  {:}\n]",
            self.progstr(","),
            self.prog
                .iter()
                .map(|i| format!("{i}"))
                .collect::<Vec<_>>()
                .join(",\n  ")
        );
        while out.step() {
            println!("out.out = {:?}", out.out);
        }
        out
    }

    fn progstr(&self, sep: &str) -> String {
        self.prog
            .iter()
            .map(|instruction| {
                let Instruction { opcode, operand } = instruction;
                format!("{}{}{}", opcode.as_digit(), sep, operand.0)
            })
            .collect::<Vec<_>>()
            .join(sep)
    }

    fn outstr(&self) -> String {
        self.out
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn part1(comp: &Computer) -> String {
    let comp = comp.run();
    comp.outstr()
    // comp.out
    //     .iter()
    //     .map(|value| value.to_string())
    //     .collect::<Vec<_>>()
    //     .join(",")
}

fn part2(comp: &Computer) -> u64 {
    let s = comp
        .progstr("")
        .chars()
        .rev()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("");
    let octal = format!("{s}0");
    let num = u64::from_str_radix(&octal, 8).expect("should be able to parse");
    println!("s = {s}\noctal = 0o{octal}\nnum = {num}");

    let mut comp = comp.clone();
    comp.a = num;
    let comp = comp.run();
    println!("comp.out = {:}", comp.outstr());

    num
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-17.txt")?;
    let comp = Computer::new(&input);

    println!("{}", part1(&comp));
    println!("{}", part2(&comp));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST2: &str = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_parse() {
        let c = Computer::new(TEST.trim());
        // dbg!(c);
    }

    #[test]
    fn test_ex1() {
        let text = "
Register A: 0
Register B: 0
Register C: 9

Program: 2,6";
        let comp = Computer::new(text.trim());
        let result = comp.run();
        assert_eq!(result.b, 1);
    }

    #[test]
    fn test_ex2() {
        let text = "
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
        let comp = Computer::new(text.trim());
        let result = comp.run();
        assert_eq!(result.out, vec![0, 1, 2]);
    }

    #[test]
    fn test_ex3() {
        let text = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let comp = Computer::new(text.trim());
        let result = comp.run();
        assert_eq!(result.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(result.a, 0);
    }

    #[test]
    fn test_ex4() {
        let text = "
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";
        let comp = Computer::new(text.trim());
        let result = comp.run();
        assert_eq!(result.b, 44354);
    }

    #[test]
    fn test_run() {
        let comp = Computer::new(TEST);
        let result = comp.run();
        let expected = vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0];
        assert_eq!(result.out, expected);
    }

    #[test]
    fn test_part1() {
        let comp = Computer::new(TEST);
        let result = part1(&comp);
        let expected = "4,6,3,5,6,3,5,2,1,0";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let comp = Computer::new(TEST2);
        let result = part2(&comp);
        println!();
        println!();
        assert_eq!(result, 117440);
    }

    #[test]
    fn test_part2_print() {
        let mut comp = Computer::new(TEST2);
        for a in 117440 - 2..=117440 + 10 {
            comp.a = a;
            comp.run();
            let s = comp
                .progstr("")
                .chars()
                .rev()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("");
            println!("a = {a} = {a:#o} | s = {} | prog = {}", s, comp.progstr(""));
        }
    }

    //
}
