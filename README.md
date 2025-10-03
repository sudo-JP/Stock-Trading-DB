# Stock Trading DB 

This project is connected to the other two repositories. Different repos for better encapsulation, since these projects will be run on separate Proxmox containers: 
[Trade Frontend](https://github.com/sudo-JP/Stock-Trading-Trade)
[ML](https://github.com/sudo-JP/Stock-Trading-ML)

Current architecture. Project is still in progress, so lots of empty files for now.
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
