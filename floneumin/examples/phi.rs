use std::io::Write;

use floneumin_language::{
    local::Phi,
    model::{GenerationParameters, Model},
};
use futures_util::stream::StreamExt;
use floneumin_streams::text_stream::TextStream;

#[tokio::main]
async fn main() {
    let mut llm = Phi::start().await;
    let prompt = "The following is a 300 word essay about why the capital of France is Paris:";
    print!("{}", prompt);

    let stream = llm
        .stream_text(prompt, GenerationParameters::default().with_max_length(300))
        .await
        .unwrap();

    let mut sentences = stream.words();
    while let Some(text) = sentences.next().await {
        print!("{}", text);
        std::io::stdout().flush().unwrap();
    }
}
