table! {
    products (id) {
        id -> Int4,
        title -> Varchar,
        stock -> Float8,
        rating -> Nullable<Float8>,
        price -> Nullable<Int4>,
        description -> Nullable<Varchar>,
        text_searchable_product_col -> Tsvector,
    }
}
