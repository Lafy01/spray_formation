use rand::{distributions::Alphanumeric, Rng};


pub fn generate_random_id(prefix: &str) -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    
    format!("{}_{}", prefix, random_string)
}