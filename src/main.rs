use std::fs;
use std::process::Command;

fn main() {
    pretty_env_logger::init();

    let user_dirs = directories::UserDirs::new().expect("Failed to get user directories");
    let history_path = user_dirs.home_dir().join(".zsh_history_test");

    // let mut file = OpenOptions::new()
    //     .open(&history_path)
    //     .expect("Failed to open file");

    // let mut writer = BufWriter::new(&mut file);

    let history = fs::read_to_string(history_path.clone()).expect("Unable to read file");
    let cleaned_history = history.replace("\n", " \n");

    fs::write(history_path, cleaned_history).expect("Unable to write file");

    Command::new("history")
        .status()
        .expect("Failed to execute command");
}
