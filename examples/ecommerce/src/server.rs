mod ecommerce;
use ecommerce::product_info_server::{ProductInfo, ProductInfoServer};
use ecommerce::{Product, ProductId};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
struct MyProduct {}

#[tonic::async_trait]
impl ProductInfo for MyProduct {
    async fn get_product(&self, _request: Request<ProductId>) -> Result<Response<Product>, Status> {
        Ok(Response::new(Product {
            id: 11.to_string(),
            name: "bow".to_string(),
            description: "bow!".to_string(),
        }))
    }
    async fn add_product(&self, _request: Request<Product>) -> Result<Response<ProductId>, Status> {
        Ok(Response::new(ProductId {
            value: 23.to_string(),
        }))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let product = MyProduct::default();
    let addr = "[::1]:5050".parse()?;
    Server::builder()
        .add_service(ProductInfoServer::new(product))
        .serve(addr)
        .await?;
    Ok(())
}
