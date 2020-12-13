# KSoft.rs

A [KSoft](https://api.ksoft.si/) api wrapper written in pure Rust

## Usage

### Cargo.toml
```toml
[dependencies.ksoft]
version = "1.0.4"

[dependencies.tokio]
version = "0.2"
features = ["macros"]
```

### Usage example
```rust
use ksoft::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("TOKEN HERE"); //create the client
    
    if let Ok(meme) = client.images.random_meme().await { //try to get a random meme handling the possible error
        //Do some logical stuff here...
    } else {
        //Error handling stuff
    }
}
```

Also there is an extra error management tool, it is ApiResponse, its behaviour is the same as Result,
as it is a renaming of it used to difference between an http error and an API error or unsuccessful response

```rust
pub type ApiResponse<S, E> = Result<S, E>;
```

#### ApiResponse example
```rust
use ksoft::{Client, ApiResponse};

#[tokio::main]
async fn main() {
    let client = Client::new("TOKEN HERE"); //create the client
    
    if let Ok(image) = client.images.get_image("image id here").await { //image var will be ApiResponse<Image, ImageError>
        match image {
            Ok(image) => {
                //Do something with the image
            },
            Ok(why) => { //In this case, why will be an ImageError struct
                //Do some handling stuff
            }
        }
    } else {
        //Error handling stuff
    }
}
```