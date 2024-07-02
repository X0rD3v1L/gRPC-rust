# gRPC Calculator Example

This project demonstrates a simple gRPC-based calculator service with an admin interface using Rust and the `tonic` crate.

## Prerequisites

Before running this project, ensure you have the following installed:

- Rust (including Cargo)
- Protobuf compiler (`protoc`)
- `tonic` crate (included via Cargo dependencies)

## Project Structure

The project consists of the following files:

- **`client.rs`**: Example client code that connects to the calculator service and performs addition and division operations.
- **`calculator.proto`**: Protocol Buffer file defining the gRPC service and message types for the calculator service.
- **`main.rs`**: Server implementation that hosts the calculator service and admin interface, including request count tracking and authentication.

## Setup

1. Clone the repository:

   ```bash
   git clone <repository_url>
   cd <repository_directory>
   ```

2. Install dependencies:

   ```bash
   cargo build
   ```

## Running the Server

To start the gRPC server, run:

```bash
cargo run --bin server
```

This will start the server at `localhost:50051`.

## Using the Client

The client code (`client.rs`) demonstrates how to interact with the calculator service:

```bash
cargo run --bin client
```

This will connect to the server and perform addition and division operations.

## Additional Notes

- **Authentication**: The server implements a basic authentication mechanism (`check_auth` function) using an authorization token.
- **Error Handling**: Error handling is implemented throughout the server and client code to manage errors gracefully.
- **Concurrency**: The server utilizes Tokio for asynchronous operations to handle multiple client requests concurrently.