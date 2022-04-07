use std::str::FromStr;

use crate::utils;
use reqwest::header::HeaderMap;
use utils::request::Request;
use serde_json::Value;

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
    async fn parse(&mut self) -> anyhow::Result<Value> {
        let mut request = Request::new();
        request.headers = self.headers.clone();
        Ok(Value::from_str(
            request
                .get(&self.url)
                .send()
                .await?
                .text()
                .await?
                // 从第5个字符开始
                .get(5..)
                // 截取到倒数第一个字符
                .and_then(|s| s.get(..s.len() - 1))
                .unwrap_or(""),
        )?)
    }
    
    pub async fn start(&mut self) {
        match self.parse().await{
            Ok(resp) => println!("{:#?}", resp),
            Err(err) => eprintln!("{err}")
        }
    }
}
