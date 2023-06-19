use serde_json;

use std::path::Path;
use std::fs::{File};


#[derive(Serialize, Deserialize)]
pub struct Config
{
    pub consumer_key : String,
    pub consumer_secret_key : String,

    pub access_key : String,
    pub access_secret_key : String,

    pub currencies_wishlist : Vec<String>,

    pub parsing_interval : u64
}

impl Config 
{
    pub fn read(path_file: &Path) -> Option<Config>
    {
        let mut file = match File::open(path_file)
        {
            Ok(f) => f,
            Err(_) => return None,
        };

        serde_json::from_reader(file).ok()
    }
}
