table! {
    account (id) {
        balance -> Text,
        user_id -> Text,
        id -> Int4,
        name -> Text,
    }
}

table! {
    use super::sql_types::CurrencyType;
    use diesel::sql_types::*;
    transaction (id) {
        kind -> Bool,
        title -> Text,
        value -> Text,
        currency -> CurrencyType,
        time -> Date,
        user_id -> Text,
        id -> Int4,
        bank_account -> Int4,
    }
}

table! {
    users (username) {
        name -> Text,
        username -> Text,
        password -> Text,
        api_token -> Text,
        role -> Bool,
    }
}

joinable!(account -> users (user_id));
joinable!(transaction -> account (bank_account));
joinable!(transaction -> users (user_id));

allow_tables_to_appear_in_same_query!(account, transaction, users,);
pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "currency_type"))]
    pub struct CurrencyType;
}
