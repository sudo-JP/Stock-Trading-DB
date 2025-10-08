use std::collections::HashMap;
use chrono::prelude::{DateTime, Utc}; 

pub struct MLModel {
    model_id: i32, 
    model_name: String, 
    model_version: String, 
    hyperparameters: HashMap<String, String>,
    trained_at: DateTime<Utc> 
}

pub struct ModelPrediction {
    prediction_id: i32, 
    instrument_id: i32,
    time: DateTime<Utc>, 
    prediction_value: f32, 
    confidence: f32 
}
