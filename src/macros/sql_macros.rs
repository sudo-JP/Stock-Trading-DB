#[macro_export]
macro_rules! sql_col {
    ($table:literal, $( $attr:ident ),*) => {
        let mut query = String::from("INSERT INTO "); 
        query.push_str($table); 
        query.push_str(" (");
        $(
            query.push_str(stringify!($attr))
            query.push_str(", ");
        )*

        query.truncate(query.len() - 2); // removes trailing ", "
        query 
    };
}


fn sql_placeholders(n: usize) -> String {
    let mut placeholders = String::from(""); 
    for i in 1..=n {
        placeholders.push_str(&format!("${}", i));
        if i != n {
            placeholders.push_str(", "); 
        }
    }
    placeholders
}

/*
 * This is for tail recursion improvision
 * */
#[macro_export]
macro_rules! count_args_helper {
    ($acc:expr) => { $acc };
    ($acc:expr, $x:ident, $( $xs:ident ),* ) => { count_args_helper!($acc + 1, $($xs),*) }
}

#[macro_export]
macro_rules! count_args {
    ( $( $attr:ident ),* ) => { 
        count_args_helper!(0, $($attr),*)
    }
}


#[macro_export]
macro_rules! sql_str {
    (INSERT, $table:literal, $structure:expr, $( $attr:ident ),*) => {
        {
            let mut query = sql_col!($table, $($attr),*);

            query.push_str(") VALUES (");
            let n: usize = count_args!($($attr),*);
            query.push_str(sql_placeholders(n)); 
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
    ( $($other:tt)* ) => {
        {
            let e = String::from("Invalid macro usage");
            eprint!("{}", e);
            e
        }
    }; 
}

#[macro_export]
macro_rules! sql_repo {


    (query, $query:expr, $structure:expr, $( $attr:ident ),*) => {
        {
            let q = sqlx::query($query)
            $(
                .bind(&$structure.$attr)
            )*;
            q
        }
    }; 

    (query_as, $query:expr, $struct_type:ty, $( $attr:ident ),*) => {
        {

            let q  = sqlx::query_as::<sqlx::Postgres, $struct_type>($query)
            $(
                .bind($attr)
            )*; 
            q
        }
    };
    ( $($other:tt)* ) => {
        compile_error!("Invalid macro usage");
    }; 
}
