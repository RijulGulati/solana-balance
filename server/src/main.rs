use actix_web::{
    get,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use solana_balance::{self, Cluster, SolanaError};
use serde::{Deserialize, Serialize};

mod config;

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::read_config();
    HttpServer::new(|| {
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method();
        App::new().wrap(cors).service(get_balance)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}

#[derive(Serialize)]
struct SolanaResponse<T> {
    data: T,
    status: u8,
}

#[derive(Deserialize)]
struct BalanceRequest {
    pubkey: Option<String>,
    cluster: Option<i32>,
}

impl<T> SolanaResponse<T> {
    pub fn success(t: T) -> SolanaResponse<T> {
        SolanaResponse { data: t, status: 0 }
    }

    pub fn error(t: T) -> SolanaResponse<T> {
        SolanaResponse { data: t, status: 1 }
    }
}

#[get("/balance")]
async fn get_balance(req: web::Query<BalanceRequest>) -> impl Responder {
    let pubkey = match &req.pubkey {
        Some(key) => {
            if key.len() == 0 {
                return HttpResponse::BadRequest().json(SolanaResponse::error(SolanaError {
                    error: String::from("missing pubkey"),
                }));
            }
            key
        }
        None => {
            return HttpResponse::BadRequest().json(SolanaResponse::error(SolanaError {
                error: String::from("missing pubkey"),
            }));
        }
    };

    let req_cluster = match &req.cluster {
        Some(c) => c,
        None => {
            return HttpResponse::BadRequest().json(SolanaResponse::error(SolanaError {
                error: String::from("missing cluster"),
            }));
        }
    };

    println!("Received pubkey: {}", pubkey);
    println!("Received cluster: {}", req_cluster);

    let cluster = match req_cluster {
        1 => Cluster::MainnetBeta,
        2 => Cluster::Testnet,
        3 => Cluster::Devnet,
        _ => {
            return HttpResponse::BadRequest().json(SolanaResponse::error(SolanaError {
                error: String::from("invalid cluster"),
            }))
        }
    };

    match solana_balance::get_solana_balance(&pubkey, cluster) {
        Ok(balance) => HttpResponse::Ok().json(SolanaResponse::success(balance)),
        Err(error) => HttpResponse::Ok().json(SolanaResponse::error(error)),
    }
}
