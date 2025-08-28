# Todo API

A simple REST API for managing todos built with Rust, Axum, and Diesel.

## Features

- ✅ Create todos
- ✅ List all todos
- ✅ Get specific todo by ID
- ✅ Update todos
- ✅ Delete todos
- ✅ PostgreSQL database with UUID primary keys
- ✅ Automatic timestamps (created_at, updated_at)

## Prerequisites

- Rust (latest stable)
- PostgreSQL database
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

## Setup

1. **Database Setup**
   ```bash
   # Make sure PostgreSQL is running
   # Create database (if not exists)
   createdb test123
   ```

2. **Environment Configuration**
   ```bash
   # The .env file should contain:
   DATABASE_URL=postgres://postgres:1729@localhost:5432/test123
   ```

3. **Run Migrations**
   ```bash
   diesel migration run
   ```

4. **Build and Run**
   ```bash
   cargo build
   cargo run
   ```

The server will start on `http://localhost:8001`

## API Endpoints

### GET /todos
Get all todos
```bash
curl http://localhost:8001/todos
```

### POST /todos
Create a new todo
```bash
curl -X POST http://localhost:8001/todos \
  -H "Content-Type: application/json" \
  -d '{"todo": "Learn Rust", "done": false}'
```

### GET /todos/:id
Get a specific todo by ID
```bash
curl http://localhost:8001/todos/{todo-id}
```

### PUT /todos/:id
Update a todo
```bash
curl -X PUT http://localhost:8001/todos/{todo-id} \
  -H "Content-Type: application/json" \
  -d '{"todo": "Learn Rust with Axum", "done": true}'
```

### DELETE /todos/:id
Delete a todo
```bash
curl -X DELETE http://localhost:8001/todos/{todo-id}
```

## Testing

Run the Python test script:
```bash
python test_api.py
```

## Project Structure

```
backend/api/
├── src/
│   ├── main.rs          # Main application and route handlers
│   ├── models.rs        # Database models and structs
│   └── schema.rs        # Generated database schema
├── migrations/          # Database migrations
├── Cargo.toml          # Dependencies
├── .env                # Environment variables
└── diesel.toml         # Diesel configuration
```