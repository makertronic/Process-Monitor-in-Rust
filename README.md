# Process Monitor in Rust
This is a simple Rust program designed to monitor and automatically restart a specified command within a GNU Screen session. It periodically checks if the Screen session is running and launches it if it's not present.
Features

## Monitors a full command line (e.g., ./golden-miner-pool-prover --pubkey=...).
Runs the command in a detached Screen session for persistence.
Configurable check interval in seconds.
Uses screen -ls to detect running sessions and screen -dmS to start new ones.

## Usage
Compile and run the program with Cargo:
cargo build --release
./target/release/process_monitor --command "full command here" --time 60

## Example:
process_monitor --command "./miner-pool-prover --pubkey=XXXXXXXXXXXXXXXX --label=GPU --name=79503" --time 60
This will create a Screen session named miner-pool-prover-session (based on the executable name) and check every 60 seconds.

## Dependencies
None beyond the standard library. See Cargo.toml for details.

## License
MIT
