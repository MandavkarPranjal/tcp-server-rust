# TCP Server in Rust

This project is a simple TCP server implemented in Rust, designed for educational purposes. It demonstrates how to handle TCP connections and read from and write to sockets asynchronously using the Tokio library.

## Installation

To install this project, follow these steps:

1. **Clone the repository:**
    ```sh
    git clone https://github.com/MandavkarPranjal/tcp-server-rust.git
    cd tcp-server-rust
    ```

2. **Install Rust and Cargo:**
   If you haven't installed Rust and Cargo yet, follow the instructions on [rustup.rs](https://rustup.rs/) to install them.

3. **Build the project:**
    ```sh
    cargo build
    ```

## Running the Server

To run the TCP server, you have two options: using the default configuration file or specifying a custom configuration file.

### Using the Default Configuration File

The default configuration file is located at `src/configuration/config.json`. To run the server with this configuration, simply execute:

```sh
cargo run
```

## Using a Custom Configuration File
If you want to use a custom configuration file, specify the file path with the --config (or -c) flag:
```sh
cargo run -- --config path/to/your/config.json
```

## Example Configuration File
The configuration file should be in JSON format and look like this:
```json
{
  "address": "127.0.0.1",
  "port": 7878
}
```
- address: The IP address on which the server will listen for incoming connections.
- port: The port number on which the server will listen.

## Explanation
This TCP server is built to handle multiple connections asynchronously using the Tokio library. It listens for incoming connections on a specified address and port, and echoes back any data received from clients.

## Key Features
- Asynchronous Handling: Uses Tokio for asynchronous networking, allowing the server to handle multiple clients concurrently.
- Configuration: Server configuration (address and port) can be specified through a JSON file.
- Graceful Shutdown: Handles shutdown gracefully when receiving a termination signal (Ctrl+C).

## Project Structure
- src/main.rs: The main entry point of the application. Sets up the server and handles incoming connections.
- configuration/config.json: The default configuration file for the server.

## Note
This project is built for educational purposes. It serves as a basic example of how to create a TCP server in Rust using asynchronous programming with Tokio. For production use, additional features and error handling would be necessary.

Feel free to explore and modify the code to better understand how it works and to fit your learning needs.
