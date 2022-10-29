use std::fmt::{Display, Debug, Result, Formatter};
use std::hash::Hash;
use std::{cmp::Eq, ops::Add};
use std::path::Path;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Jump {
    Left = 0,
    Down = 1,
    Right = 2,
    Up = 3,
}

impl Jump {
    fn from(idx: usize) -> Self {
        match idx {
            0 => Jump::Left,
            1 => Jump::Down,
            2 => Jump::Right,
            3 => Jump::Up,
            num => panic!(
                "Please initialize only in the range 0 to 3, but received {}",
                num
            ),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct SolitaireAction {
    pub point: Point,
    pub action: Jump,
}

impl SolitaireAction {
    pub fn value(&self) -> (Point, Jump) {
        (self.point, self.action)
    }
}


#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct SolitaireState {
    pub value: [[i32; 7]; 7],
}


pub type StateT = [[i32; 7]; 7];
pub type ActionT = (Point, Jump);

// NOTE: we used the hashing function here
impl SolitaireState {
    fn value(&self) -> [[i32; 7]; 7] {
        self.value
    }
}

pub struct Solitaire {
    pub state: SolitaireState,
    pub holes: Vec<Point>,
}


pub fn get_empty_state() -> [[i32; 7]; 7] {
    let mut arr = [[0; 7]; 7];
    for row_idx in [0, 1, 5, 6] {
        arr[row_idx][0] = -1;
        arr[row_idx][1] = -1;
        arr[row_idx][5] = -1;
        arr[row_idx][6] = -1;
    }
    arr
}

pub fn get_start_state() -> [[i32; 7]; 7] {
    let mut arr = [[1; 7]; 7];
    for row_idx in [0, 1, 5, 6] {
        arr[row_idx][0] = -1;
        arr[row_idx][1] = -1;
        arr[row_idx][5] = -1;
        arr[row_idx][6] = -1;
    }
    arr[3][3] = 0;
    arr
}

impl Solitaire {
    fn _set_point_to_value(&mut self, p: Point, value: i32) {
        // only change value if it is inbound
        if p.x > 0 || p.y > 0 || p.x < 6 || p.y < 6 {
            self.state.value[p.y as usize][p.x as usize] = value
        }
    }

    fn _get_value_of_point(&self, p: Point) -> i32 {
        // return -1 if we are out of bounds
        if p.x < 0 || p.y < 0 || p.x > 6 || p.y > 6 {
            return -1;
        }
        self.state.value[p.y as usize][p.x as usize]
    }

    pub fn finished(&self) -> bool {
        match self.actions() {
            None => true,
            Some(_actions) => false,
        }
    }

    pub fn actions(&self) -> Option<Vec<SolitaireAction>> {
        let mut possible_actions = Vec::new();
        // todo: refactor. This works but is not really consice
        for hole in &self.holes {
            let right_jump = *hole + Point { x: -2, y: 0 };
            let t = self._get_value_of_point(right_jump);
            let pin = self._get_value_of_point(*hole + Point { x: -1, y: 0 });
            // let t = self.state.value()[right_jump.y as usize][right_jump.x as usize];
            if t == 1 && pin == 1 {
                possible_actions.push(SolitaireAction {
                    point: right_jump,
                    action: Jump::Right,
                });
            };

            let left_jump = *hole + Point { x: 2, y: 0 };
            let t = self._get_value_of_point(left_jump);
            let pin = self._get_value_of_point(*hole + Point { x: 1, y: 0 });
            // let t = self.state.value()[left_jump.y as usize][left_jump.x as usize];
            if t == 1 && pin == 1 {
                possible_actions.push(SolitaireAction {
                    point: left_jump,
                    action: Jump::Left,
                });
            };

            let down_jump = *hole + Point { x: 0, y: -2 };
            let t = self._get_value_of_point(down_jump);
            let pin = self._get_value_of_point(*hole + Point { x: 0, y: -1 });
            // let t = self.state.value()[down_jump.y as usize][down_jump.x as usize];
            if t == 1 && pin == 1 {
                possible_actions.push(SolitaireAction {
                    point: down_jump,
                    action: Jump::Down,
                });
            };

            let up_jump = *hole + Point { x: 0, y: 2 };
            let t = self._get_value_of_point(up_jump);
            // let t = self.state.value()[up_jump.y as usize][up_jump.x as usize];
            let pin = self._get_value_of_point(*hole + Point { x: 0, y: 1 });
            if t == 1 && pin == 1 {
                possible_actions.push(SolitaireAction {
                    point: up_jump,
                    action: Jump::Up,
                });
            };
        }
        match possible_actions.len() {
            0 => None,
            _ => Some(possible_actions),
        }
    }

    pub fn current_state(&self) -> StateT {
        self.state.value()
    }

    pub fn new() -> Self {
        let arr = get_start_state();
        Solitaire {
            state: SolitaireState { value: arr },
            holes: vec![Point { x: 3, y: 3 }],
        }
    }

    pub fn reset(&mut self) {
        let state = get_start_state();
        self.state = SolitaireState { value: state };
        self.holes = vec![Point { x: 3, y: 3 }];
    }

    pub fn simulate_action(&self, action: &ActionT) -> (SolitaireState, Vec<Point>) {
        let state = self.state;
        let mut holes = self.holes.clone();

        let (pin, jump) = action;
        let (new_pin, removed_pin) = match *jump {
            Jump::Down => (*pin + Point { x: 0, y: 2 }, *pin + Point { x: 0, y: 1 }),
            Jump::Up => (*pin + Point { x: 0, y: -2 }, *pin + Point { x: 0, y: -1 }),
            Jump::Left => (*pin + Point { x: -2, y: 0 }, *pin + Point { x: -1, y: 0 }),
            Jump::Right => (*pin + Point { x: 2, y: 0 }, *pin + Point { x: 1, y: 0 }),
        };
        if self._get_value_of_point(new_pin) != 0 {
            println!("This is self\n{}", self);
            println!("This is the intended action: pin {:?} jump {:?}", pin, jump)
        }
        assert_eq!(self._get_value_of_point(new_pin), 0);
        // IMPORTANT NOTE: self.state.value() is not easily modifiable, therefore we need self.state.value[...] here.
        let mut state = self.state.clone();
        state.value[new_pin.y as usize][new_pin.x as usize] = 1;
        state.value[pin.y as usize][pin.x as usize] = 0;
        state.value[removed_pin.y as usize][removed_pin.x as usize] = 0;

        let idx = holes
            .iter()
            .enumerate()
            .filter(|(_idx, &p)| p == new_pin)
            .map(|(idx, _p)| idx)
            .next()
            .unwrap();
        holes.remove(idx);
        holes.push(removed_pin);
        holes.push(*pin);

        holes.sort();
        
        (state, holes)
    }

    pub fn take_action(&mut self, action: &ActionT) -> f64 {
        let (state, holes) = self.simulate_action(action);
        self.state = state;
        self.holes = holes;

        // for now we just add one to the reward for each removed pin, but
        // we need to change this later on, since there needs to be a special reward if we end up in the middle
        1.
    }

    /// calculate the sum of distances between all points
    // pub fn hash(&self) -> (i8, f64, f64) {
    //     let length = self.holes.len();
    //     let mut sum : i32 = 0;
    //     // the polygon needs to be closed, therefore add the first point to the end of the vector
    //     let mut holes = self.holes.clone();
    //     holes.push(self.holes[0]);
    //     let mut min_distance: f64 = 100.;
    //     for (idx, h) in holes[..length].iter().enumerate() {
    //         let h_next = holes[idx + 1];
    //         sum += h.x - h_next.x - h_next.x * h.y;

    //         let new_dist = (( (h.x - 3).pow(2) + (h.y - 3).pow(2)) as f64).sqrt();
    //         if new_dist < min_distance {
    //             min_distance = new_dist;
    //         }
    //     }
    //     let a = 0.5 * sum as f64;
    //     (length as i8, a.abs(), min_distance)
    // }
    pub fn hash(&self) -> (i8, f64, f64) {
       Solitaire::hash_state(&self.state, &self.holes)
    }

    pub fn hash_as_str(&self) -> String {
        Solitaire::hash_state_as_string(&self.state, &self.holes)
    }

    pub fn hash_state(state: &SolitaireState, holes: &Vec<Point>) -> (i8, f64, f64) {
        let length = holes.len();
        let mut sum : f64 = 0.;
        let holes = holes.clone();
        let mut sum_mid: f64 = 0.;
        for (idx, h) in holes[..length-1].iter().enumerate() {
            for h_next in holes[idx..].iter() {
                let dist = (((h.x - h_next.x).pow(2) + (h.y - h_next.y).pow(2)) as f64).sqrt();
                sum += dist;
            }
            sum_mid += (( (h.x - 3).pow(2) + (h.y - 3).pow(2)) as f64).sqrt();
        }
        // add last hole to sum_mid
        let lh = holes[length - 1];
        sum_mid += (( (lh.x - 3).pow(2) + (lh.y - 3).pow(2)) as f64).sqrt();
        (length as i8, (sum * 1_000_000.).round() / 1_000_000. , (sum_mid * 1_000_000.).round() / 1_000_000.)
    }

    pub fn hash_state_as_string(state: &SolitaireState, holes: &Vec<Point>) -> String {
        let (num_holes, sum_of_dist, sum_of_dist_to_origin) = Solitaire::hash_state(state, holes);
        let s = format!("{}_{}_{}", num_holes, sum_of_dist, sum_of_dist_to_origin);
        s
    }


    pub fn get_holes_from_state(state: &SolitaireState) -> Vec<Point> {
        let mut holes = Vec::new();
        for (ridx, row) in state.value.iter().enumerate() {
            for (cidx, value) in row.iter().enumerate() {
                if *value == 0 {
                    holes.push(Point{ x: cidx as i32, y: ridx as i32});
                }
            }
        }
        holes
    }

    pub fn from_state(state: SolitaireState) -> Self {
        let mut holes = Solitaire::get_holes_from_state(&state);
        holes.sort();
        Solitaire {
            state: state,
            holes: holes,
        }
    }
}

impl Display for Solitaire {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in self.state.value.iter() {
            let v: String = row
                .iter()
                .map(|val| match val {
                    1 => String::from(" x "),
                    _ => String::from("   "),
                })
                .fold(String::new(), |a, b| a + &b);
            writeln!(f, "{}", v).unwrap();
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taking_solitaire_actions() {
        let mut env = Solitaire::new();

        let action = SolitaireAction {
            action: Jump::Down,
            point: Point { x: 3, y: 1 },
        };
        println!("This is env\n{}", env);
        env.take_action(&action.value());
        println!("This is env\n{}", env);
        assert_eq!(env.holes, vec![Point { x: 3, y: 1 }, Point { x: 3, y: 2 }]);

        let action = SolitaireAction {
            action: Jump::Right,
            point: Point { x: 1, y: 2 },
        };
        env.take_action(&action.value());
        assert_eq!(
            env.holes,
            vec![
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 1 },
            ]
        );
    }

    #[test]
    fn test_get_start_state() {
        let arr = get_start_state();
        let start_state = [
            [-1, -1, 1, 1, 1, -1, -1],
            [-1, -1, 1, 1, 1, -1, -1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 0, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [-1, -1, 1, 1, 1, -1, -1],
            [-1, -1, 1, 1, 1, -1, -1],
        ];
        assert_eq!(arr, start_state);
    }

    #[test]
    fn test_get_empty_state() {
        let arr = get_empty_state();
        let start_state = [
            [-1, -1, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, -1, -1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [-1, -1, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, -1, -1],
        ];
        assert_eq!(arr, start_state);
    }

    #[test]
    fn test_simulate_action() {

        let state = SolitaireState {
            value: [
                [-1, -1, 0, 1, 0, -1, -1],
                [-1, -1, 1, 1, 0, -1, -1],
                [ 0,  0, 1, 0, 0,  0,  0],
                [ 0,  0, 0, 0, 0,  0,  0],
                [ 0,  0, 1, 0, 0,  0,  0],
                [-1, -1, 1, 0, 0, -1, -1],
                [-1, -1, 0, 0, 0, -1, -1],
            ],
        };

        let mut env = Solitaire::from_state(state);
        let (result, holes) = env.simulate_action(&(Point { x: 2, y: 1 }, Jump::Right));

        let expected = SolitaireState {
            value: [
                [-1, -1, 0, 1, 0, -1, -1],
                [-1, -1, 0, 0, 1, -1, -1],
                [ 0,  0, 1, 0, 0,  0,  0],
                [ 0,  0, 0, 0, 0,  0,  0],
                [ 0,  0, 1, 0, 0,  0,  0],
                [-1, -1, 1, 0, 0, -1, -1],
                [-1, -1, 0, 0, 0, -1, -1],
            ],
        };
        assert_eq!(result, expected);
        // check that the state of env is unchanged
        assert_eq!(env.state, state);
    }

    /// THIS WHOLE BLOG DOES TOUGH SHIT.
    /// IT TRIES TO "PROOF" THAT OUR HASHING METHOD IS CORRECT
    /// check if rotation works correctly
    // helper functions to check rotation

    pub fn _rotate_90(state: &SolitaireState) -> SolitaireState {
        let mut s90 = [[-1; 7]; 7];
        for (row_idx, row) in state.value().iter().rev().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                s90[6 - col_idx][6 - row_idx] = *val
            }
        }
        SolitaireState { value: s90 }
    }

    pub fn _rotate_180(state: &SolitaireState) -> SolitaireState {
        let mut s180 = [[-1; 7]; 7];
        for (row_idx, row) in state.value().iter().rev().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                s180[row_idx][6 - col_idx] = *val
            }
        }
        SolitaireState { value: s180 }
    }

    pub fn _rotate_270(state: &SolitaireState) -> SolitaireState {
        let mut s270 = [[-1; 7]; 7];
        for (row_idx, row) in state.value().iter().rev().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                s270[col_idx][row_idx] = *val
            }
        }
        SolitaireState { value: s270 }
    }

    pub fn _mirror(state: &SolitaireState) -> SolitaireState {
        let mut mirrored = [[-1; 7]; 7];
        for (row_idx, row) in state.value().iter().rev().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                mirrored[row_idx][col_idx] = *val
            }
        }
        SolitaireState { value: mirrored }
    }

    pub fn _mirror_90(state: &SolitaireState) -> SolitaireState {
        let mut mirrored90 = [[-1; 7]; 7];
        for (row_idx, row) in state.value().iter().rev().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                mirrored90[6 - col_idx][row_idx] = *val
            }
        }
        SolitaireState { value: mirrored90 }
    }

    pub fn _mirror_180(state: &SolitaireState) -> SolitaireState {
        let mut mirrored180 = [[-1; 7]; 7];
        for (row_idx, row) in state.value().iter().rev().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                mirrored180[6 - row_idx][6 - col_idx] = *val
            }
        }
        SolitaireState { value: mirrored180 }
    }

    pub fn _mirror_270(state: &SolitaireState) -> SolitaireState {
        let mut mirrored270 = [[-1; 7]; 7];
        for (row_idx, row) in state.value().iter().rev().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                mirrored270[col_idx][6 - row_idx] = *val
            }
        }
        SolitaireState { value: mirrored270 }
    }

    pub fn generate_random_state() {

    }


    #[test]
    fn test_hash_3_holes() {
        let state = SolitaireState {
            value: [
                [-1, -1, 1, 1, 1, -1, -1],
                [-1, -1, 1, 1, 1, -1, -1],
                [ 1,  1, 1, 1, 1,  1,  1],
                [ 1,  1, 1, 1, 1,  0,  1],
                [ 1,  1, 1, 1, 0,  1,  1],
                [-1, -1, 1, 1, 0, -1, -1],
                [-1, -1, 1, 1, 1, -1, -1],
            ],
        };
        let mut env = Solitaire::from_state(state);
        let (num_holes, sum_of_dist, sum_of_dist_to_origin) = env.hash();
        assert_eq!(num_holes, 3);
        assert_eq!(sum_of_dist, 4.650282);
        assert_eq!(sum_of_dist_to_origin, 5.650282);

        let arr: Vec<fn(&SolitaireState) -> SolitaireState> = vec![_rotate_90, _rotate_180, _rotate_270, _mirror, _mirror_90, _mirror_180, _mirror_270];

        for f in &arr {
            let env = Solitaire::from_state(f(&state));
            println!("This is the env\n{}", env);
            let (num_holes, sum_of_dist, sum_of_dist_to_origin) = env.hash();
            assert_eq!(num_holes, 3);
            assert_eq!(sum_of_dist, 4.650282);
            assert_eq!(sum_of_dist_to_origin, 5.650282);
        }
    }

    #[test]
    fn test_single_states() {
        let state = SolitaireState { value: 
         [
            [-1, -1, 1, 1, 0, -1, -1], 
            [-1, -1, 1, 1, 1, -1, -1], 
            [0,   1, 1, 1, 1,  1,  0], 
            [1,   1, 1, 1, 1,  1,  1], 
            [0,   1, 1, 1, 1,  0,  0], 
            [-1, -1, 1, 1, 1, -1, -1], 
            [-1, -1, 1, 1, 1, -1, -1]
         ] 
        };
        let env = Solitaire::from_state(state);
        let (num_holes, area, min_dist) = env.hash();
        println!("These are the holes {:?}", env.holes);
        println!("This is env\n{}\n\nAnd the num holes {}, area {}, min_dist {}", env, num_holes, area, min_dist);

        let env = Solitaire::from_state(_rotate_90(&state));
        println!("These are the holes {:?}", env.holes);
        let (new_num_holes, new_area, new_min_dist) = env.hash();
        println!("This is env2\n{}\n\nAnd the num holes {}, area {}, min_dist {}", env, new_num_holes, new_area, new_min_dist);

        assert_eq!(num_holes, new_num_holes);
        assert_eq!(area, new_area);
        assert_eq!(min_dist, new_min_dist);
    }

    #[test]
    fn test_hash_state_as_string() {
        let state = SolitaireState {
            value: [
                [-1, -1, 1, 1, 1, -1, -1],
                [-1, -1, 1, 1, 1, -1, -1],
                [ 1,  1, 1, 1, 1,  1,  1],
                [ 1,  1, 1, 1, 1,  0,  1],
                [ 1,  1, 1, 1, 0,  1,  1],
                [-1, -1, 1, 1, 0, -1, -1],
                [-1, -1, 1, 1, 1, -1, -1],
            ],
        };

        let env = Solitaire::from_state(state);
        let result = Solitaire::hash_state_as_string(&env.state, &env.holes);
        assert_eq!(String::from("3_4.650282_5.650282"), result);

    }

    #[test]
    fn test_randomly_generated_state() {
        let arr: Vec<fn(&SolitaireState) -> SolitaireState> = vec![_rotate_90, _rotate_180, _rotate_270, _mirror, _mirror_90, _mirror_180, _mirror_270];

        for _ in 0..10_000 {
            let mut rng = rand::thread_rng();
            let state = SolitaireState {
                value: [
                    [-1, -1, rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2), -1, -1],
                    [-1, -1, rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2), -1, -1],
                    [ rng.gen_range(0..2),  rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2),  rng.gen_range(0..2),  rng.gen_range(0..2)],
                    [ rng.gen_range(0..2),  rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2),  rng.gen_range(0..2),  rng.gen_range(0..2)],
                    [ rng.gen_range(0..2),  rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2),  rng.gen_range(0..2),  rng.gen_range(0..2)],
                    [-1, -1, rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2), -1, -1],
                    [-1, -1, rng.gen_range(0..2), rng.gen_range(0..2), rng.gen_range(0..2), -1, -1],
                ],
            };
            let env = Solitaire::from_state(state);

            let (num_holes, area, min_dist) = env.hash();
            for f in &arr {
                let env = Solitaire::from_state(f(&state));
                let (new_num_holes, new_area, new_min_dist) = env.hash();
                assert_eq!(num_holes, new_num_holes);
                assert_eq!(area, new_area);
                assert_eq!(min_dist, new_min_dist);
            }
        }
    }
}

