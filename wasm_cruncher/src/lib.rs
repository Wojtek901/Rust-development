use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn find_primes(limit: u32) -> u32{
    let begin_computetion_info = format!("Begin search of prime numbers up to {} ...", limit);
    console::log_1(&begin_computetion_info.into());

    if limit == 1{
        return 0
    }
    if limit == 2{
        return 1
    }

    let mut found_primes: Vec<u32> = Vec::new();

    for number_to_check in 2..limit {
        let has_divisor = found_primes.iter().any(|&p| number_to_check % p == 0);

        if !has_divisor {
            found_primes.push(number_to_check);
        }
    }

    found_primes.len() as u32
}
