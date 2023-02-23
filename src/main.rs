use board_game_io_base::server::Server;
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

use board_game_io_cross_clues::game::Game;

#[tokio::main]
async fn main() {
    let trace_subscriber = FmtSubscriber::builder()
        .with_span_events(FmtSpan::NONE)
        .finish();
    tracing::subscriber::set_global_default(trace_subscriber)
        .expect("setting tracing default failed");
    Server::<Game>::run("127.0.0.1:9002".to_string()).await;
}
