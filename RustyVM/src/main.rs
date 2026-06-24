use std::fs;

#[derive(Debug)]
enum Command {
    Push(f32),
    Add,
    Print,
    Halt,
    Sub,
    Dup,
    Jnz(usize),
    Store(usize),
    Load(usize),
}

fn parse_file(file_name: &str) -> Vec<Command>{
    let mut program: Vec<Command> = Vec::new();
    let file_content = fs::read_to_string(file_name).expect("File does not exist or is in other directory.");

    for (i, line) in file_content.lines().enumerate(){
        let content: Vec<&str> = line.split_whitespace().collect();

        if content.is_empty(){
            continue;
        }

        match content[0].to_uppercase().as_str() {
            "ADD" => {program.push(Command::Add)}
            "PRINT" => program.push(Command::Print),
            "HALT" => program.push(Command::Halt),
            "SUB" => program.push(Command::Sub),
            "DUP" => program.push(Command::Dup),

            "PUSH" => {
                let val_to_push = content.get(1).expect("Command <Push> should be given value to push: Push <value>");
                let value = val_to_push.parse().expect("Wrong symbol, <value> should be a number");
                program.push(Command::Push(value));
            }

            "JNZ" => {
                let val_to_push = content.get(1).expect("Command <Jnz> should be given value to jump to: Jnz <value>");
                let value = val_to_push.parse().expect("Wrong symbol, <value> should be a number");
                program.push(Command::Jnz(value));
            }

            "STORE" => {
                let address_str = content.get(1).expect("Command <Store> should be given address: Store <address>");
                let address = address_str.parse().expect("Wrong symbol, <address> should be a number");
                program.push(Command::Store(address));
            }

            "LOAD" => {
                let address_str = content.get(1).expect("Command <Load> should be given address: Load <address>");
                let address = address_str.parse().expect("Wrong symbol, <address> should be a number");
                program.push(Command::Load(address));
            }

            unknown_command => {
                panic!("Wrong command in line {}. Recieved unknown instruction {}.", i, unknown_command);
            }
        }
    }

    program
}

fn run_vm(program: Vec<Command>){
    let mut pc: usize = 0;
    let mut stack: Vec<f32> = Vec::new();
    let mut ram: Vec<f32> = vec![0.0; 256];

    loop {
        if pc >= program.len() {
            println!("Program Counter out of memory!");
            break;
        }

        let current_command = &program[pc];

        match current_command {
            Command::Push(value) => {
                stack.push(*value);
            }

            Command::Add => {
                match (stack.pop(), stack.pop()) {
                    (Some(b), Some(a)) => {
                        let result = a + b;
                        stack.push(result);
                    }
                    _ => {
                        println!("No number on stack");
                        break;
                    }
                }
            }

            Command::Print => {
                match stack.pop() {
                    Some(val) => {
                        println!("{}", val);
                    }

                    None => {
                        println!("finish");
                    }
                }
            }

            Command::Halt => {
                println!("finish");
                break;
            }

            Command::Sub => {
                match (stack.pop(), stack.pop()) {
                    (Some(b), Some(a)) => {
                        let result = a - b;
                        stack.push(result);
                    }
                    _ => {
                        println!("No number on stack");
                        break;
                    }
                }
            }

            Command::Dup => {
                match stack.pop(){
                    Some(value) => {
                        stack.push(value);
                        stack.push(value);
                    }
                    None => {
                        println!("Empty stack.")
                    }
                }
            }

            Command::Jnz(jump) => {
                if let Some(condition) = stack.pop() {
                    if condition != 0.0 {
                        pc = *jump;
                        continue;
                    }
                } else {
                    println!("No number on stack");
                    break;
                }
            }

            Command::Store(address) => {
                if let Some(value) = stack.pop() {
                    ram[*address] = value;
                } else {
                    println!("No number on stack");
                    break;
                }
            }

            Command::Load(address) => {
                let value = ram[*address];
                stack.push(value);
            }
        }

        pc += 1;
    }
}

fn main() {
    let program: Vec<Command> = parse_file("program_ram.rvm");

    run_vm(program);
}