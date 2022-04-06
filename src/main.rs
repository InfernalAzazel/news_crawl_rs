mod crawl;
mod utils;

use crawl::news_cctv_com::NewsCctvCom;

#[tokio::main]
async fn main() {
    NewsCctvCom::new().start().await;
}
