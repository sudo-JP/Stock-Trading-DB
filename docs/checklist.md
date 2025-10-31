# Developer Checklist 

## SQL Schema 
- [x] Alpaca API Account Table
- [ ] Alpaca API Order Table 
- [x] Generic Position table 
- [ ] Alpaca API Position Table 
- [ ] Alpaca API Asset Table `(instruments)`
- [x] Generic Market Data Ticks Table [ ] Actual market data tick Table
- [x] Generic Labeled Data Table 
- [ ] Actual Labeled Data Table 
- [x] Generic feature sets Table 
- [ ] Actual feature sets Table [x] Generic Model Prediction Table 
- [ ] Actual Model Prediction Table 
- [x] Generic Technical Indicators Table 
- [ ] Actual Technical Indicators Table 
- [x] Generic Model Table 
- [x] Generic trades Table 
- [ ] Actual trades Table 
- [ ] Actual Model Table 
- [ ] Check indexing for any appropriate table 

## Repository 
- [x] Account CRUD
- [x] Instrument (Asset) CRUD
- [ ] Market Data Tick CRUD (Missing Delete and Update)
- [ ] ML Model CRUD (Missing Delete and Update)
- [x] Order CRUD
- [x] Position CRUD 
- [ ] Trade CRUD (Missing Delete and Update)

## TCP/Networking 
- [x] Struct and enums matched with C++ 
- [x] Deserialize message 
- [x] Reading the bytes from C++ (LittleEndian)
- [x] Read string from bytes 
- [x] Read Utc Time from bytes
- [x] Convert read bytes to respective number (u32, u64, etc...)
- [x] Match struct value with enum for table, operation
- [x] Server connection with C++ 
- [ ] Queue for distribution workload 
- [ ] Error handling for bad bytes 
- [ ] Handshake Protocol 
- [ ] Shutdown Protocol 
- [ ] Python ML Serialze 
- [ ] Define Python ML Protocol Structs 
- [ ] Python ML TCP Client 
- [ ] Deserialize ML TCP for C++ 
- [ ] Reading the bytes from Python (LittleEndian)
- [ ] Define to C++ from Python Protocol 
- [ ] Setup client to C++

## Controllers 
- [x] Alpaca Order 
- [x] Alpaca Order Validation
- [x] Alpaca Asset 
- [x] Alpaca Asset Validation
- [x] Alpaca Account 
- [x] Alpaca Account Validation
- [x] Alpaca Position 
- [x] Alpaca Position Validation
- [ ] Trade 
- [ ] Trade Validation
- [ ] Market Data Ticks 
- [ ] Market Data Ticks Validation
- [ ] Trade 
- [ ] Trade Validation
- [ ] ML Model 
- [ ] ML Model Validation
- [ ] ML Prediction 
- [ ] ML Prediction Validation
- [ ] Labeled Data 
- [ ] Labeled Data Validation 
- [ ] Feature sets 
- [ ] Feature sets Validation
- [ ] Technical Indicators 
- [ ] Technical Indicators Validation


## Threading 
- [ ] Task pool 
- [ ] Producer method for pushing tasks C++
- [ ] Consumer method for executing controller functions for C++ 
- [ ] Producer method for pushing tasks Python ML 
- [ ] Consumer method for executing controller functions for Python Ml 
- [ ] Spawn threads workers after handshake 
- [ ] Join threads after shutdown protocol initialized 

## Database / Config
- [x] Singleton Database class shared across repositories 
- [x] Retrieve database credentials from `.env` 
- [ ] Connection pooling setup (idle timeout configured) 
- [ ] Retry strategy for transient DB errors 
- [x] Migrations loading 

## Test
- [x] Database connection worked
- [x] Insert order to database and query it back
- [ ] Testing CRUD on every other table
- [x] TCP connection with C++ frontend
- [x] TCP u32 prints out from C++
- [ ] TCP string from C++
- [ ] TCP Utc from C++
- [ ] Controller testing
- [ ] Handshake protocol
- [ ] Shutdown protocol
- [ ] Validation logic tests (e.g., Account negative cash, field normalization)
- [ ] Serialization/deserialization round-trip tests (Rust <-> C++ <-> Python)
- [ ] ML TCP integration tests (Python ML -> Rust -> DB)
- [ ] Stress tests for async queue / thread pool
- [ ] Error handling / malformed packet tests
- [ ] Database transaction rollback tests


## Main
- [ ] Establish handshake protocol
- [ ] Spawn threads based on handshake
- [ ] Receive shutdown protocol
- [ ] Join threads and gracefully shutdown the system
- [ ] Initialize database singleton and connection pool
- [ ] Start TCP listener(s) for C++ and Python ML clients
- [ ] Initialize task queues for C++ and Python ML
- [ ] Catch and handle runtime errors during startup
