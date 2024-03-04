# Rust API for flecsd-axum-api

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

## Overview

This server was generated by the [openapi-generator]
(https://openapi-generator.tech) project. By using the
[OpenAPI-Spec](https://github.com/OAI/OpenAPI-Specification) from a remote
server, you can easily generate a server stub.

To see how to make this your own, look here: [README]((https://openapi-generator.tech))

- API version: 2.0.0
- Build date: 2024-03-04T11:09:54.345751658Z[Etc/UTC]



This autogenerated project defines an API crate `flecsd-axum-api` which contains:
* An `Api` trait defining the API in Rust.
* Data types representing the underlying data model.
* Axum router which accepts HTTP requests and invokes the appropriate `Api` method for each operation.
  * Request validations (path, query, body params) are included.

## Using the generated library

The generated library has a few optional features that can be activated through Cargo.

* `server`
    * This defaults to enabled and creates the basic skeleton of a server implementation based on Axum.
    * To create the server stack you'll need to provide an implementation of the API trait to provide the server function.
* `conversions`
    * This defaults to disabled and creates extra derives on models to allow "transmogrification" between objects of structurally similar types.

See https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section for how to use features in your `Cargo.toml`.

### Example

```rust
struct ServerImpl {
   // database: sea_orm::DbConn,
}

#[allow(unused_variables)]
#[async_trait]
impl flecsd-axum-api::Api for ServerImpl {
  // API implementation goes here
}

pub async fn start_server(addr: &str) {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Init Axum router
    let app = flecsd-axum-api::server::new(Arc::new(ServerImpl));

    // Add layers to the router
    let app = app.layer(...);

    // Run the server with graceful shutdown
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
```
