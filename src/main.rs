mod file_processing;
mod utils;

use file_processing::{process_files, get_all_log_and_txt_files, load_state};
use std::io;
use std::env;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: <folder_path> [interval_in_seconds]");
        return Ok(());
    }

    let folder_path = &args[1];
    let interval_secs: Option<u64> = if args.len() > 2 {
        args[2].parse().ok()
    } else {
        None
    };

    let state_file_path = Path::new(folder_path).join("file_state.json");
    let state_file = state_file_path.to_str().unwrap();

    let mut file_state = load_state(state_file)?;

    // Process the initial files once
    let initial_files = get_all_log_and_txt_files(Path::new(folder_path))?;
    process_files(&mut file_state, initial_files, state_file)?;

    // If no interval is provided, just run once and exit
    if interval_secs.is_none() {
        return Ok(());
    }

    // Set up file watcher if interval is provided
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(Path::new(folder_path), RecursiveMode::Recursive).unwrap();

    // Get the interval from the provided argument, or use the default (10 seconds)
    let interval_secs = interval_secs.unwrap_or(10);

    // Continuous loop with the specified interval
    loop {
        let mut events = vec![];

        thread::sleep(Duration::from_secs(interval_secs));

        while let Ok(event) = rx.try_recv() {
            if let Ok(event) = event {
                events.push(event);
            }
        }

        let mut files_to_check = vec![];

        for event in events {
            let Event { paths, .. } = event;
            for path in paths {
                if let Some(extension) = path.extension() {
                    if extension == "log" || extension == "txt" {
                        files_to_check.push(path);
                    }
                }
            }
        }

        process_files(&mut file_state, files_to_check, state_file)?;
    }
}
