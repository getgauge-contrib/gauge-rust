use common::*;

pub fn init() {
    let steps_file = &CONFIG.steps_file;

    create_dir(&CONFIG.step_implementations_path).unwrap();
    match copy_file(&CONFIG.skel_path.join(&steps_file), &CONFIG.step_implementations_path.join(&steps_file)) {
        Ok(_) => println!(" create {}", &steps_file),
        Err(e) => println!("Error: {}", e),
    }
}
