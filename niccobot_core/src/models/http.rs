use reqwest::Client as HttpClient;

pub struct HttpKey;

impl serenity::prelude::TypeMapKey for HttpKey {
    type Value = HttpClient;


}