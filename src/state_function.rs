use crate::peg_solitaire_environment::{StateT, ActionT};
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
}


mod tests {
    use super::*;

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