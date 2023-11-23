# Product Info Service Example

simple gRPC service for storing and retrieving product information. 

The service has two RPCs:

- `addProduct`: Adds a new product by taking in a `Product` message and returning a `ProductID`
- `getProduct`: Retrieves a product by taking in a `ProductID` and returning a `Product`

The `Product` message contains basic details like an ID, name, and description for each product.

## Running the Code

To implement and run the server:

```
- Compile productInfo.proto
- Implement the ProductInfo service 
- Run a gRPC server 
```

To implement and run the client:

```
- Compile productInfo.proto
- Import and use the generated client code
- Create a client channel 
- Call service methods like `add_product` and `get_product`
```

