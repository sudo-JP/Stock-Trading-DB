use crate::models::account::Account;
use crate::repositories::prelude_repo::*;

pub struct AccountRepository {
    pool: PgPool
}

impl AccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /*
     * This should fetch one, since we only update and insert 
     * */
    pub async fn query_by_id(&self, id: &str) -> Result<Account, sqlx::Error> {
        let result = sqlx::query_as::<sqlx::Postgres, Account>("
            SELECT * FROM accounts WHERE account_id = $1
            ")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(result)
    }

    pub async fn upsert(&self, account: &Account) -> Result<bool, sqlx::Error> {
        sqlx::query("INSERT INTO account
            (account_id, currency, buying_power, cash, portfolio_value, equity,
             unrealized_pl, realized_pl, status, last_update) VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ON CONFLICT (account_id) DO UPDATE SET
            currency = EXCLUDED.currency,
            buying_power = EXCLUDED.buying_power, 
            cash = EXCLUDED.cash,
            portfolio_value = EXCLUDED.portfolio_value,
            equity = EXCLUDED.equity,
            unrealized_pl = EXCLUDED.unrealized_pl, 
            realized_pl = EXCLUDED.realized_pl, 
            status = EXCLUDED.status, 
            last_update = EXCLUDED.last_update;") 
            .bind(&account.account_id)
            .bind(&account.currency)
            .bind(account.buying_power)
            .bind(account.cash)
            .bind(account.portfolio_value)
            .bind(account.equity)
            .bind(account.unrealized_pl)
            .bind(account.realized_pl)
            .bind(&account.status)
            .bind(account.last_update)
            .execute(&self.pool)
            .await?;

        Ok(true)
    }

    pub async fn delete_by_id(&self, account_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM account WHERE account_id = $1;")
            .bind(account_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

