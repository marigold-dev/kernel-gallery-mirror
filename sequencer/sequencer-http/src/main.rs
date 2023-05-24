use actix_cors::Cors;
use actix_web::{
    http::StatusCode,
    web::{self, Json, Query},
    App, HttpResponseBuilder, HttpServer, Responder,
};
use sequencer::{Kernel, NativeNode, Node, Runtime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Path {
    path: String,
}

#[derive(Deserialize, Serialize)]
pub struct Body {
    pub data: String,
}

async fn post_message<N: Node>(node: web::Data<N>, body: Json<Body>) -> impl Responder {
    let data = hex::decode(&body.data).unwrap();

    node.submit_operation(data).await;

    "Operation submitted"
}

async fn get_state_value<N: Node>(node: web::Data<N>, query: Query<Path>) -> impl Responder {
    let res = node.as_ref().get_value(&query.path).await;
    match res {
        Some(data) => {
            let res = hex::encode(data);
            HttpResponseBuilder::new(StatusCode::OK).body(res)
        }
        None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish(),
    }
}

async fn get_state_subkeys<N: Node>(node: web::Data<N>, query: Query<Path>) -> impl Responder {
    let res = node.as_ref().get_subkeys(&query.path).await;
    match res {
        Some(data) => {
            let json = serde_json::to_string(&data);
            match json {
                Ok(json) => HttpResponseBuilder::new(StatusCode::OK).body(json),
                Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
            }
        }
        None => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}

struct MyKernel {}

impl Kernel for MyKernel {
    fn entry<R: Runtime>(host: &mut R) {
        kernel::entry(host)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sled_database_uri = "/tmp/sequencer-storage";
    /*let tezos_node_uri = "http://localhost:8732";
    let rollup_node_uri = "http://localhost:8732";*/
    let tezos_node_uri = "http://ghostnet.tezos.marigold.dev";
    let rollup_node_uri = "http://ghostnet.tezos.marigold.dev";

    let node =
        sequencer::NativeNode::new::<MyKernel>(sled_database_uri, tezos_node_uri, rollup_node_uri);
    let state = web::Data::new(node);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .route("/operations", web::post().to(post_message::<NativeNode>))
            .route("/state/value", web::get().to(get_state_value::<NativeNode>))
            .route(
                "/state/subkeys",
                web::get().to(get_state_subkeys::<NativeNode>),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
