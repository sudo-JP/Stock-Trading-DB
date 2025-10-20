#[derive(Debug, sqlx::FromRow)]
pub struct LabeledData {
    pub instrument_id: i32, 
    pub label_id: i32, 
    pub future_return_5min: f32, 
    pub future_return_1H: f32, 
    pub regime_label: String 
}
