use crate::models::{MLModel, ModelPrediction}; 


pub struct MLRepository {
    pool: PgPool
}

impl MLRepository {
    async fn create_model(&self, ml: &Model) -> Result<MLModel, Error> {
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

        Ok(model)
    }

    async fn get_model_by_name_version(&self, model_name: &String, model_version: &String) -> Result<MLModel, Error> {
        let model = sqlx::query_as!(
            MLModel,
            "SELECT * FROM models WHERE model_name = $1 AND model_version = $2;", 
            model_name, 
            model_version
            )
            .fetch_one(&self.pool)
            .await?;
        Ok(model)
    }

    async fn get_all_models_by_name(&self, model_name: &String) -> Result<Vec<MLModel>, Error>  {
        let models = sqlx::query_as!(
            "SELECT * FROM models WHERE model_name = $1", 
            model_name
            )
            .fetch_all(&self.pool)
            .await?;
        Ok(models)
    }
}
