table! {
    products (id) {
        id -> Int4,
        title -> Varchar,
        stock -> Float8,
        rating -> Nullable<Float8>,
        price -> Nullable<Int4>,
    }
}
