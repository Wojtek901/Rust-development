use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::env::args;
use std::path::Path;

enum GitCommand {
    Init,
    Commit(String),
    Unknown,
}

fn parse_command(args: Vec<String>) -> GitCommand {
    if args.len() < 2 {
        return GitCommand::Unknown;
    }

    let command = args[1].to_lowercase();

    match command.as_str() {
        "init" => GitCommand::Init,
        "commit" => {
            if args.len() == 3 {
                GitCommand::Commit(args[2].clone())
            } else {
                println!("Error: Commit command requires a file name.");
                GitCommand::Unknown
            }
        },
        _ => GitCommand::Unknown,
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let command: GitCommand = parse_command(args);

    match command {
        GitCommand::Init => {
            let creating_repo_result = fs::create_dir(".rustgit");

            match creating_repo_result {
                Ok(_) => {
                    fs::create_dir(".rustgit/objects").unwrap();
                    println!("Repository initialized correctly.");
                }
                Err(_) => {
                    println!("Repository already exists.");
                    println!("You can commit files with command: commit <file name>");
                }
            }
        }

        GitCommand::Commit(commit_file_name) => {
            if !Path::new(".rustgit").exists() {
                println!("RustGIT directory is not initialized. Before committing make sure to initialize it.");
                println!("Use command: init");
                return;
            }

            if let Ok(file_data) = fs::read_to_string(&commit_file_name) {

                let mut hasher = DefaultHasher::new();
                file_data.hash(&mut hasher);
                let file_id: u64 = hasher.finish();

                let commited_file_path = format!(".rustgit/objects/{}", file_id);

                fs::write(commited_file_path, &file_data).unwrap();
                fs::write(".rustgit/HEAD", file_id.to_string()).unwrap();

                println!("Changes staged successfully!!!");
            } else {
                println!("Wrong file name, or file directory. Check spelling or file existence.");
            }
        }

        GitCommand::Unknown => {
            println!("Unknown command. Try one of these:");
            println!(" - init - to initialize repository");
            println!(" - commit <file path> - to commit changes of a given file");
        }
    }
}
