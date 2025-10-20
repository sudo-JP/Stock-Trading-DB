use crate::models::account::Account;
use crate::repositories::prelude_repo::*;

struct AccountRepository {
    pool: PgPool
}

impl AccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, account: &Account) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("INSERT INTO account WHERE 
            (account_id, currency, buying_power, cash, portfolio_value, equity,
             unrealized_pl, realized_pl, status, last_update) VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);") 
            .bind(&account.account_id)
            .bind(&account.currency)
            .bind(account.buying_power)
            .bind(account.cash)
            .bind(account.portfolio_value)
            .bind(account.equity)
            .bind(account.unrealized_pl)
            .bind(account.realized_pl)
            .bind(&account.status)
            .bind(account.last_updated)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_account_by_id(&self, account_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM account WHERE account_id = $1;")
            .bind(account_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

