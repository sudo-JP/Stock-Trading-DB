use crate::models::Account;
use anyhow::{Result, anyhow};
use crate::repositories::AccountRepository;
use crate::controllers::CppController;
use crate::protocols::{CppBinaryMessage, SQLCommand};

pub enum AccountResult<T> {
    SUCCESS, 
    FAILURE, 
    VALUE(T), 
}

struct AccountController {
    repo: AccountRepository
}

fn validate_account(mut account: Account) -> Result<Account> {
    if account.cash < 0.0 {
        return Err(anyhow!("Negative balance"));
    }
    account.currency = account.currency.to_uppercase();

    Ok(account)
}

impl AccountController {
    pub fn new(repo: AccountRepository) -> Self {
        Self { repo }
    }
}

impl CppController<Account, AccountResult<Account>> for AccountController {

    async fn handle_operation(&self, bn: CppBinaryMessage, model: Account) -> Result<AccountResult<Account>, sqlx::Error> {
        let acc = match validate_account(model) {
            Ok(a) => a, 
            Err(e) => { 
                eprint!("Unable to process Account due to bad field: {}", e);
                return Err(sqlx::Error::Protocol("Can't valid Account".into())); 
            }
        };
        
        match bn.sql_command {
            SQLCommand::INSERT | SQLCommand::UPDATE => {
                //AccountRepository::insert(AccountRepository)
                self.repo.upsert(&acc).await?;
            } 
            SQLCommand::DELETE => {
                self.repo.delete_by_id(&acc.account_id).await?;
            }
            SQLCommand::SELECT => {
                return Ok(AccountResult::VALUE(self.repo.query_by_id(&acc.account_id).await?));
            }
                _ => { return Err(sqlx::Error::Protocol("Unknown SQL command".into())); }
        }; 

        Ok(AccountResult::SUCCESS)
    }
}


