use rl::brute_force_solver::brute_force_solving;
use rl::peg_solitaire_environment::Solitaire;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json;

fn main() {
    println!("Hello, world!");

    // let s = brute_force_solving(50_000_000);
    let s = brute_force_solving(100_000);

    let state = Solitaire::new().hash_as_str();

    // let state = env.hash_as_str();
    println!("length of s {}", s.qs.len());
    println!("This is the value of the start state {:?} and it's appearance {}", s.get_state_value(&state), s.get_state_counter(&state));
    let b: HashMap<String, f64> = s.qs.into_iter().map(|(k, v)| (k, v.1)).collect();

    let serialized_json = serde_json::to_string(&b).unwrap();
    let path_json: &Path = Path::new("serialized.json");
    fs::write(path_json, serialized_json).unwrap();
}
