use std::sync::Arc;
use anyhow::Result;
use axum::{http::StatusCode, routing::post, Json, Router};
use dashmap::DashSet;
use log::info;
use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tokio::{signal, task};

static ADDR: &str = "0.0.0.0:3000"; // 使用 IP 地址
const PICK_AMOUNT: usize = 5;
const PASSAGE_AMOUNT: usize = 10;

struct SurveyIdGenerator {
    generated_map: Arc<DashSet<usize>>,
}

impl SurveyIdGenerator {
    fn new() -> Self {
        SurveyIdGenerator {
            generated_map: Arc::new(DashSet::new()),
        }
    }

    async fn generate(&self) -> Result<usize> {
        let map = Arc::clone(&self.generated_map);
        let random_number = task::spawn_blocking(move || {
            let mut rng = rand::thread_rng();
            loop {
                let number: usize = rng.gen_range(100000..=999999);
                if map.insert(number) {
                    return Ok(number);
                }
            }
        }).await.map_err(anyhow::Error::from)?; // 错误处理
        
        random_number
    }
}

static SURVEY_ID_GENERATOR: Lazy<SurveyIdGenerator> = Lazy::new(|| SurveyIdGenerator::new());

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("listening on {}", ADDR);
    
    let app = Router::new()
        .route("/api/generate", post(create_survey));
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    let server = axum::serve(listener, app);
    let graceful = server.with_graceful_shutdown(async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to create signal handler")
            .recv()
            .await;
        info!("Received shutdown signal; shutting down gracefully...");
    });
    info!("Server running...");
    graceful.await.unwrap();
    info!("Server has exited.");
}

async fn create_survey(
    Json(generate_request): Json<GenerateRequest>,
) -> (StatusCode, Json<Survey>) {
    
    match SURVEY_ID_GENERATOR.generate().await {
        Ok(id) => {
            let list = generate_survey_list().await;
            info!("调查员 {} 生成一份问卷，问卷编号 {}，文章编号列表：{:?}", 
                generate_request.researcher_id, id, list);
            let survey = Survey {
                survey_id: id,
                survey_list: list,
            };
            (StatusCode::CREATED, Json(survey))
        },
        Err(err) => {
            info!("生成问卷时出错：{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Survey::default())) // 返回错误时提供默认值
        }
    }
}

async fn generate_survey_list() -> Vec<usize> {
    task::spawn_blocking(|| {
        let numbers: Vec<usize> = (1..=PASSAGE_AMOUNT).collect();
        let mut rng = thread_rng();
        numbers.choose_multiple(&mut rng, PICK_AMOUNT).cloned().collect()
    }).await.unwrap()
}

#[derive(Serialize)]
struct Survey {
    survey_id: usize,
    survey_list: Vec<usize>,
}

#[derive(Deserialize)]
struct GenerateRequest {
    researcher_id: usize,
}

impl Default for Survey {
    fn default() -> Self {
        Survey {
            survey_id: 0,
            survey_list: vec![],
        }
    }
}
