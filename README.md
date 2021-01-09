# KSoft.rs

A [KSoft](https://api.ksoft.si/) api wrapper written in pure Rust

## Usage for *asynchronous* client

### Cargo.toml
```toml
[dependencies.ksoft]
version = "1.0.6"

[dependencies.tokio]
version = "1.0"
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
use ksoft::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("TOKEN HERE"); //create the client
    
    if let Ok(image) = client.images.get_image("image id here").await { //image var will be ApiResponse<Image, ImageError>
        match image {
            Ok(image) => {
                //Do something with the image
            },
            Err(why) => { //In this case, why will be an ImageError struct
                //Do some handling stuff
            }
        }
    } else {
        //Error handling stuff
    }
}
```

## Usage for *blocking* client
*This is an optional feature for those people that want a blocking client for non-asynchronous contexts*

Both features ***cannot*** be enabled at the same time

### Cargo.toml
```toml
[dependencies.ksoft]
version = "1.0.6"
default-features=false
features = ["blocking"]
```

### Usage example
```rust
use ksoft::blocking::Client

fn main() {
    let client = Client::new("TOKEN HERE"); //create the client
    
    if let Ok(meme) = client.images.random_meme() { //try to get a random meme handling the possible error
        //Do some logical stuff here...
    } else {
        //Error handling stuff
    }
}
```