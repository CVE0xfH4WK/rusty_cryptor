use twitter_api;
use oauth::Token;



pub struct Twitter<'x>
{
    access_token : Token<'x>,
    consumer_token : Token<'x>,
}


impl<'x> Twitter<'x>
{
    pub fn new(access_key : String, access_secret_key : String,
            consumer_key : String, consumer_secret_key : String) -> Self
    {
        Twitter
        {
            access_token : Token::new(access_key, access_secret_key),
            consumer_token : Token::new(consumer_key, consumer_secret_key)
        }
    }


    pub fn tweet(&self, message : String)
    {
        twitter_api::update_status(&self.consumer_token, &self.access_token, &message, &None).unwrap();
    }
}
