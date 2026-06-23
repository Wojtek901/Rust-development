#[derive(Debug)]
enum Command {
    Push(f32),
    Add,
    Print,
    Halt,
}

fn main() {
    let program = vec![
        Command::Push(5.0),
        Command::Push(6.0),
        Command::Add,
        Command::Print,
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
        }

        pc += 1;
    }
}