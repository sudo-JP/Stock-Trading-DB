use chrono::prelude::{DateTime, Utc}; 
use std::collections::HashMap;

#[derive(Debug, sqlx::FromRow)]
pub struct FeatureSets {
    pub instrument_id: i32, 
    pub feat_id: i32, 
    pub time: DateTime<Utc>, 
    pub feature_vector: HashMap<String, String>,
    pub feature_version: String 
}
