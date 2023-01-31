use std::collections::BTreeMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ProductMsg {
    category: String,
    id: isize,
    title: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ProductsMsg {
    products: Vec<ProductMsg>,
}

struct Product {
    id: isize,
    title: String,
}

impl Product {
    pub fn from_product_msg(msg: ProductMsg) -> Self {
        Self {
            id: msg.id,
            title: msg.title,
        }
    }
}

struct Products {
    products: BTreeMap<String, Vec<Product>>,
}

impl Products {
    pub fn from_products_msg(msg: ProductsMsg) -> Self {
        let mut products = BTreeMap::new();

        for product in msg.products {
            let category = products
                .entry(product.category.clone())
                .or_insert_with(Vec::new);
            category.push(Product::from_product_msg(product));
        }

        Self { products }
    }
}

fn main() {
    let body = reqwest::blocking::get("https://dummyjson.com/products")
        .unwrap()
        .json::<ProductsMsg>()
        .unwrap();

    let products = Products::from_products_msg(body);
    for (category_name, category_products) in products.products.iter() {
        println!("Category: {category_name}");
        for product in category_products.iter() {
            println!("{} - {}", product.id, product.title);
        }
    }
}
