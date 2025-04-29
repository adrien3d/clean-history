use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let user_dirs = directories::UserDirs::new().expect("Failed to get user directories");
    let input_path = user_dirs.home_dir().join(".zsh_history");
    let output_path = user_dirs.home_dir().join(".zsh_history_cleaned");

    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut seen = HashSet::new();
    let mut unique_commands = Vec::new();

    // We reverse to keep the *last* occurrence
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
    for line in lines.iter().rev() {
        // Extract command portion from the zsh history line
        let command = if let Some(index) = line.find(";") {
            &line[index + 1..]
        } else {
            line
        };

        if !seen.contains(command) {
            seen.insert(command.to_string());
            unique_commands.push(line.clone());
        }
    }

    unique_commands.reverse(); // Restore original order (keeping last occurrences)

    let mut output = File::create(&output_path)?;
    for line in unique_commands {
        writeln!(output, "{}", line)?;
    }

    println!("De-duplicated history written to {:?}", &output_path);
    Ok(())
}
