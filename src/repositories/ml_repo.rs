use crate::models::{MLModel, ModelPrediction}; 


pub struct MLRepository {
    pool: PgPool
}
impl MLRepository {
    async fn create_model(&self, ml: &Model) {
        let model = sqlx::query_as!(
            MLModel, 
            r#"INSERT INTO (model_name, model_version, hyperparameters, trained_at) VALUES ($1, $2, $3, $4) RETURNING *;"#,
            ml.model_name,
            ml.model_version, 
            ml.hyperparameters,
            ml.trained_at
            )
            .fetch_one(&self.pool)
            .await?;
    }
}
