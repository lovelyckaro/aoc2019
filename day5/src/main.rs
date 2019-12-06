use std::fs;
use std::io;
use std::collections::VecDeque;
use std::io::Write;
use std::cmp::min;

#[derive(Debug)]
enum Op {
    Adress(i32),
    Value(i32)
}

impl Op {
    fn into_number(self, memory : &Vec<i32>) -> i32 {
        match self {
            Op::Adress(index) => {
                memory[index as usize]
            },
            Op::Value(value) => {
                value
            }
        }
    }
    fn into_index(self) -> usize {
        match self {
            Op::Adress(index) => index as usize,
            Op::Value(_) => panic!("Turning value into index")
        }
    }
}

type Destination = Op;

#[derive(Debug)]
enum Instruction {
    Add(Op, Op, Destination),
    Multiply(Op, Op, Destination),
    Input(Destination),
    Output(Op),
    JumpIfTrue(Op, Destination),
    JumpIfFalse(Op, Destination),
    LessThan(Op, Op, Destination),
    Equals(Op, Op, Destination),
    Stop
}
// Takes string CBA, where A defines mode for parameter 1, B for parameter 2 etc.
fn build_parameter_queue(modes : &str, parameters : &[i32]) -> VecDeque<Op>{
    let mut operands : VecDeque<Op> = VecDeque::with_capacity(3);
    // println!("Modes: {:?}", modes);
    let modes = modes.chars().rev();
    // println!("Parameters: {:?}", parameters);
    
    for (parameter, mode) in parameters.iter().zip(modes) {
        match mode {
            '1' => operands.push_back(Op::Value(*parameter)),
            '0' => operands.push_back(Op::Adress(*parameter)),
            _ => panic!("Invalid opMode")
        }
    }
    operands
}

impl Instruction {
    fn new(nums : &[i32]) -> Instruction {
        let mut function : String = nums[0].to_string();
        while function.len() < 5 {
            function.insert(0, '0');
        }
        match function.get(3..5) {
            Some("99") => Instruction::Stop,
            Some("01") => {
                let mut operands : VecDeque<Op> = build_parameter_queue(function.get_mut(0..=2).expect("function len < 3"), &nums[1..]);
                Instruction::Add(
                    operands.pop_front().unwrap(), 
                    operands.pop_front().unwrap(), 
                    operands.pop_front().unwrap()
                )
            },
            Some("02") => {
                let mut operands : VecDeque<Op> = build_parameter_queue(function.get_mut(0..=2).expect("function len < 3"), &nums[1..]);
                Instruction::Multiply(
                    operands.pop_front().unwrap(), 
                    operands.pop_front().unwrap(), 
                    operands.pop_front().unwrap()
                )
            },
            Some("03") => Instruction::Input(Op::Adress(nums[1])),
            Some("04") => {
                let mut operands : VecDeque<Op> = build_parameter_queue(function.get_mut(0..=2).expect("function len < 3"), &nums[1..=1]);
                Instruction::Output(operands.pop_front().unwrap()) 
            },
            Some("05") => {
                let mut operands : VecDeque<Op> = build_parameter_queue(function.get_mut(0..=2).expect("function len < 3"), &nums[1..=2]);
                Instruction::JumpIfTrue(
                    operands.pop_front().unwrap(),
                    operands.pop_front().unwrap()
                )
            },
            Some("06") => {
                let mut operands : VecDeque<Op> = build_parameter_queue(function.get_mut(0..=2).expect("function len < 3"), &nums[1..=2]);
                Instruction::JumpIfFalse(
                    operands.pop_front().unwrap(),
                    operands.pop_front().unwrap()
                )
            },
            Some("07") => {
                let mut operands : VecDeque<Op> = build_parameter_queue(function.get_mut(0..=2).expect("function len < 3"), &nums[1..]);
                Instruction::LessThan(
                    operands.pop_front().unwrap(),
                    operands.pop_front().unwrap(),
                    operands.pop_front().unwrap()
                )
            },
            Some("08") => {
                let mut operands : VecDeque<Op> = build_parameter_queue(function.get_mut(0..=2).expect("function len < 3"), &nums[1..]);
                Instruction::Equals(
                    operands.pop_front().unwrap(),
                    operands.pop_front().unwrap(),
                    operands.pop_front().unwrap()
                )
            },
            
            _ => panic!("Non defined instruction")
        }
    }
}

fn run(memory : &mut Vec<i32>) {
    let mut pc = 0;
    use Instruction::*;
    loop {
        let current = Instruction::new(memory.get(pc..min(pc+4, memory.len())).unwrap());
        println!("PC: {}", pc);
        println!("Current instruction: {:?}", current);
        match current {
            Stop => break,
            Add(op1, op2, dest) => {
                let value = op1.into_number(&memory) + op2.into_number(&memory);
                let index = dest.into_index();
                memory[index as usize] = value;
                pc += 4;
            },
            Multiply(op1, op2, dest) => {
                let value = op1.into_number(&memory) * op2.into_number(&memory);
                let index = dest.into_index();
                memory[index as usize] = value;
                pc += 4;
            },
            Input(dest) => {
                let inp = get_int();
                let index = dest.into_index();
                memory[index as usize] = inp;
                pc += 2;
            },
            Output(op1) => {
                let value = op1.into_number(&memory);
                println!("{}", value);
                pc += 2;
            },
            JumpIfTrue(op1, dest) => {
                let value = op1.into_number(&memory);
                if value != 0 {
                    pc = dest.into_number(&memory) as usize;
                }else {
                    pc += 3;
                }
            },
            JumpIfFalse(op1, dest) => {
                let value = op1.into_number(&memory);
                if value == 0 {
                    pc = dest.into_number(&memory) as usize;
                }else {
                    pc += 3;
                }
            },
            LessThan(op1, op2, dest) => {
                let op1 = op1.into_number(&memory);
                let op2 = op2.into_number(&memory);
                let index = dest.into_index();
                if op1 < op2 {
                    memory[index] = 1;
                }else {
                    memory[index] = 0;
                }
                pc += 4;
            },
            Equals(op1, op2, dest) => {
                let op1 = op1.into_number(&memory);
                let op2 = op2.into_number(&memory);
                let index = dest.into_index();
                if op1 == op2 {
                    memory[index] = 1;
                }else {
                    memory[index] = 0;
                }
                pc += 4;
            }
        }
    }
}

fn get_int() -> i32 {
    print!("Input > ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    trimmed.parse().expect("Failed to parse int")
}

fn get_initial_memory() -> Vec<i32> {
    let file = fs::read_to_string("input5.txt").expect("Failed to read from input5.txt");
    file.trim_end_matches('\n').split(',').map(|x| x.parse().unwrap()).collect()
}

fn get_example_program() -> Vec<i32> {
    let program = String::from("3,3,1108,-1,8,3,4,3,99");
    program.split(',').map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let mut memory = get_initial_memory();
    // let mut memory = get_example_program();
    // println!("Initial memory: {:?}", memory);
    run(&mut memory);
}
