use common::*;

pub fn run() {
    let internal_port = env_var(CONFIG.internal_port).unwrap();
    println!("Run on {}", internal_port);
}
