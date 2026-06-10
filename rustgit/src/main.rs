use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::env::args;
use std::iter::Enumerate;

enum GitCommand{
    Init,
    Command(String),
    Unknow,
}

fn main() {
    let args: Vec<String> = args().collect();




}
