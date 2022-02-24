table! {
    products (id) {
        id -> Int4,
        title -> Varchar,
        stock -> Float8,
        rating -> Nullable<Int4>,
        price -> Nullable<Int4>,
    }
}
