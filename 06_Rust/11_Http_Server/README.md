# Async Project Management API (Axum + Tokio)

A fully asynchronous, multithreaded REST API built using **Rust**, **Axum**, and **Tokio**.  
This project demonstrates proper async architecture, shared state management, and isolated blocking file I/O.

---

## Features

- Async REST API using Axum
- Tokio multi-threaded runtime
- Thread-safe shared state (`Arc<RwLock<...>>`)
- Blocking file I/O isolated using `spawn_blocking`
- Full CRUD for:
  - Projects
  - Tasks (nested under projects)
- Structured console performance logs
- JSON file-based persistence

---

## Tech Stack

- Rust
- Axum
- Tokio (multi-thread runtime)
- Serde
- UUID

---

## Project Structure
src/
├── main.rs 
├── api.rs 
├── handler.rs
├── model.rs 
└── route.rs 

---

## API Endpoints

### Project Endpoints

| Method | Endpoint              |
|--------|-----------------------|
| GET    | /projects             |
| GET    | /projects/:id         |
| POST   | /projects             |
| PUT    | /projects/:id         |
| DELETE | /projects/:id         |

---

### Task Endpoints

| Method | Endpoint                              |
|--------|----------------------------------------|
| GET    | /projects/:id/tasks                   |
| GET    | /projects/:id/tasks/:task_id          |
| POST   | /projects/:id/tasks                   |
| PUT    | /projects/:id/tasks/:task_id          |
| DELETE | /projects/:id/tasks/:task_id          |

---

## Data Storage

All data is persisted locally in  `projects.json`

---

## Running the Application

```bash
cargo run
```
