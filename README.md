# Trident Project

This project (with name obviously playing with Neptune name) provides an API for calculating statistics on batches of numerical data.

## Building the Project

1.  Make sure you have Rust and Cargo installed.
2.  Navigate to the project directory in your terminal.
3.  Run the following command to build the project:

    ```bash
    cargo build
    ```

## Running the Project

1.  After building, run the project using:

    ```bash
    cargo run
    ```
    This will start the server, which listens on `localhost:3000`.

## Testing

To run the test suite:

```bash
cargo test
```

## Load Testing

Run load testing with drill:

```bash
cargo install drill
drill --benchmark benchmark.yml --stats
```

## Sample Curl Commands

### Add Batch

To add a batch of values for a given symbol, use the following command:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"symbol": "AAPL", "values": [1.0, 2.0, 3.0]}' http://localhost:3000/add_batch
```

### Get Stats

To retrieve statistics for a given symbol and `k` value, where the k is number of `1e{k}` prices for calculation use the following command:

```bash
curl "http://localhost:3000/stats?symbol=AAPL&k=3"