use futures;
use directories;
use passionfruit;

fn main() {
    let download = futures::executor::block_on(
        passionfruit::Download::new("https://imgur.com/t/meme/8TfWvYh").start(),
    );

    match download {
        Ok(result) => {
            if let Some(dirs) = directories::UserDirs::new() {
                result.write_to( std::string::String::from(dirs.document_dir().unwrap().to_str().unwrap()), "out".to_string()).unwrap();
            }
        },

        Err(why) => panic!("An error occured: {}", why)
    }
}
