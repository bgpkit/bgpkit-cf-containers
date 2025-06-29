use axum::extract::Query;
use axum::{Json, Router};
use axum::routing::get;
use bgpkit_parser::BgpElem;
use serde::{Deserialize, Serialize};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[derive(Serialize, Deserialize)]
struct Result {
    error: Option<String>,
    data: Vec<BgpElem>,
    meta: Option<QueryMeta>,
}

#[derive(Serialize, Deserialize)]
struct QueryMeta {
    collector: String,
    prefix: String,
    ts_start: String,
    ts_end: String,
    files: Vec<String>
}

#[derive(Deserialize, Serialize)]
struct Params {
    collector: String,
    prefix: String,
    ts_start: String,
    ts_end: String,
}

async fn parse(Query(params): Query<Params>) -> Json<Result> {
    let result = tokio::task::spawn_blocking(move || {
        // extract parameters from the query
        let collector = params.collector;
        let prefix = params.prefix;
        let ts_start = params.ts_start;
        let ts_end = params.ts_end;

        let files = match bgpkit_broker::BgpkitBroker::new()
            .ts_end(ts_end.clone())
            .ts_start(ts_start.clone())
            .collector_id(collector.clone())
            .query(){
            Ok(items) => items,
            Err(e) => {
                return Json(Result {
                    error: Some(e.to_string()),
                    data: vec![],
                    meta: None,
                });
            }
        };

        let urls: Vec<String> = files.iter().map(|f| f.url.clone()).collect();

        let mut items: Vec<BgpElem> = vec![];

        for file in files {
            let mut parser = match bgpkit_parser::BgpkitParser::new(file.url.as_str()){
                Ok(parser) => parser,
                Err(e) => {
                    return Json(Result {
                        error: Some(e.to_string()),
                        data: vec![],
                        meta: None,
                    });
                }
            };

            parser = match parser.add_filter("prefix", prefix.as_str()){
                Ok(parser) => parser,
                Err(e) => {
                    return Json(Result {
                        error: Some(e.to_string()),
                        data: vec![],
                        meta: None,
                    });
                }
            };
            items.extend(parser.into_elem_iter());
        }

        Json(Result {
            error: None,
            data: items,
            meta: Some(QueryMeta {
                collector,
                prefix,
                ts_start,
                ts_end,
                files: urls,
            }),
        })
    }).await.unwrap();
    result
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/search", get(parse))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO))
        );

    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter("info,bgpkit_broker=error")
        .compact()
        .init();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}