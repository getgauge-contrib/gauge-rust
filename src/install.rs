use std::fs;
use std::path::PathBuf;
use std::env;

fn create_dir(dirpath: &PathBuf) {
    fs::create_dir_all(dirpath).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

fn copy_file(from: &PathBuf, to: &PathBuf) {
    match fs::copy(from, to) {
        Ok(_) => println!("Created: {:?}", to.file_name().unwrap()),
        Err(e) => {
            panic!("Could not create {:?}. Error: {}",
                   to.file_name().unwrap(),
                   e)
        }
    }
}

pub fn init() {
    let steps_file: &str = "steps.rs";

    let project_root: PathBuf = PathBuf::from(env::var("GAUGE_PROJECT_ROOT").unwrap());
    let plugin_dir: PathBuf = env::current_dir().unwrap();

    let step_implementations_dir: PathBuf = project_root.join("step_implementations");
    let skel_dir: PathBuf = plugin_dir.join("skel");

    create_dir(&step_implementations_dir);
    copy_file(&skel_dir.join(steps_file),
              &step_implementations_dir.join(steps_file));
}
