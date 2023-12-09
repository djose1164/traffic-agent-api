// @generated automatically by Diesel CLI.

diesel::table! {
    traffic_tickets (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 256]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        last_name -> Nullable<Varchar>,
        #[max_length = 128]
        email -> Nullable<Varchar>,
        #[max_length = 128]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    vehicles (id) {
        #[max_length = 20]
        id -> Varchar,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 50]
        owner -> Varchar,
        #[max_length = 30]
        motor_type -> Varchar,
        year -> Integer,
        #[max_length = 20]
        color -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    traffic_tickets,
    users,
    vehicles,
);
