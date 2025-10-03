# Parallel Stock Trading DB 

Current architecture 
```bash
.
├── Cargo.lock
├── Cargo.toml
├── compose.yaml
├── Dockerfile
├── README.md
└── src
    ├── controllers
    │   └── stockController.rs
    ├── database
    │   ├── connection.rs
    │   └── mod.rs
    ├── main.rs
    ├── models
    │   ├── feature_sets.rs
    │   ├── instruments.rs
    │   ├── labeled_data.rs
    │   ├── market_data_ticks.rs
    │   ├── ml_models.rs
    │   ├── mod.rs
    │   ├── positions.rs
    │   ├── schema.sql
    │   ├── technical_indicators.rs
    │   └── trades.rs
    ├── repositories
    │   ├── instrument_repo.rs
    │   └── mod.rs
    ├── routes
    │   ├── mod.rs
    │   └── stockRoute.rs
    └── services
        └── stockService.rs
```
