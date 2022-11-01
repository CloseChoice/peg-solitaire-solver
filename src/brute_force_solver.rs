use crate::peg_solitaire_environment::{Solitaire, SolitaireAction};
use crate::state_function::StateFunction;
use std::time::Instant;

pub fn simulate_and_get_least_played_action(s: &mut StateFunction, env: &Solitaire) -> SolitaireAction {
    let mut state_counter = i32::MAX;
    let mut prefered_action = env.actions().unwrap()[0]; // just initialize this with first action
    for action in env.actions().unwrap().iter() {
        let (state, holes) = env.simulate_action(&action.value());
        let hash = Solitaire::hash_state_as_string(&state, &holes);
        let c = s.get_state_counter(&hash);
        if c == 0 {
            return *action;
        }
        if c < state_counter {
            state_counter = c;
            prefered_action = *action;
        }

    }
    prefered_action.clone()
}

pub fn brute_force_solving(
    repetitions: u128,
) -> StateFunction
    {
    
    let mut s = StateFunction::new();
    let mut length = 0;
    
    let now = Instant::now();
    for idx in 0..repetitions {
        if idx % 50_000 == 1 {
            if s.qs.len() - length == 0 {
                println!("No new values found, abort");
                break;
            }

            length = s.qs.len();
            let dummy_state = Solitaire::new().hash_as_str();
            println!(
                "Repetition: {} of {} -- after {} seconds. Length of s {}. This is the best yet {:?}",
                idx,
                repetitions,
                now.elapsed().as_secs(),
                length,
                s.get_state_value(&dummy_state)
            );
        }

        let mut env = Solitaire::new();
        let mut reward = 0.;
        let mut state_vec = Vec::new();
        let mut visited_states = Vec::new();

        let mut action = simulate_and_get_least_played_action(&mut s, &env);
        let hash = env.hash_as_str();
        state_vec.push(hash);
        visited_states.push(env.state.to_string());
        while !env.finished() {
            reward += env.take_action(&action.value());
            let hash = env.hash_as_str();
            state_vec.push(hash);
            visited_states.push(env.state.to_string());
            // println!("These are the actions {:?}", env.actions());
            // println!("Is the game finished {}", env.finished());
            if env.finished() {
                if env.holes.len() == 32 && env.state.value[3][3] == 1 {
                    reward += 10.;
                }

                for (hash, visited_state) in state_vec.iter().zip(visited_states.iter()) {
                    s.update_state_value_with_fn(hash.clone(), visited_state.clone(), f64::max, reward);
                }
                break;
            }
            action = simulate_and_get_least_played_action(&mut s, &env);
        }
    }
    s 
}