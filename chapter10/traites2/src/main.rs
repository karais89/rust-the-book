use traites2::client::{Tweet, Summary};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("러스트 언어 공부를 시작했습니다."),
        reply: false,
        retweet: false,
    };

    println!("새 트윗 1개: {}", tweet.summarize());
}
