<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/bump-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/bump-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/bump-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/bump-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/bump-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/bump-rs/test?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/bump-api">
    <img src="https://img.shields.io/crates/d/bump-api?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/bump-api">
    <img src="https://img.shields.io/crates/v/bump-api?style=flat-square" alt="Crates.io" />
</a>

</p>

[Bump](https://www.bump.sh) client, generated from the OpenAPI spec.

# Usage

```rust
use bump_api::BumpClient;
use bump_api::model::*;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let response = client
        .post_diffs()
        .url("your url")
        .previous_url("your previous url")
        .previous_definition("your previous definition")
        .previous_references(
            vec![
                Reference { location : Some("your location".to_owned()), content :
                Some("your content".to_owned()) }
            ],
        )
        .definition("your definition")
        .references(
            vec![
                Reference { location : Some("your location".to_owned()), content :
                Some("your content".to_owned()) }
            ],
        )
        .expires_at("your expires at")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

```

This example loads configuration from environment variables, specifically:

* `BUMP_BASE_URL`

* `BUMP_AUTHORIZATION_TOKEN`

* `BUMP_BASIC_TOKEN`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
bump-api = "0.1.0"
```


# Documentation

* [Client Library Documentation](https://docs.rs/bump-api)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*
