#[macro_export]
macro_rules! sql_repo {
    (query, $query:expr, $structure:expr, $pool:expr, $( $attr:ident ),*) => {
        {
            sqlx::query($query)
            $(
                .bind($structure.$attr)
            )*
            .execute($pool)
            .await?
        }
    }; 

    (query_as, $query:expr, $structure:expr, $struct_type:ty, $( $attr:ident ),*) => {
        {
            sqlx::query_as::<sqlx::Postgres, $struct_type>($query)
            $(
                .bind($structure.$attr)
            )*
        }
    };
    ( _* ) => {

        return Err(sqlx::Error::Protocol("Invalid Macro usage".into())); 
    }; 
}
