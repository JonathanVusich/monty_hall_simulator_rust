extern crate rand;

use std::time::Duration;
use std::thread;
use rand::distributions::uniform::Uniform;
use rand::Rng;
use rand::thread_rng;

fn main() {
    println!("{:?}", generate_successes(false, 100000000));
    thread::sleep(Duration::from_secs(3));
}

fn generate_successes(user_switch: bool, iterations: usize) -> usize {
    let static_door_set = [[0, 1, 1], [1, 0, 1], [0, 1, 1]];
    let mut successes = 0;
    let door_set = generate_vector(iterations);
    let door_choice = generate_vector(iterations);
    for x in 0..iterations {
        if user_switch {
            if static_door_set[door_set[x]][door_choice[x]] == 1 {
                successes+=1;
            }
        } else {
            if static_door_set[door_set[x]][door_choice[x]] == 0 {
                successes+=1;
            };
        }
    }
    return successes;
}

fn generate_vector(iterations: usize) -> Vec<usize> {
    let range = Uniform::new(0, 3);
    let random_vec = thread_rng().sample_iter(&range).take(iterations).collect();
    return random_vec;
}