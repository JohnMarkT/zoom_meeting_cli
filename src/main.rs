use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Meetings {
    meetings: HashMap<String, String>,
}

const RESERVED_NAMES: &[&str] = &["add", "help", "join", "ls", "rm"];

fn print_help() {
    println!("Usage: zoom <command> [<meeting_name>]");
    println!("Commands:");
    println!("  ls            List all available meetings");
    println!("  join <name>   Join the specified meeting (or just use 'zoom <name>')");
    println!("  add <name> <id>  Add a new meeting with the specified name and ID");
    println!("  rm <name>     Remove the specified meeting");
    println!("  help          Display this help message");
}

fn open_zoom_meeting(meeting_id: &str) {
    let zoom_url = format!("zoommtg://zoom.us/join?confno={}", meeting_id);
    Command::new("open")
        .arg(&zoom_url)
        .spawn()
        .expect("Failed to open Zoom meeting");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        std::process::exit(1);
    }

    let command = &args[1];

    if command == "help" {
        print_help();
        return;
    }

    let meetings_file_path = env::var("ZOOMCLI_MEETINGS_FILE").unwrap_or_else(|_| "meetings.json".to_string());

    let mut file = File::open(&meetings_file_path).unwrap_or_else(|_| {
        eprintln!("Unable to open {}: No such file or directory", meetings_file_path);
        std::process::exit(1);
    });

    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read meetings.json");

    let mut meetings: Meetings = serde_json::from_str(&data).expect("Unable to parse meetings.json");

    match command.as_str() {
        "ls" => {
            for (name, id) in &meetings.meetings {
                println!("{}: {}", name, id);
            }
        }
        "join" => {
            if args.len() < 3 {
                eprintln!("Usage: zoom join <meeting_name>");
                std::process::exit(1);
            }

            let meeting_name = &args[2];
            let meeting_id: String = args[2..].join("").chars().filter(|c| c.is_digit(10)).collect();

            if !meeting_id.is_empty() && meeting_id.chars().all(char::is_numeric) {
                open_zoom_meeting(&meeting_id);
            } else if let Some(meeting_id) = meetings.meetings.get(meeting_name) {
                open_zoom_meeting(meeting_id);
            } else {
                eprintln!("Meeting '{}' not found", meeting_name);
                std::process::exit(1);
            }
        }
        _ if command.chars().all(char::is_numeric) => {
            let meeting_id: String = args.join("").chars().filter(|c| c.is_digit(10)).collect();
            if !meeting_id.is_empty() && meeting_id.chars().all(char::is_numeric) {
                open_zoom_meeting(&meeting_id);
            } else {
                eprintln!("Invalid meeting ID '{}'", command);
                std::process::exit(1);
            }
        }
        "add" => {
            if args.len() < 4 {
                eprintln!("Usage: zoom add <meeting_name> <meeting_id>");
                std::process::exit(1);
            }

            let meeting_name = &args[2];
            if RESERVED_NAMES.contains(&meeting_name.as_str()) {
                eprintln!("Error: '{}' is a reserved name and cannot be used as a meeting name", meeting_name);
                std::process::exit(1);
            }

            let meeting_id: String = args[3..].join("").chars().filter(|c| c.is_digit(10)).collect();
            if !matches!(meeting_id.len(), 9 | 10 | 11) {
                eprintln!("Error: Meeting ID must contain 9, 10, or 11 digits");
                std::process::exit(1);
            }

            meetings.meetings.insert(meeting_name.clone(), meeting_id.clone());

            let updated_data = serde_json::to_string_pretty(&meetings).expect("Unable to serialize meetings");
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&meetings_file_path)
                .expect("Unable to open meetings.json for writing");
            file.write_all(updated_data.as_bytes()).expect("Unable to write to meetings.json");

            println!("Meeting '{}' added with ID '{}'", meeting_name, meeting_id);
        }
        "rm" => {
            if args.len() != 3 {
                eprintln!("Usage: zoom rm <meeting_name>");
                std::process::exit(1);
            }

            let meeting_name = &args[2];

            if meetings.meetings.remove(meeting_name).is_some() {
                let updated_data = serde_json::to_string_pretty(&meetings).expect("Unable to serialize meetings");
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&meetings_file_path)
                    .expect("Unable to open meetings.json for writing");
                file.write_all(updated_data.as_bytes()).expect("Unable to write to meetings.json");

                println!("Meeting '{}' removed", meeting_name);
            } else {
                eprintln!("Meeting '{}' not found", meeting_name);
                std::process::exit(1);
            }
        }
        _ => {
            if let Some(meeting_id) = meetings.meetings.get(command) {
                open_zoom_meeting(meeting_id);
            } else {
                eprintln!("Unknown command: {}", command);
                print_help();
                std::process::exit(1);
            }
        }
    }
}
