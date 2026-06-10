use std::env::args;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Raport {
    file_name: String,
    searched_word: String,
    number_of_occurrences: u32,
    print_logs: bool,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 || args.len() > 4 {
        println!("Wrong numbers of arguments");
        println!("Expected: <file path> <log to search> <if print logs>");
        return;
    }

    let file_path = &args[1];
    let search_word = &args[2];
    let print_logs = args.len() == 4;

    let mut final_raport = Raport {
        file_name: file_path.to_string(),
        searched_word: search_word.to_string(),
        number_of_occurrences: 0,
        print_logs,
    };

    if final_raport.print_logs {
        println!("--- Wykryte logi: ---");
    }

    if let Ok(lines) = read_lines(file_path) {
        for line_result in lines {
            if let Ok(line) = line_result {

                for word in line.split_whitespace() {
                    if word.trim_end_matches(':') == final_raport.searched_word {
                        final_raport.number_of_occurrences += 1;
                        if final_raport.print_logs {
                            println!("{}", line);
                        }
                        break;
                    }
                }
            }
        }
    } else {
        println!("ERROR: can not find or access file {}", file_path);
        return;
    }

    println!("\n--- Raport for file: {} ---", final_raport.file_name);
    println!("Searched word: '{}', found {} times", final_raport.searched_word, final_raport.number_of_occurrences);
}