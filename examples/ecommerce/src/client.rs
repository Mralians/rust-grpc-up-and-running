mod ecommerce;
use ecommerce::{product_info_client::ProductInfoClient, Product, ProductId};
use tonic::{Request, Response};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://[::1]:5050";
    let mut client = ProductInfoClient::connect(addr).await?;
    let response = client
        .get_product(Request::new(ProductId {
            value: 23.to_string(),
        }))
        .await?;
    println!("{:#?}", response);
    Ok(())
}
