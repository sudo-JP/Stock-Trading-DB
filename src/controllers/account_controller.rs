use crate::models::Account;
use crate::repositories::account_repo::AccountRepository;
use crate::controllers::CppController;
use crate::protocols::{BinaryMessage, SQLCommand};

struct AccountController;

impl CppController<Account> for AccountController {
    fn handle_operation(&self, bn: BinaryMessage, model: Account) {
       match bn.sql_command {
           SQLCommand::INSERT => {} 
           SQLCommand::UPDATE => {}
           SQLCommand::DELETE => {}
           SQLCommand::SELECT => {}
            _ => {}
       } 
    }
}


/*pub fn insert_account_to_db(acc: Account) -> Result<bool, sqlx::Error> {
    //AccountRepository::create
}*/
