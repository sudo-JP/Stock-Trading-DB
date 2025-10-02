use chrono::prelude::{DateTime, Utc}; 
use std::collections::HashMap;

pub struct FeatureSets {
    instrument_id: i32, 
    feat_id: i32, 
    time: DateTime<Utc>, 
    feature_vector: HashMap<String, String>,
    feature_version: String 
}
