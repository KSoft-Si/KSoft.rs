# KSoft.rs

A [KSoft](https://api.ksoft.si/) api wrapper written in pure Rust

## Usage

### Cargo.toml
```toml
[dependencies.ksoft]
git = "https://github.com/AlvaroMS25/KSoft.rs"
branch = "master"

[dependencies.tokio]
version = "0.2"
features = ["macros"]
```

### Usage example
```rust
use ksoft::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("TOKEN HERE"); //crate the client
    
    if let Ok(meme) = client.images.random_meme().await { //try to get a random meme handling the possible error
        //Do some logical stuff here...
    } else {
        //Error handling stuff
    }
}
```

Also there is an extra error management tool, it is ApiResponse, its behaviour is almost the same as Result,
is only there to difference between an http error and an API error or unsuccessful response

```rust
pub enum ApiResponse<S, E> {
    Success(S),
    Failed(E)
}
```

#### ApiResponse example
```rust
use ksoft::{Client, ApiResponse};

#[tokio::main]
async fn main() {
    let client = Client::new("TOKEN HERE"); //crate the client
    
    if let Ok(image) = client.images.get_image("image id here").await { //image var will be ApiResponse<Image, Error404>
        match image {
            ApiResponse::Success(image) => {
                //Do something with the image
            },
            ApiResponse::Failed(why) => {
                //Do some handling stuff
            }
        }
    } else {
        //Error handling stuff
    }
}
```

Todo: finish the usage example and make docs on gitbook