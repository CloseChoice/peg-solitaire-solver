use rl::brute_force_solver::brute_force_solving;
use rl::peg_solitaire_environment::{Solitaire, SolitaireState, get_start_state};
use rl::state_function::StateFunction;
use std::collections::HashMap;
use std::fs;
use clap::Parser;
use std::path::Path;
// use serde_json;
use std::fmt::Write;
use mysql::*;
use mysql::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   db: String,

   /// Number of times to greet
   #[arg(short, long)]
   password: String,

   #[arg(long, default_value_t = 3306)]
   port: u32,

   #[arg(long)]
   host: String,

   #[arg(long, short)]
   user: String,
}

fn build_connection_string() -> String {
    let args = Args::parse();
    let mut s = String::new();
    write!(&mut s, "mysql://{}:{}@{}:{}/{}", args.user, args.password, args.host, args.port, args.db);

    s
}

struct PegSolitaireValues {
     holes: i32,
     hash: String,
     value: f64,
     position: String,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = build_connection_string();
    // let url = "mysql://tobias:uH9rBpF4uN7gGBwG@localhost:3306/peg_solitaire";
    println!("This is the url {:?}", url);
    // let url = "mysql://root:password@localhost:3307/db_name";
    let pool = Pool::new(url.as_str())?;

    let mut conn = pool.get_conn()?;
    println!("Connection created");

    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TABLE if not exists `peg_solitaire_values_test` (
          `hash` varchar(100) NOT NULL,
          `value` int DEFAULT NULL,
          `holes` int NOT NULL,
          `position` varchar(100) NOT NULL,
          PRIMARY KEY (`hash`,`holes`)
          )
        PARTITION BY LIST(holes) (
            PARTITION pHoles_1 VALUES IN (1,2,3,4,5,6,7,8,9,10,11,12,13),
            PARTITION pHoles_2 VALUES IN (14,15),
            PARTITION pHoles_3 VALUES IN(16),
            PARTITION pHoles_4 VALUES IN(17),
            PARTITION pHoles_5 VALUES IN(18),
            PARTITION pHoles_6 VALUES IN(19),
            PARTITION pHoles_7 VALUES IN(20),
            PARTITION pHoles_8 VALUES IN(21),
            PARTITION pHoles_9 VALUES IN(22),
            PARTITION pHoles_10 VALUES IN(23),
            PARTITION pHoles_11 VALUES IN(24),
            PARTITION pHoles_12 VALUES IN(25),
            PARTITION pHoles_13 VALUES IN(26, 27, 28, 29, 30, 31, 32)
        );")?;
     println!("Hello, world!");

     let mut s = StateFunction::new();
     let state = SolitaireState {
          value: [
              [-1, -1, 0, 0, 0, -1, -1],
              [-1, -1, 0, 0, 0, -1, -1],
              [ 0,  0, 0, 0, 0,  0,  0],
              [ 0,  0, 0, 0, 1,  1,  0],
              [ 0,  0, 0, 0, 0,  0,  0],
              [-1, -1, 0, 0, 0, -1, -1],
              [-1, -1, 0, 0, 0, -1, -1],
          ],
         };

//     let state = SolitaireState {
//          value: [
//              [-1, -1, 1, 1, 1, -1, -1],
//              [-1, -1, 1, 1, 1, -1, -1],
//              [ 1,  1, 1, 1, 1,  1,  1],
//              [ 1,  1, 1, 0, 1,  1,  1],
//              [ 1,  1, 1, 1, 1,  1,  1],
//              [-1, -1, 1, 1, 1, -1, -1],
//              [-1, -1, 1, 1, 1, -1, -1],
//          ],
//         };
     s.iterate_game(state, vec![], vec![], 0., &mut 0);
    
//     let s = brute_force_solving(50_000_000);
     // let s = brute_force_solving(100);
 
     let state = Solitaire::new().hash_as_str();
 
     // let state = env.hash_as_str();
     println!("length of s {}", s.qs.len());
//     println!("This is the value of the start state {:?} and it's appearance {}", s.get_state_value(&state), s.get_state_counter(&state));
     let b = s.qs.into_iter().map(|(k, v)| {
                                             let split = k.split("_").collect::<Vec<&str>>();
                                             let holes = split[0].parse::<i32>().unwrap();
                                               PegSolitaireValues {
                                               holes: holes.into(),
                                               hash: k,
                                               value: v.1,
                                               position: v.2
                                             }
                                           }
                                  ).collect::<Vec<PegSolitaireValues>>();

     // let b: HashMap<String, (f64, String)> = s.qs.into_iter().map(|(k, v)| (k, (v.1, v.2))).collect();
    conn.exec_batch(
        r"INSERT INTO peg_solitaire_values_test(hash, holes, value, position)
          VALUES (:hash, :holes, :value, :position)",
        b.iter().map(|p| params! {
            "holes" => &p.holes,
            "hash" => &p.hash,
            "position" => &p.position,
            "value" => &p.value,
        })
    )?;
// 
//     let serialized_json = serde_json::to_string(&b).unwrap();
//     let path_json: &Path = Path::new("serialized_deep_search_hole_peg_dist.json");
//     // let path_json: &Path = Path::new("serialized_with_new_hash.json");
//     fs::write(path_json, serialized_json).unwrap();
     Ok(())
}
