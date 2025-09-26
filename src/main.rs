use std::env;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
use std::string::ToString;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 || args[1] != "--command" || args[3] != "--time" {
        println!("Usage: {} --command \"full command line\" --time <interval>", args[0]);
        return;
    }

    let full_cmd = args[2].clone();
    let interval: u64 = match args[4].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid interval: must be a number");
            return;
        }
    };

    let cmd_parts: Vec<&str> = full_cmd.split_whitespace().collect();
    if cmd_parts.is_empty() {
        println!("Invalid command");
        return;
    }

    let exe = cmd_parts[0].to_string();
    let proc_args: Vec<String> = cmd_parts[1..].iter().map(|&s| s.to_string()).collect();

    let path = Path::new(&exe);
    let process_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let session_name = format!("{}-session", process_name);

    println!("Monitoring session '{}' for command '{}' every {} seconds", session_name, full_cmd, interval);

    loop {
        if !is_session_running(&session_name) {
            println!("Session '{}' not found. Starting...", session_name);
            let mut cmd = Command::new("screen");
            cmd.arg("-dmS").arg(&session_name).arg(&exe);
            for arg in &proc_args {
                cmd.arg(arg);
            }
            match cmd.spawn() {
                Ok(_) => println!("Successfully started session '{}'", session_name),
                Err(e) => println!("Failed to start session '{}': {}", session_name, e),
            }
        } else {
            println!("Session '{}' is running.", session_name);
        }

        sleep(Duration::from_secs(interval));
    }
}

fn is_session_running(session: &str) -> bool {
    let output = Command::new("screen")
        .arg("-ls")
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            stdout.lines().any(|line| {
                line.contains(&format!(".{}", session)) &&
                (line.contains("(Detached)") || line.contains("(Attached)"))
            })
        }
        Err(_) => {
            println!("Failed to execute screen -ls. Assuming session is not running.");
            false
        }
    }
}