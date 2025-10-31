# Rust Backend Architecture

## Overview
This backend is designed as a central hub for managing trading data, Alpaca API interactions, and ML predictions. It serves as a bridge between the C++ frontend (trading system) and the Python ML modules. The architecture emphasizes memory safety, asynchronous execution, and modularity.

Key components:
- Database / Repository Layer
- Controllers (business logic)
- TCP / Networking
- Threading and Task Queue
- Main 
- Testing and Validation

## Database / Repository Layer
**Purpose:** Persistent storage, CRUD operations, schema management, and data validation.

**Design Highlights:**
- Singleton database connection shared across all repositories.
- Configuration via `.env` for credentials and connection parameters.
- Connection pooling with idle timeout to maintain performance.
- Retry strategy for transient database errors.
- Migrations and schema versioning handled during startup.
- Each table has a corresponding repository for encapsulated CRUD operations.

## Controllers
**Purpose:** Encapsulate business logic, validation, and repository orchestration.

**Implemented Controllers:**
- Alpaca Account
- Alpaca Asset (Instrument)
- Alpaca Order
- Alpaca Position

**Planned Controllers:**
- Trade
- Market Data Ticks
- ML Model
- ML Prediction
- Labeled Data
- Feature Sets

**Responsibilities:**
- Validate incoming data.
- Upsert or query relevant repositories.
- Handle domain-specific rules (e.g., account cash ≥ 0).

## TCP / Networking
**Purpose:** Facilitate communication with the C++ frontend and Python ML modules.

**Design Highlights:**
- Binary protocol with LittleEndian encoding.
- Structs and enums mirrored between Rust and C++.
- Message deserialization for numeric types, strings, and UTC timestamps.
- Controllers invoked based on `SQLCommand`, `MessageType` and `SQLTable`.
- Handshake protocol for client validation.
- Shutdown protocol for graceful connection teardown.

## Threading 
**Purpose:** Enable asynchronous processing of incoming requests without overloading the database.

**Design Highlights:**
- Task pool for pending requests from C++ and Python ML clients.
- Producer-consumer model:
  - Producers push incoming tasks into the queue.
  - Consumers pop tasks and execute corresponding controller functions.
- Worker threads spawned after handshake, joined after shutdown.
- Supports concurrency while maintaining safety and order.

## Main 
**Purpose:** Initialize and coordinate all components.

**Startup Sequence:**
1. Load configuration and environment variables.
2. Initialize database singleton and connection pool.
3. Start TCP listeners for C++ and Python ML clients.
4. Initialize task queues.
5. Spawn worker threads for each task queue.
6. Accept handshake from clients.
7. Block until shutdown signal received.
8. Gracefully join threads and clean up resources.

## Testing and Validation
**Purpose:** Ensure system correctness, robustness, and integration across languages.

**Unit Tests:**
- Repository CRUD operations.
- Validation logic for accounts and trading data.
- Serialization/deserialization round-trip.

**Integration Tests:**
- C++ → Rust → DB → Rust → C++ message round-trip.
- Python ML → Rust → DB → Rust → Python ML.
- Handshake and shutdown protocols.

**Stress & Fault Tests:**
- Simulate concurrent requests from C++ and Python ML.
- Inject malformed or incomplete packets to test robustness.
- Test database transaction rollback under failure scenarios.

## Notes / Future Improvements
- Add structured logging per thread and per client for easier debugging.
- Evaluate queue backpressure and maximum task limits to avoid overload.
- Optional replication or sharding for partition tolerance if scaling is needed.
- Protocol versioning and migration strategy for future updates.
- Documentation for binary message layouts and table schemas.

