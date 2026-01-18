use axum::{ http::Response, routing::{ get, post, delete }, Router };
use dashmap::DashMap;
use mystic_bot::{
    api::{
        delete::delete_game::delete_game_handler,
        get::{
            get_eval::eval_position_handler,
            root::root_handler,
            static_eval::static_eval_handler,
        },
        post::{
            add_game::new_game_handler,
            best_move::best_move_handler,
            make_move::make_move_handler,
        },
    },
    bot::include::types::{ GlobalMap, LOGO, ServerState },
};
use std::{ net::SocketAddr, sync::Arc, time::Duration };
use tower_http::trace::{ TraceLayer, DefaultMakeSpan, DefaultOnRequest };
use tracing::Span;
use tracing_subscriber::{ fmt, prelude::* };
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "MysticBot")]
struct Cli {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber
        ::registry()
        .with(
            tracing_subscriber::EnvFilter
                ::try_from_default_env()
                .unwrap_or_else(|_| "mystic_bot=trace,axum=info,tower_http=info".into())
        )
        .with(
            fmt
                ::layer()
                .with_thread_ids(true)
                .with_target(true)
                .with_level(true)
                .with_timer(fmt::time::UtcTime::rfc_3339())
        )
        .init();

    println!("{}", LOGO);

    let cli = Cli::parse();

    let global_map = Arc::new(GlobalMap {});

    let state = ServerState {
        engines: Arc::new(DashMap::new()),
        global_map,
    };

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(log_response);

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/game", post(new_game_handler))
        .route("/game", delete(delete_game_handler))
        .route("/game/best", post(best_move_handler))
        .route("/game/move", post(make_move_handler))
        .route("/eval", get(eval_position_handler))
        .route("/static", get(static_eval_handler))
        .layer(trace_layer)
        .with_state(state);

    GlobalMap::opening_db();

    let addr = SocketAddr::from(([127, 0, 0, 1], cli.port));
    tracing::info!(%addr, "MysticBot server initializing");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn log_response<B>(response: &Response<B>, latency: Duration, span: &Span) where B: std::fmt::Debug {
    let status = response.status();
    tracing::info!(
        parent: span, 
        ?status, 
        ?latency, 
        module = "mystic_bot::api",
        "HTTP response dispatched"
    );
}
