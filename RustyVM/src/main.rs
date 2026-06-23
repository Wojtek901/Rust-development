#[derive(Debug)]
enum Command {
    Push(f32),
    Add,
    Print,
    Halt,
    Sub,
    Dup,
    Jnz(usize),
}

fn main() {
    let program = vec![
        Command::Push(6.0),
        Command::Dup,
        Command::Print,
        Command::Push(1.0),
        Command::Sub,
        Command::Dup,
        Command::Jnz(1),
        Command::Halt,
    ];

    let mut pc: usize = 0;
    let mut stack: Vec<f32> = Vec::new();

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
        }

        pc += 1;
    }
}