use crate::peg_solitaire_environment::{StateT, ActionT, Solitaire, SolitaireState};
use std::collections::HashMap;

#[derive(Debug)]
pub struct StateFunction {
    pub qs: HashMap<String, (i32, f64)>,
}

impl StateFunction {
    pub fn new() -> Self {
        let hash: HashMap<String, (i32, f64)> = HashMap::new();
        StateFunction { qs: hash }
    }

    pub fn update_state_value_with_fn<F>(&mut self, 
                              state_hash: &String,
                              fun: F ,
                              value: f64
                            ) where F: Fn(f64, f64) -> f64
        {
        let old_value = self.qs.get(state_hash).unwrap_or(&(0, 0.));
        let new_value = fun(old_value.1, value);
        self.qs.insert(state_hash.clone(), (old_value.0 + 1, new_value));
    }

    pub fn get_state_value(&self, 
                           state_hash: &String) -> Option<f64> {
        match self.qs.get(state_hash) {
            Some(value) => Some(value.1.clone()),
            None => None
        }
    }

    pub fn get_state_counter(&self,
                             state_hash: &String) -> i32 {
        match self.qs.get(state_hash) {
            Some(value) => value.0.clone(),
            None => 0
        }
    }

    pub fn get_least_seen_state(&self, state_hashes: Vec<&String>) -> String {
        let mut least_seen_state = state_hashes[0];
        let mut counter = i32::MAX;

        for state in state_hashes.iter() {
            let c = self.get_state_counter(state);
            // don't waste time, take this state immediately
            if c == 0 {
                return (*state).to_string()
            }
            if c < counter {
                least_seen_state = state;
                counter = c;
            }
        }
        (*least_seen_state).to_string()
    }

    pub fn update_reward_and_logging(&mut self, state: SolitaireState, visited_hashes: Vec<String>, reward: f64, iterations: &mut i128) {
        let reward_entry = match visited_hashes[visited_hashes.len() - 1].as_str() {
            "32_1565.69579_72.843619" => reward + 10.,
            _ => reward,
        };
        for hash in visited_hashes {
            self.update_state_value_with_fn(&hash, f64::max, reward_entry);
        }
        *iterations += 1;
        // println!("EVERYTHING DONE: This is env\n{}", Solitaire::from_state(state));
        if ((*iterations) % 100_000 == 0) {
            println!("Reached {} iterations, visited {} positions", iterations, self.qs.len());
        }
    }

    pub fn iterate_game(&mut self, state: SolitaireState, mut visited_hashes: Vec<String>, reward: f64, iterations: &mut i128) {
        let env = Solitaire::from_state(state);
        // println!("START OF FUNCTION: This is env\n{}", Solitaire::from_state(state.clone()));
        // println!("START OF FUNCTION: These are hashes: {:?}", visited_hashes);
        let current_hash = env.hash_as_str();
        visited_hashes.push(current_hash.clone());
        // weitere opt möglichkeit: check ob hash in der state function ist, wenn ja, füge allen vorherigen states
        // den gleichen wert hinzu
        if self.qs.get(&current_hash).is_some() {
           let (num, reward) = self.qs.get(&current_hash).unwrap();
           self.update_reward_and_logging(state, visited_hashes , *reward, iterations);
        }
        else {
        let actions = env.get_symmetry_reduced_actions();
            match actions {
                Some(actions) => {
                    // println!("We iterate over {} actions", actions.len());
                    // wir iterieren hier über alle möglichkeiten ohne die Symmetrie zu beachten -> schreibe env.get_symmetry_reduced_actions()
                    // vllt ist ein check auch noch hilfreich, denn wenn ein hash schon den max wert hat, wird dieser nicht mehr
                    // weiter verbessert
                    for action in actions {
                        let (state, _) = env.simulate_action(&action.value());
                        // println!("DURING ITERATION: This is env\n{}", Solitaire::from_state(state.clone()));
                        self.iterate_game(state, visited_hashes.clone(), reward + 1., iterations);
                    }
                },
                None => {
                    self.update_reward_and_logging(state, visited_hashes , reward, iterations);
                    // let reward_entry = match env.hash_as_str().as_str() {
                    //     "32_1565.69579_72.843619" => reward + 10.,
                    //     _ => reward,
                    // };
                    // for hash in visited_hashes {
                    //     self.update_state_value_with_fn(&hash, f64::max, reward_entry);
                    // }
                    // *iterations += 1;
                    // println!("EVERYTHING DONE: This is env\n{}", Solitaire::from_state(state));
                    // if ((*iterations) % 100_000 == 0) {
                    //     println!("Reached {} iterations, visited {} positions", iterations, self.qs.len());
                    // }
                }
            }
        }
    }
}


mod tests {
    use super::*;
    use crate::peg_solitaire_environment::SolitaireState;


    #[test]
    fn test_iterate_game_complete() {
        let state = SolitaireState {
            value: [
                [-1, -1, 1, 1, 1, -1, -1],
                [-1, -1, 1, 1, 1, -1, -1],
                [ 1,  1, 1, 1, 1,  1,  1],
                [ 1,  1, 1, 0, 1,  1,  1],
                [ 1,  1, 1, 1, 1,  1,  1],
                [-1, -1, 1, 1, 1, -1, -1],
                [-1, -1, 1, 1, 1, -1, -1],
            ],
        };
        let env = Solitaire::from_state(state.clone());
        let mut state_function = StateFunction::new();
        state_function.iterate_game(state, vec![], 0., &mut 0);
        
        println!("other: {:?}", state_function.qs.get(&env.hash_as_str()));
        println!("LEN OF state function: {}", state_function.qs.len());
        for (h, s) in state_function.qs.iter() {
            println!("Hash {}\tvisits {}\tvalue {}", h, s.0, s.1);
        }
        println!("other: {:?}", state_function.qs);
    }

    #[test]
    fn test_iterate_game_endgame2() {
        let state = SolitaireState {
            value: [
                [-1, -1, 0, 0, 0, -1, -1],
                [-1, -1, 0, 0, 0, -1, -1],
                [ 0,  0, 0, 0, 0,  0,  0],
                [ 0,  0, 0, 0, 0,  1,  0],
                [ 0,  1, 1, 0, 1,  0,  1],
                [-1, -1, 0, 0, 1, -1, -1],
                [-1, -1, 0, 0, 0, -1, -1],
            ],
        };
        let env = Solitaire::from_state(state.clone());
        let mut state_function = StateFunction::new();
        state_function.iterate_game(state, vec![], 26., &mut 0);
        
        println!("other: {:?}", state_function.qs.get(&env.hash_as_str()));
        println!("LEN OF state function: {}", state_function.qs.len());
        for (h, s) in state_function.qs.iter() {
            println!("Hash {}\tvisits {}\tvalue {}", h, s.0, s.1);
        }
        println!("other: {:?}", state_function.qs);
    }

    #[test]
    fn test_iterate_game_middlegame() {
        let state = SolitaireState {
            value: [
                [-1, -1, 1, 0, 0, -1, -1],
                [-1, -1, 1, 0, 0, -1, -1],
                [ 0,  0, 0, 0, 0,  0,  0],
                [ 0,  0, 0, 1, 1,  1,  0],
                [ 0,  1, 0, 0, 1,  0,  1],
                [-1, -1, 0, 0, 1, -1, -1],
                [-1, -1, 0, 0, 0, -1, -1],
            ],
        };
        let env = Solitaire::from_state(state.clone());
        let mut state_function = StateFunction::new();
        state_function.iterate_game(state, vec![], 25., &mut 0);
        
        println!("other: {:?}", state_function.qs.get(&env.hash_as_str()));
        println!("other: {:?}", state_function.qs);
    }

    #[test]
    fn test_iterate_game_endgame() {
        let state = SolitaireState {
            value: [
                [-1, -1, 0, 0, 0, -1, -1],
                [-1, -1, 1, 0, 0, -1, -1],
                [ 0,  0, 1, 0, 0,  0,  0],
                [ 0,  1, 0, 0, 0,  0,  0],
                [ 0,  0, 0, 0, 0,  0,  0],
                [-1, -1, 0, 0, 0, -1, -1],
                [-1, -1, 0, 0, 0, -1, -1],
            ],
        };
        let mut state_function = StateFunction::new();
        state_function.iterate_game(state, vec![], 0., &mut 0);
        
        let result = HashMap::from([
            (String::from("32_1523.795903_69.843619"), (1, 2.0)),
            (String::from("32_1565.69579_72.843619"), (1, 2.0)),
            (String::from("31_1430.068762_67.681342"), (1, 1.0)),
        ]);
        // {"32_1523.795903_69.843619": (1, 2.0), "32_1565.69579_72.843619": (1, 2.0), "31_1430.068762_67.681342": (1, 1.0)}
        // println!("other: {:?}", state_function.qs);
        // assert_eq!(result, state_function.qs);

    }

    #[test]
    fn test_insert_value() {
        let mut hash = StateFunction::new();
        hash.update_state_value_with_fn(&String::from("hello"), f64::max, 1.);
        hash.update_state_value_with_fn(&String::from("dummy"), f64::max, 100.);
        hash.update_state_value_with_fn(&String::from("hello"), f64::max, -100.);
        hash.update_state_value_with_fn(&String::from("hello"), f64::max, 1000.);

        let expected = HashMap::from([(String::from("hello"), (3, 1000.)),
                                      (String::from("dummy"), (1,  100.))]);

        assert_eq!(expected, hash.qs);
    }

    #[test]
    fn test_get_value() {
        let mut hash = StateFunction::new();
        hash.update_state_value_with_fn(&String::from("hello"), f64::max, 1.);
        hash.update_state_value_with_fn(&String::from("dummy"), f64::max, 100.);

        assert_eq!(Some(1.), hash.get_state_value(&String::from("hello")));
        assert_eq!(None, hash.get_state_value(&String::from("hello_other")));
    }

    #[test]
    fn test_get_least_seen_value() {
        let mut hash = StateFunction::new();
        hash.update_state_value_with_fn(&String::from("hello"), f64::max, 1.);
        hash.update_state_value_with_fn(&String::from("dummy"), f64::max, 100.);
        hash.update_state_value_with_fn(&String::from("hello"), f64::max, -100.);
        hash.update_state_value_with_fn(&String::from("hello"), f64::max, 1000.);

        assert_eq!(String::from("dummy"), hash.get_least_seen_state(vec![&String::from("hello"), &String::from("dummy")]));
        assert_eq!(String::from("never_seen"), hash.get_least_seen_state(vec![&String::from("hello"), &String::from("dummy"), &String::from("never_seen")]));
    }
}