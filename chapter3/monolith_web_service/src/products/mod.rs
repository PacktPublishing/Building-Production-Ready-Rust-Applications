use serde_derive::{Deserialize, Serialize};

// pub enum _ProductType {
//     mp3,
//     book,
//     mp4,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    product_name: String,
    media_type: Option<String>,
    gtin: String,
}

fn _get_product(_product_name: String) {}

fn _new_product(_product: Product) {}
