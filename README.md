# passionfruit

A Rust library to fetch files off of the interwebs and optionally download them.
In short, a [`reqwest`](https://crates.io/crates/reqwest) interface.

## Example

Similarly to `reqwest`, this example uses the module [`tokio`](https://crates.io/crates/tokio) to make the `fn main()` asynchronous:

```rs
use tokio;
use directories;
use passionfruit;


#[tokio::main]
async fn main() {
    match passionfruit::Download::new("https://i.imgur.com/ISfpRae.jpeg").start().await {
        Ok(result) => {
            if let Ok(_) = result.write_to(
                directories::UserDirs::new()
                    .unwrap()
                    .desktop_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                    "lol".to_string()
                ) {
                println!("Download completed!")
            }
        }
        Err(why) => panic!("It appears something went wrong: {}", why)
    }
}
```

An example which doesn't use `tokio` but instead [`futures`](https://crates.io/crates/futures):

```rs
use futures;
use directories;
use passionfruit;

fn main() {
    let download = futures::executor::block_on(
        passionfruit::Download::new("https://i.imgur.com/8iiChzd.jpeg").start(),
    );

    match download {
        Ok(result) => {
            if let Some(dirs) = directories::UserDirs::new() {
                result.write_to(
                    dirs.document_dir()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(), 
                    "out".to_string()
                ).unwrap();
            }
        },

        Err(why) => panic!("An error occured: {}", why)
    }
}
```
###### Stream Passionfruti by Rav and Kill Bill: The Rapper here: [https://open.spotify.com/track/6WhtwHTLgxYRNoW1OPDuQo?autoplay=true](https://open.spotify.com/track/6WhtwHTLgxYRNoW1OPDuQo?autoplay=true)
