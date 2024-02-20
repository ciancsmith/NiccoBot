use reqwest::Client as HttpClient;
use crate::client::{Context, Data, Error};


pub struct HttpKey;

impl serenity::prelude::TypeMapKey for HttpKey {
    type Value = HttpClient;


}