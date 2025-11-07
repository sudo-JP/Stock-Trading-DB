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
        let result = crate::sql_repo!(query_as, 
            "SELECT * FROM accounts WHERE account_id = $1".into(),
            Account, id).fetch_one(&self.pool).await?; 

        Ok(result)
    }


    pub async fn upsert(&self, account: &Account) -> Result<bool, sqlx::Error> {
        let query_str: String = crate::sql_str!(UPSERT, 19, "account", 
            id, currency, cash, buying_power, equity, portfolio_value,
            effective_buying_power, daytrading_buying_power, regt_buying_power,
            non_marginable_buying_power, last_equity, sma, position_market_value,
            long_market_value, short_market_value, status, crypto_status,
            balance_asof, daytrade_count
        );

        crate::sql_repo!(query, &query_str, &account, 
            id, currency, cash, buying_power, equity, portfolio_value,
            effective_buying_power, daytrading_buying_power, regt_buying_power,
            non_marginable_buying_power, last_equity, sma, position_market_value,
            long_market_value, short_market_value, status, crypto_status,
            balance_asof, daytrade_count
        ).execute(&self.pool).await?;

        Ok(true)
    }

    pub async fn delete_by_id(&self, account_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("")
            .bind(account_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

