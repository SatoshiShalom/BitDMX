# Backend Layer

Rollup node simulator, batch builder, and proof aggregator for BitVMX-Z.

## Components

- **rollup.rs**: Rollup node state management and transaction processing
- **batch.rs**: Transaction batch builder
- **aggregator.rs**: Recursive proof aggregation
- **api.rs**: REST API endpoints
- **storage.rs**: SQLite persistence layer

## Running the Node

```bash
cargo run --bin bitvmx-z-node
```

The node will start on `http://localhost:3000`.

## API Endpoints

- `GET /health` - Health check
- `GET /state` - Get current rollup state
- `POST /submit` - Submit transaction
- `GET /batches` - Get batch history

## Example Request

```bash
curl -X POST http://localhost:3000/submit \
  -H "Content-Type: application/json" \
  -d '{"from": "alice", "to": "bob", "amount": 1000}'
```
