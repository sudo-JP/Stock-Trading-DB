use crate::macros::sql_macros;
use crate::models::account::Account;
use crate::repositories::prelude_repo::*;
use crate::sql_repo;

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
        let query_str = "INSERT INTO account
            (id, currency, cash, buying_power, equity, portfolio_value,
            effective_buying_power, daytrading_buying_power, regt_buying_power,
            non_marginable_buying_power, last_equity, sma, position_market_value,
            long_market_value, short_market_value, status, crypto_status,
            balance_asof, daytrade_count) VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19)
            ON CONFLICT (id) DO UPDATE SET
            currency = EXCLUDED.currency,
            cash = EXCLUDED.cash,
            buying_power = EXCLUDED.buying_power,
            equity = EXCLUDED.equity,
            portfolio_value = EXCLUDED.portfolio_value,
            effective_buying_power = EXCLUDED.effective_buying_power,
            daytrading_buying_power = EXCLUDED.daytrading_buying_power,
            regt_buying_power = EXCLUDED.regt_buying_power,
            non_marginable_buying_power = EXCLUDED.non_marginable_buying_power,
            last_equity = EXCLUDED.last_equity,
            sma = EXCLUDED.sma,
            position_market_value = EXCLUDED.position_market_value,
            long_market_value = EXCLUDED.long_market_value,
            short_market_value = EXCLUDED.short_market_value,
            status = EXCLUDED.status,
            crypto_status = EXCLUDED.crypto_status,
            balance_asof = EXCLUDED.balance_asof,
            daytrade_count = EXCLUDED.daytrade_count;"; 

        sql_repo!(query, &query_str, &account, &self.pool, 
        id, currency, cash, buying_power, equity, portfolio_value, 
        effective_buying_power, daytrading_buying_power, regt_buying_power,
        non_marginable_buying_power, last_equity, sma, position_market_value,
        long_market_value, short_market_value, status, crypto_status,
        balance_asof, daytrade_count); 

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

