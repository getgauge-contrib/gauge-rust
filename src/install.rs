use common::*;

pub fn init() {
    let skel_dir = path_to(CONFIG.skel_path, Root::Plugin);
    let step_implementations_dir = path_to(CONFIG.step_implementations_path, Root::Project);
    let steps_file = CONFIG.steps_file;

    create_dir(&step_implementations_dir).unwrap();
    match copy_file(&skel_dir.join(&steps_file), &step_implementations_dir.join(&steps_file)) {
        Ok(_) => println!(" created {}", &steps_file),
        Err(e) => println!("Error: {}", e),
    }
}
