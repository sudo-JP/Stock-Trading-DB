#[macro_export]
macro_rules! sql_str {
    (INSERT, $count:expr, $table:literal, $( $attr:ident ),*) => {
        {
            let mut query = String::from("INSERT INTO "); 
            query.push_str($table); 
            query.push_str(" (");
            $(
                query.push_str(stringify!($attr))
                query.push_str(", ");
            )*
            query.truncate(query.len() - 2); // removes trailing ", "

            query.push_str(") VALUES (");
            let n: usize = $count as usize;
            for i in 1..=n {
                query.push_str(&format!("${}", i));
                if i != n {
                    query.push_str(", "); 
                }
            }
            query.push_str(");");
            query 

        }
    };
    (UPSERT, $count:expr, $conflict:literal, $table:literal, $( $attr:ident ),*) => {
        {
            let mut query = String::from("INSERT INTO "); 
            query.push_str($table);
            query.push_str(" (");

            $(
                query.push_str(stringify!($attr))
                query.push_str(", ");
            )*
            query.truncate(query.len() - 2); // removes trailing ", "

            query.push_str(") VALUES (");
            let n: usize = $count as usize;
            for i in 1..=n {
                query.push_str(&format!("${}", i));
                if i != n {
                    query.push_str(", "); 
                }
            }

            query.push_str(") ON CONFLICT (");
            query.push_str($conflict); 
            query.push_str(") DO UPDATE SET ");

            $(
                let setting = format!("{} = EXCLUDED.{}", stringify!($attr), stringify!($attr));
                query.push_str(&setting);
                query.push_str(", ");
            )*

            query.truncate(query.len() - 2); // removes trailing ", "
            query.push(';');
            query 
        }
    };
    (DELETE) => {
    };
    (SELECT) => {
    };
    ( $($other:tt)* ) => {
        eprint!("Invalid macro usage");
    }; 
}

#[macro_export]
macro_rules! sql_repo {
    (query, $query:expr, $structure:expr, $pool:expr, $( $attr:ident ),*) => {
        {
            sqlx::query($query)
            $(
                .bind(&$structure.$attr)
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
    ( $($other:tt)* ) => {
        eprint!("Invalid macro usage");
    }; 
}
