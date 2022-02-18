use crate::data::get_connection;
use crate::models::{NewProduct, Product};
use crate::schema;
use crate::schema::products::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_product<'a>(product: &'a Product) -> Product {
    use schema::products;

    let conn = get_connection();

    let new_product = NewProduct {
        product_name: product.product_name.clone(),
        product_type: product.product_type.clone(),
        amount: product.amount,
    };

    let ret = diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(&conn)
        .expect("Error saving new post");

    println!("Ret: {:?}", ret);

    ret
}

pub fn update_product<'a>(con: &PgConnection, product: &'a Product) {
    let product = diesel::update(products)
        .set(product)
        .get_result::<Product>(con)
        .expect(&format!("Unable to find post {}", product.id)); //.get_result();

    println!("Published post {}", product.id);
}

pub fn delete_product<'a>(con: &PgConnection, product: &'a Product) {
    let num_deleted = diesel::delete(products.find(product.id))
        .execute(con)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}

pub fn show_products() -> Vec<Product> {
    let connection = get_connection();
    let results = products
        .limit(10)
        .load::<Product>(&connection)
        .expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.product_name);
    //     println!("----------\n");
    //     println!("{}", post.address);
    // }

    results
}
