use crate::utils;
use reqwest::header::HeaderMap;
use utils::request::Request;
use serde_json::{Result, Value};

pub struct NewsCctvCom {
    pub url: String,
    pub headers: HeaderMap,
}

impl NewsCctvCom {
    pub fn new() -> NewsCctvCom {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        NewsCctvCom {
            url: "https://news.cctv.com/2019/07/gaiban/cmsdatainterface/page/news_1.jsonp?cb=news"
                .to_string(),
            headers,
        }
    }
    async fn parse(&mut self) {
        let mut request = Request::new();
        request.headers = self.headers.clone();
        let rep = match request.get(self.url.clone()).send().await {
            Ok(v) => match v.text().await {
                Ok(t) => {
                    let slice = &t[5..];
                    let v: Value = serde_json::from_str(&slice[..slice.len() - 1]).expect("");
                    v
                }
                Err(e) => {
                    println!("{}", e);
                    panic!()
                }
            },
            Err(e) => {
                println!("{}", e);
                panic!()
            }
        };
        println!("{:#?}", rep);
    }
    pub async fn start(&mut self) {
        self.parse().await;
    }
}
