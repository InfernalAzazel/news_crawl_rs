use reqwest::header::HeaderMap;
use reqwest::{Client, RequestBuilder};

pub struct Request {
    pub headers: HeaderMap,
    pub client: Client,
}

impl Request {
    pub fn new() -> Request {
        Request {
            headers: HeaderMap::new(),
            client: reqwest::Client::new(),
        }
    }
    pub fn post(&self, url: String) -> RequestBuilder {
        self.client.post(url).headers(self.headers.clone())
    }
    pub fn get(&self, url: String) -> RequestBuilder {
        self.client.get(url).headers(self.headers.clone())
    }
}
