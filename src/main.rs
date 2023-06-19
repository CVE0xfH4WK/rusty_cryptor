extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate twitter_api;
extern crate oauth_client as oauth;

use std::time;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

#[macro_use]
mod config;
mod parser;
mod twitter;

const CONF_FILENAME: &'static str = ".crypto-bot.conf";


fn get_home_dir() -> PathBuf {
    match env::home_dir() {
        Some(path) => path,
        None => {
            panic!("Impossible to get your home dir!");
        }
    }
}


pub fn build_message(prices : &mut HashMap<&str, parser::CurrencyPrices>) -> String
{
    let mut string = String::from("");

    for (currency, value) in prices.iter()
    {
        let text = format!("#{} : £{}, €{}, ${}, 元{}, ₴{}, ¥{};\n", 
            currency, value.GBP, value.EUR,
            value.USD, value.CNY, value.UAH, value.JPY);
    
        string = string + &text[..];
    }
}


fn main() 
{
    let mut conf_file: PathBuf = get_home_dir();
    conf_file.push(Path::new(CONF_FILENAME));


    let config = match config::Config::read(&conf_file)
    {
        Some(v) => v,
        None => panic!("Can't find config file!"),
    };

    let parser = parser::Parser::new();
    let twitter = twitter::Twitter::new(
        config.access_key, config.access_secret_key,
        config.consumer_key, config.consumer_secret_key);

    let mut prices = HashMap::new();

    loop
    {
        for currency in &config.currencies_wishlist
        {
            let currency_ = &currency[..];
            
            let price = parser.get_price(currency_);
            prices.insert(currency_, price);
        }

        // let msg = build_message(&mut prices);
        twitter.tweet(message);

        std::thread::sleep(time::Duration::from_micros(1000 * config.parsing_interval));
    }
}