
pub fn sql_placeholders(n: usize) -> String {
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
 * This is for tail recursion optimization
 * */
#[macro_export]
macro_rules! count_args_helper {
    ($acc:expr) => { $acc };
    ($acc:expr, $x:ident) => { $acc + 1 };
    ($acc:expr, $x:ident, $( $xs:ident ),+ ) => 
    { crate::count_args_helper!($acc + 1, $($xs),+) }
}

#[macro_export]
macro_rules! count_args {
    ( $( $attr:ident ),+ ) => { 
        crate::count_args_helper!(0, $($attr),+)
    }
}

#[macro_export]
macro_rules! sql_col {
    ($table:literal, $( $attr:ident ),+) => {
        {
            let mut sql_stmt = String::from("INSERT INTO "); 
            sql_stmt.push_str($table); 
            sql_stmt.push_str(" (");
            $(
                sql_stmt.push_str(stringify!($attr));
                sql_stmt.push_str(", ");
            )+

            sql_stmt.truncate(sql_stmt.len() - 2); // removes trailing ", "
            sql_stmt.push_str(") VALUES (");
            let n: usize = crate::count_args!($($attr),+);
            sql_stmt.push_str(&crate::macros::sql_placeholders(n)); 
            sql_stmt 

        }
    };
}



#[macro_export]
macro_rules! sql_insert {
    (INSERT, $table:literal, $structure:expr, $( $attr:ident ),+) => {
        {
            sqlx::query({
                let mut sql_stmt = sql_col!($table, $($attr),+);
                sql_stmt.push_str(");");
                sql_stmt
            }.as_str())
            $(
                .bind(&$structure.$attr)
            )+
            
        }
    };
    (UPSERT, $table:literal, $structure:expr, $conflict:literal, $( $attr:ident ),+) => {
        {
            {
                sqlx::query({
                    let mut sql_stmt = crate::sql_col!($table, $($attr),+);

                    sql_stmt.push_str(") ON CONFLICT (");
                    sql_stmt.push_str($conflict); 
                    sql_stmt.push_str(") DO UPDATE SET ");

                    $(
                        let setting = format!("{} = EXCLUDED.{}", stringify!($attr), stringify!($attr));
                        sql_stmt.push_str(&setting);
                        sql_stmt.push_str(", ");
                    )+

                    sql_stmt.truncate(sql_stmt.len() - 2); // removes trailing ", "
                    sql_stmt.push(';');
                    sql_stmt
                }.as_str())
                $(
                    .bind(&$structure.$attr)
                )+
            }
        }
    };
}

