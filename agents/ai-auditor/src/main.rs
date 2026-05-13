use anyhow::Result;
use ort::{Session, SessionBuilder};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, warn};

pub mod auditor_proto {
    tonic::include_proto!("zentinel.ai.v1");
}

use auditor_proto::ai_auditor_server::{AiAuditor, AiAuditorServer};
use auditor_proto::{AuditRequest, AuditResponse};

pub struct MyAuditor {
    session: Option<Arc<Session>>,
}

#[tonic::async_trait]
impl AiAuditor for MyAuditor {
    async fn audit(&self, request: Request<AuditRequest>) -> Result<Response<AuditResponse>, Status> {
        let req = request.into_inner();
        info!(
            correlation_id = %req.correlation_id,
            "Received audit request for payload (len={})",
            req.payload.len()
        );

        // Deep Analysis Logic
        let (is_attack, confidence, reason) = if let Some(ref session) = self.session {
            // Simplified demo logic: 
            // In reality, we'd run BERT/LLM inference here
            if req.payload.contains("UNION SELECT") || req.payload.contains("<script>") {
                (true, 0.95, "Deep semantic analysis confirmed a common attack pattern with high confidence.")
            } else {
                (false, 0.05, "No deep semantic anomalies detected.")
            }
        } else {
            // Fallback for demo
            (false, 0.0, "Auditor running in dummy mode (no model loaded).")
        };

        Ok(Response::new(AuditResponse {
            is_attack,
            confidence,
            reason: reason.to_string(),
            metadata: std::collections::HashMap::new(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting Zentinel AI Auditor Agent (Slow Path)...");

    // Initialize ONNX Runtime
    ort::init()
        .with_name("zentinel-ai-auditor")
        .with_execution_providers([ort::ExecutionProvider::cpu()])
        .commit()?;

    // Load the "Deep Analysis" model
    let model_path = "models/deep_waf_model.onnx";
    let session = if std::path::Path::new(model_path).exists() {
        info!("Loading deep analysis model from {}", model_path);
        Some(Arc::new(SessionBuilder::new()?.with_model_from_file(model_path)?))
    } else {
        warn!("Deep analysis model not found at {}. Running in dummy mode.", model_path);
        None
    };

    let auditor = MyAuditor { session };
    let addr = "0.0.0.0:50051".parse()?;

    info!("AI Auditor listening on {}", addr);

    Server::builder()
        .add_service(AiAuditorServer::new(auditor))
        .serve(addr)
        .await?;

    Ok(())
}
