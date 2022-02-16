table! {
    customers (id) {
        id -> Int4,
        customer_name -> Varchar,
        customer_address -> Text,
    }
}

table! {
    orders (id) {
        id -> Int4,
        product_name -> Varchar,
        product_id -> Int4,
        customer_id -> Int4,
        amount -> Int4,
        address -> Text,
        fulfilled -> Bool,
    }
}

table! {
    products (id) {
        id -> Int4,
        product_name -> Varchar,
        product_type -> Varchar,
        amount -> Int4,
    }
}

table! {
    stocks (id) {
        id -> Int4,
        product_name -> Varchar,
        product_id -> Int4,
        amount -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    customers,
    orders,
    products,
    stocks,
);
