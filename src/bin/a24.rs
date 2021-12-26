#![feature(array_chunks)]

use std::fmt::{Debug, Formatter, Write, Display};
use std::rc::Rc;
use z3::ast::{Ast, Datatype};
use adventofcode2021::prelude::*;

type Integer = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Register::X => 'x',
            Register::Y => 'y',
            Register::Z => 'z',
            Register::W => 'w',
        };
        f.write_char(ch)
    }
}

impl Register {
    fn try_from(s: &str) -> Option<Register> {
        match s {
            "x" => Some(Register::X),
            "y" => Some(Register::Y),
            "z" => Some(Register::Z),
            "w" => Some(Register::W),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Var(Register),
    Immediate(Integer),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Var(reg) => Display::fmt(reg, f),
            Operand::Immediate(imm) => f.write_fmt(format_args!("{}", imm)),
        }
    }
}

impl Operand {
    fn try_from(s: &str) -> Option<Operand> {
        let imm_op = s.parse::<Integer>().ok().map(Operand::Immediate);
        imm_op.or_else(|| Some(Operand::Var(Register::try_from(s)?)))
    }

    fn value(&self, cpu: &CpuState) -> Integer {
        match self {
            Operand::Var(reg) => cpu.regs[*reg as usize],
            Operand::Immediate(imm) => *imm,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

impl Instruction {
    fn try_from(instruction: &str) -> Option<Instruction> {
        let mut split = instruction.split_ascii_whitespace();
        let instr = split.next()?;
        if instr == "inp" {
            let reg = Register::try_from(split.next()?)?;
            Some(Instruction::Inp(reg))
        } else {
            let reg = Register::try_from(split.next()?)?;
            let op = Operand::try_from(split.next()?)?;
            match instr {
                "add" => Some(Instruction::Add(reg, op)),
                "mul" => Some(Instruction::Mul(reg, op)),
                "div" => Some(Instruction::Div(reg, op)),
                "mod" => Some(Instruction::Mod(reg, op)),
                "eql" => Some(Instruction::Eql(reg, op)),
                _ => None
            }
        }
    }

    fn target_register(&self) -> Register {
        match self {
            Instruction::Inp(reg) => *reg,
            Instruction::Add(reg, _) => *reg,
            Instruction::Mul(reg, _) => *reg,
            Instruction::Div(reg, _) => *reg,
            Instruction::Mod(reg, _) => *reg,
            Instruction::Eql(reg, _) => *reg,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CpuState {
    port: u32,
    regs: [Integer; 4],
}

#[derive(Debug)]
enum CpuError {
    Overflow,
    DivideByZero,
    NegativeRemainder,
    InputOutOfBound,
}

type CpuResult = std::result::Result<(), CpuError>;

impl CpuState {
    fn reset(&mut self) {
        self.port = 0;
        self.regs.fill(0);
    }

    fn valid_state(&self) -> bool {
        self.regs[Register::Z as usize] == 0
    }

    fn execute(&mut self, instructions: &[Instruction], inputs: &[Integer; 14]) -> CpuResult {
        instructions.iter()
            .try_for_each(|instr| self.execute_instruction(instr, inputs))
    }

    fn execute_instruction(&mut self, instr: &Instruction, inputs: &[Integer; 14]) -> CpuResult {
        match instr {
            Instruction::Inp(reg) => {
                let next_input = *inputs.get(self.port as usize).ok_or(CpuError::InputOutOfBound)?;
                self.regs[*reg as usize] = next_input;
                self.port += 1;
            }
            Instruction::Add(reg, op) => {
                let op_value = op.value(self);
                let reg_value = &mut self.regs[*reg as usize];
                *reg_value = reg_value.checked_add(op_value).ok_or(CpuError::Overflow)?;
            }
            Instruction::Mul(reg, op) => {
                let op_value = op.value(self);
                let reg_value = &mut self.regs[*reg as usize];
                *reg_value = reg_value.checked_mul(op_value).ok_or(CpuError::Overflow)?;
            }
            Instruction::Div(reg, op) => {
                let op_value = op.value(self);
                let reg_value = &mut self.regs[*reg as usize];
                *reg_value = reg_value.checked_div (op_value).ok_or(CpuError::DivideByZero)?;
            }
            Instruction::Mod(reg, op) => {
                let op_value = op.value(self);
                if op_value < 0 {
                    return Err(CpuError::NegativeRemainder);
                }
                let reg_value = &mut self.regs[*reg as usize];
                if *reg_value < 0 {
                    return Err(CpuError::NegativeRemainder);
                }
                *reg_value = reg_value.checked_rem(op_value).ok_or(CpuError::DivideByZero)?;
            }
            Instruction::Eql(reg, op) => {
                let op_value = op.value(self);
                let reg_value = &mut self.regs[*reg as usize];
                *reg_value = (*reg_value == op_value) as Integer;
            }
        }
        if matches!(instr, Instruction::Add(Register::Z, Operand::Var(Register::Y))) {
            eprintln!("z = {}", self.regs[Register::Z as usize]);
        }

        Ok(())
    }



}

fn generate_code(instructions: &[Instruction]) {
    let mut i = 0_usize;
    println!("fn run(inputs: &[u8; 14]) -> Option<bool> {{");

    println!("let mut x: i64 = 0;");
    println!("let mut y: i64 = 0;");
    println!("let mut z: i64 = 0;");
    println!("let mut w: i64 = 0;");
    // println!("let mut i: usize = 0;");

    for instr in instructions.iter() {
        match instr {
            Instruction::Inp(reg) => {
                println!("{} = inputs[{}] as i64;", reg, i);
                println!("unsafe {{ if {} == 0 || {} > 9 {{ std::hint::unreachable_unchecked() }} }}", *reg, *reg);
                i += 1;
            }
            Instruction::Add(reg, op) => {
                println!("{} = {}.checked_add({})?;", reg, reg, op);
            }
            Instruction::Mul(reg, op) => {
                println!("{} = {}.checked_mul({})?;", reg, reg, op);
            }
            Instruction::Div(reg, op) => {
                println!("{} = {}.checked_div({})?;", reg, reg, op);
            }
            Instruction::Mod(reg, op) => {
                println!("if {} <= 0 {{ return None; }}", op);
                println!("if {} < 0 {{ return None; }}", reg);
                println!("{} = {}.checked_rem({})?;", reg, reg, op);
            }
            Instruction::Eql(reg, op) => {
                println!("{} = ({} == {}) as i64;", reg, reg, op);
            }
        }

    }
    println!("Some(z == 0)");
    println!("}}")
}

#[derive(Clone, Copy)]
enum Operation {
    Add, Mul, Div,
    Mod, Eql
}

#[derive(Clone)]
enum Expression {
    Input(usize),
    Literal(i64),
    Arithmetic(Operation, Box<Expression>, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Input(index) => f.write_char((b'a' +  *index as u8) as char)?,
            Expression::Literal(lit) => f.write_fmt(format_args!("{}", lit))?,
            Expression::Arithmetic(op, left, right) => {
                let op= match op {
                    Operation::Add => "+",
                    Operation::Mul => "*",
                    Operation::Div => "/",
                    Operation::Mod => "%",
                    Operation::Eql => "==",
                };
                f.write_str("(")?;
                f.write_str(op)?;
                f.write_char(' ')?;
                left.fmt(f)?;
                f.write_char(' ')?;
                right.fmt(f)?;
                f.write_str(")")?;
            }
        }

        Ok(())
    }
}

fn to_expression(instructions: &[Instruction], output: Register) -> Expression {
    let index = instructions.iter().rev().position(|instr| instr.target_register() == output);
    if index.is_none() {
        return Expression::Literal(0);
    }

    let index = instructions.len()-1-index.unwrap();

    let instr = &instructions[index];
    // dbg!(instr);
    dbg!(index);

    match instr {
        Instruction::Inp(_) => Expression::Input(instructions[0..index].iter().filter(|instr| matches!(instr, Instruction::Inp(_))).count()),
        Instruction::Add(left, op)
        | Instruction::Mul(left, op)
        | Instruction::Div(left, op)
        | Instruction::Mod(left, op)
        | Instruction::Eql(left, op) => {
            let left = to_expression(&instructions[0..index], *left);
            let right = match op {
                Operand::Var(right) => to_expression(&instructions[0..index], *right),
                Operand::Immediate(imm) => Expression::Literal(*imm),
            };
            let op = match instr {
                Instruction::Add(_, _) => Operation::Add,
                Instruction::Mul(_, _) => Operation::Mul,
                Instruction::Div(_, _) => Operation::Div,
                Instruction::Mod(_, _) => Operation::Mod,
                Instruction::Eql(_, _) => Operation::Eql,
                Instruction::Inp(_) => unreachable!()
            };

            Expression::Arithmetic(op, Box::new(left), Box::new(right))
        }
    }
}

pub fn run(inputs: &[u8; 14]) -> Option<bool> {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;
    let mut w: i64 = 0;
    w = inputs[0] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(1)?;
    x = x.checked_add(10)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(13)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[1] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(1)?;
    x = x.checked_add(13)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(10)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[2] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(1)?;
    x = x.checked_add(13)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(3)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[3] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(26)?;
    x = x.checked_add(-11)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(1)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[4] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(1)?;
    x = x.checked_add(11)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(9)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[5] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(26)?;
    x = x.checked_add(-4)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(3)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[6] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(1)?;
    x = x.checked_add(12)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(5)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[7] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(1)?;
    x = x.checked_add(12)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(1)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[8] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(1)?;
    x = x.checked_add(15)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(0)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[9] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(26)?;
    x = x.checked_add(-2)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(13)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[10] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(26)?;
    x = x.checked_add(-5)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(7)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[11] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(26)?;
    x = x.checked_add(-11)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(15)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[12] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(26)?;
    x = x.checked_add(-13)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(12)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    w = inputs[13] as i64;
    unsafe { if w == 0 { std::hint::unreachable_unchecked() } }
    unsafe { if w > 9 { std::hint::unreachable_unchecked() } }
    x = x.checked_mul(0)?;
    x = x.checked_add(z)?;
    if 26 <= 0 { return None; }
    if x < 0 { return None; }
    x = x.checked_rem(26)?;
    z = z.checked_div(26)?;
    x = x.checked_add(-10)?;
    x = (x == w) as i64;
    x = (x == 0) as i64;
    y = y.checked_mul(0)?;
    y = y.checked_add(25)?;
    y = y.checked_mul(x)?;
    y = y.checked_add(1)?;
    z = z.checked_mul(y)?;
    y = y.checked_mul(0)?;
    y = y.checked_add(w)?;
    y = y.checked_add(8)?;
    y = y.checked_mul(x)?;
    z = z.checked_add(y)?;
    Some(z == 0)
}


fn solve_z3(instructions: &[Instruction]) {
    use z3::*;

    fn to_ast<'ctx>(ctx: &'ctx Context, op: Operand, regs: &[ast::Int<'ctx>]) -> ast::Int<'ctx> {
        match &op {
            Operand::Var(reg) => regs[*reg as usize].clone(),
            Operand::Immediate(imm) => ast::Int::from_i64(ctx, *imm as i64),
        }
    }

    let mut cfg = Config::default();
    // cfg.set_bool_param_value("parallel.enable", true);
    let ctx = Context::new(&cfg);
    // let goal = Goal::new(&ctx, true, false, false);

    // Tactic::new(&ctx, "")
    // let opt = Solver::new(&ctx);
    let opt = Optimize::new(&ctx);

    let mut inputs = (0..14).map(|i| ast::Int::new_const(&ctx, format!("input{:02}", i)))
        .collect::<Vec<_>>();

    // (0..2).for_each(|i| inputs[i] = ast::Int::from_i64(&ctx, 1));

    /*
    let mut regs = [
        ast::Int::from_i64(&ctx, 0),
        ast::Int::from_i64(&ctx, 0),
        ast::Int::from_i64(&ctx, 0),
        ast::Int::from_i64(&ctx, 0),
    ];

     */

    let mut z = ast::Int::from_i64(&ctx, 0);

    let mut input = 0;
    let chunks = instructions.array_chunks::<18>();
    assert!(chunks.remainder().is_empty());
    for (i, chunk) in chunks.enumerate() {
        use Instruction::*;
        use Register::*;
        use Operand::*;
        match chunk {
            [
            Inp(W), Mul(X, Immediate(0)),
            Add(X, Var(Z)), Mod(X, Immediate(26)), Div(Z, Immediate(divz @ (1 | 26))), Add(X, Immediate(addx )),
            // x = z%26 + addx
            // z /= divz
            Eql(X, Var(W)), Eql(X, Immediate(0)),
            // x = if(x == w, 0, 1)
            Mul(Y, Immediate(0)), Add(Y, Immediate(25)), Mul(Y, Var(X)), Add(Y, Immediate(1)), Mul(Z, Var(Y)),
            // z *= 25 * x + 1
            Mul(Y, Immediate(0)), Add(Y, Var(W)), Add(Y, Immediate(addy)), Mul(Y, Var(X)), Add(Z, Var(Y))
            // z += (w+addy)*x
            ] => {
                let x = (&z % 26_i64) + (*addx as i64);
                if *divz != 1 {
                    z = &z / *divz;
                }
                let w = &inputs[input];
                // let x = &x._eq(w).ite(&ast::Int::from_i64(&ctx, 1), &ast::Int::from_i64(&ctx, 0));
                // let x = &x._eq(&ast::Int::from_i64(&ctx, 0)).ite(&ast::Int::from_i64(&ctx, 1), &ast::Int::from_i64(&ctx, 0));

                let b = x._eq(w).not();
                z = &z * &b.ite(&ast::Int::from_i64(&ctx, 26), &ast::Int::from_i64(&ctx, 1));

                // z = &z * ((25_i64 * x) + 1_i64);
                z = &z + &b.ite( &(w + (*addy as i64)), &ast::Int::from_i64(&ctx, 0));
                // z = z + (w + (*addy as i64)) * x;
                input += 1;

                // dbg!(divz, addx, addy);

            }
            _ => {
                panic!("Unknown chunk {:?}", chunk);
            }
        }


        /*
            for instr in instructions.iter() {
                match instr {
                    Instruction::Inp(reg) => {
                        regs[*reg as usize] = inputs[input].clone();
                        input += 1;
                    }
                    Instruction::Add(reg, op) => {
                        let src = std::mem::replace(&mut regs[*reg as usize], ast::Int::from_i64(&ctx, 0));
                        regs[*reg as usize] = src + to_ast(&ctx, *op, &regs);
                    }
                    Instruction::Mul(reg, op) => {
                        let src = std::mem::replace(&mut regs[*reg as usize], ast::Int::from_i64(&ctx, 0));
                        regs[*reg as usize] = src * to_ast(&ctx, *op, &regs);
                    }
                    Instruction::Div(reg, op) => {
                        let src = std::mem::replace(&mut regs[*reg as usize], ast::Int::from_i64(&ctx, 0));
                        regs[*reg as usize] = src / to_ast(&ctx, *op, &regs);
                    }
                    Instruction::Mod(reg, op) => {
                        let src = std::mem::replace(&mut regs[*reg as usize], ast::Int::from_i64(&ctx, 0));
                        regs[*reg as usize] = src % to_ast(&ctx, *op, &regs);
                    }
                    Instruction::Eql(reg, op) => {
                        let src = std::mem::replace(&mut regs[*reg as usize], ast::Int::from_i64(&ctx, 0));
                        let eq = src._eq(&to_ast(&ctx, *op, &regs));
                        regs[*reg as usize] = eq.ite(&ast::Int::from_i64(&ctx, 1), &ast::Int::from_i64(&ctx, 0));

                    }
                }
         */
            }


    // dbg!("ast ready");
    // dbg!(opt.get_model());

    // let z = &regs[Register::Z as usize].simplify();
    z = z.simplify();
    // opt.assert(&z._eq(&ast::Int::from_i64(&ctx, 0)));
    // dbg!("simplified", &z);
    // opt.assert(&inputs[0]._eq(&ast::Int::from_i64(&ctx, 6)));
    // opt.assert(&inputs[1]._eq(&ast::Int::from_i64(&ctx, 5)));
    // opt.assert(&inputs[2]._eq(&ast::Int::from_i64(&ctx, 9)));
    for input in inputs.iter().skip(0) {
        opt.assert(&input.ge(&ast::Int::from_i64(&ctx, 1)));
        opt.assert(&input.le(&ast::Int::from_i64(&ctx, 9)));
    }
    // opt.assert(&inputs[13]._eq(&ast::Int::from_i64(&ctx, 9)));
    opt.assert(&z._eq(&ast::Int::from_i64(&ctx, 0)));
    opt.assert(&z._eq(&ast::Int::from_i64(&ctx, 0)));

    let num = inputs.iter().fold(ast::Int::from_i64(&ctx, 0), |a, input| (a * 10_i64) + input);
    // opt.assert(&num.ge(&ast::Int::from_i64(&ctx, 65911695311329)));
    // dbg!(&num);
    // dbg!("maximizing");
    opt.minimize(&num);
    // opt.maximize(&inputs[0]);
    while let SatResult::Sat = opt.check(&[]) {
        let mdl = opt.get_model().expect("model");
        dbg!(&mdl);
        /*
        for input in inputs.iter() {
            let e = mdl.eval(input, true);
            dbg!(e);
        }

         */

        let nums = inputs.iter().map(|input| {
            let num = mdl.eval(input, true).expect("input").as_i64().expect("i64");
            if num < 1 || num > 9 {
                panic!("num out of range {}", num);
            }
            num
        }).collect::<Vec<_>>();
        dbg!(&nums);

        let num_value = nums.iter().fold(0_i64, |a, n| {
            (a * 10_i64) + *n as i64
        });
        // let num_value = mdl.eval(&num, true).expect("num").as_i64().expect("num value");
        println!("{}", num_value);

        let mut cpu = CpuState::default();
        cpu.execute(&instructions, &nums.try_into().unwrap()).unwrap();
        println!("valid = {}", cpu.valid_state());

        opt.assert(&num.lt(&ast::Int::from_i64(&ctx, num_value)));
        // dbg!(mdl.eval(&num, true));
        // 65911695311329 too low
        // 96357999941996
        // 96357999941996
        // 69914999975369
    }
}


fn rewrite(instructions: &[Instruction]) {
    use Instruction::*;
    use Register::*;
    use Operand::*;
    let chunks = instructions.array_chunks::<18>();
    for chunk in chunks {
        match chunk {
            // Unknown chunk [Inp(W), Mul(X, Immediate(0)), Add(X, Var(Z)), Mod(X, Immediate(26)), Div(Z, Immediate(1)),
            // Add(X, Immediate(10)), Eql(X, Var(W)), Eql(X, Immediate(0)), Mul(Y, Immediate(0)), Add(Y, Immediate(25)), Mul(Y, Var(X)), Add(Y, Immediate(1)), Mul(Z, Var(Y)), Mul(Y, Immediate(0)), Add(Y, Var(W)), Add(Y, Immediate(13)), Mul(Y, Var(X)), Add(Z, Var(Y))]', src/bin/a24.rs:414:17
            [
            Inp(W), Mul(X, Immediate(0)),
            Add(X, Var(Z)), Mod(X, Immediate(26)), Div(Z, Immediate(divz @ (1 | 26))), Add(X, Immediate(addx )),
            // x = z%26 + addx
            // z /= divz
            Eql(X, Var(W)), Eql(X, Immediate(0)),
            // x = if(x == w, 0, 1)
            Mul(Y, Immediate(0)), Add(Y, Immediate(25)), Mul(Y, Var(X)), Add(Y, Immediate(1)), Mul(Z, Var(Y)),
            // z *= 25 * x + 1
            Mul(Y, Immediate(0)), Add(Y, Var(W)), Add(Y, Immediate(addy)), Mul(Y, Var(X)), Add(Z, Var(Y))
            // z += (w+addy)*x
            ] => {
                println!("x = z%26 + {}; z /= {}; z *= 25*(x!=w) + 1; z+= (w+{})*(x!=w)", addx, divz, addy);
            }
            _ => {
                panic!("Unknown chunk {:?}", chunk);
            }
        }
    }
}

pub fn main() -> Result<()> {
    let instructions = include_str!("../../data/a24_input.txt")
        .lines()
        .map(|line| Instruction::try_from(line)
            .ok_or_else(|| Error::General(format!("Could not parse instruction: {}", line))))
        .collect::<Result<Vec<_>>>()?;

    // dbg!(&instructions);

    // generate_code(&instructions);

    // let mut cpu = CpuState::default();

    // rewrite(&instructions);

        // cpu.execute(&instructions, &[6,5,9,1,1,6,9,5,3,1,1,3,2,9]).unwrap();
        // cpu.execute(&instructions, &[9,1,1,1,1,1,1,1,1,1,1,9,5,8]).unwrap();
    // dbg!(run(&[6,5,9,1,1,6,9,5,3,1,1,3,2,9]));

    // dbg!(&cpu);
    // dbg!(cpu.valid_state());

    // let expr = to_expression(&instructions, Register::Z);
    //
    // eprintln!("{}", expr);

    solve_z3(&instructions);


    Ok(())
}