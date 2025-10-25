use crate::models::Position;
use anyhow::Result;
use crate::repositories::PositionRepository;
use crate::controllers::CppController;
use crate::protocols::{CppBinaryMessage, SQLCommand};

struct PositonController {
    repo: PositionRepository
}
