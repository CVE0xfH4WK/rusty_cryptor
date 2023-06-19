use futures::{Future, Stream};

use serde_json;

use hyper::Client;
use hyper_tls::HttpsConnector;

use tokio_core::reactor::Core;

use std;
use std::str;
use std::string::String;



#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CurrencyPrices
{
    pub GBP: f64,
    pub EUR: f64,
    pub USD: f64,
    pub CNY: f64,
    pub UAH: f64,
    pub JPY: f64,
}

pub struct Parser
{

}

impl Parser
{
    pub fn new() -> Parser
    {
        let parser = Parser { };
        
        return parser;
    }


    fn get_content(&self, content : String) -> CurrencyPrices
    {
        let slice: &str = &*content;
        let data: CurrencyPrices = serde_json::from_str(slice).unwrap();

        return data;
    }


    fn get_http_response(&self, url : &str) -> String
    {
        let mut core = Core::new().unwrap();
        let client = Client::configure()
                .connector(HttpsConnector::new(4, &core.handle()).unwrap())
                    .build(&core.handle());

        let uri = url.parse().unwrap();
        
        let get_resp = client.get(uri).and_then (|resp| {resp.body().concat2()})
            .map(|chunk| std::str::from_utf8(&chunk)
                .expect("error handling").to_string());
        let response = core.run(get_resp).unwrap();
        
        return response;
    }


    pub fn get_price(&self, currency : &str) -> CurrencyPrices
    {
        let url = format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=GBP,EUR,USD,CNY,UAH,JPY", currency);

        let content = self.get_http_response(&url[..]);
        let values: CurrencyPrices = self.get_content(content);

        return values;
    }
}
