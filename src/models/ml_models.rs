use std::collections::HashMap;
use chrono::prelude::{DateTime, Utc}; 

#[derive(Debug, sqlx::FromRow)]
pub struct MLModel {
    pub model_id: i32, 
    pub model_name: String, 
    pub model_version: String, 
    pub hyperparameters: HashMap<String, String>,
    pub trained_at: DateTime<Utc> 
}

#[derive(Debug, sqlx::FromRow)]
pub struct ModelPrediction {
    pub prediction_id: i32, 
    pub instrument_id: i32,
    pub time: DateTime<Utc>, 
    pub prediction_value: f32, 
    pub confidence: f32 
}
