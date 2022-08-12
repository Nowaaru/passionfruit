use tokio;
use directories;
use passionfruit;


#[tokio::main]
async fn main() {
    match passionfruit::Download::new("https://i.imgur.com/ISfpRae.jpeg").start().await {
        Ok(result) => {
            if let Ok(_) = result.write_to(directories::UserDirs::new().unwrap().desktop_dir().unwrap().to_str().unwrap().to_string(), "lol".to_string()) {
                println!("Download completed!")
            }
        }
        Err(why) => panic!("It appears something went wrong: {}", why)
    }
}
