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
    
    if let Ok(meme) = client.images.random_meme() { //try to get a random meme handling the possible error
        //Do some logical stuff here...
    } else {
        //Error handling stuff
    }
}
```

Also there is an extra error management tool, it is ApiResponse, its behaviour is almost the same as Option

```rust
pub enum ApiResponse<S> {
    Success(S),
    Failed(RawError) // RawError is an universal api error, from this you can get the specific error using RawError.specific()
}
```

#### ApiResponse example
```rust
use ksoft::{Client, ApiResponse};

#[tokio::main]
async fn main() {
    let client = Client::new("TOKEN HERE"); //crate the client
    
    if let Ok(image) = client.images.get_image() { //image var will be ApiResponse<Image>
        match image {
            ApiResponse::Success(image) => {
                //Do something with the image
            },
            ApiResponse::Failed(why) => {
                //Or use why as RawError or get the specific error using why.specific()
                let specific_error = why.specific(); //why (RawError) gets consumed by RawError::specific() method
                
                //Do some handling stuff
            }
        }
    } else {
        //Error handling stuff
    }
}
```

Todo: finish the usage example and make docs on gitbook