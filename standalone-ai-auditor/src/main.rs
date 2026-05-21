use anyhow::Result;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, warn};

pub mod auditor_proto {
    tonic::include_proto!("zentinel.ai.v1");
}

use auditor_proto::ai_auditor_server::{AiAuditor, AiAuditorServer};
use auditor_proto::{AuditRequest, AuditResponse};

pub struct MyAuditor;

impl MyAuditor {
    /// Heuristic-based deep analysis (no ONNX model required)
    /// Detects common attack patterns using semantic rules
    fn analyze_payload(&self, payload: &str) -> (bool, f32, String) {
        let payload_lower = payload.to_lowercase();

        // SQL Injection patterns
        let sqli_patterns = [
            "union select", "or 1=1", "' or '", "drop table",
            "insert into", "delete from", "update set",
            "exec(", "execute(", "waitfor delay", "benchmark(",
            "load_file(", "into outfile", "into dumpfile",
            "information_schema", "group_concat(", "char(",
        ];

        // XSS patterns
        let xss_patterns = [
            "<script", "javascript:", "onerror=", "onload=",
            "onfocus=", "onmouseover=", "eval(", "document.cookie",
            "alert(", "prompt(", "confirm(", "fromcharcode",
        ];

        // Command Injection patterns
        let cmdi_patterns = [
            "; ls", "| cat", "&& cat", "$(", "`id`",
            "; wget", "; curl", "/etc/passwd", "/etc/shadow",
            "| nc ", "; nc ", "bash -i",
        ];

        // Path Traversal patterns
        let traversal_patterns = [
            "../", "..\\", "%2e%2e", "....//",
            "/etc/passwd", "/proc/self", "/windows/system32",
        ];

        // Check each category
        for pattern in &sqli_patterns {
            if payload_lower.contains(pattern) {
                return (true, 0.92, format!(
                    "Deep semantic analysis confirmed SQL Injection pattern: '{}'", pattern
                ));
            }
        }

        for pattern in &xss_patterns {
            if payload_lower.contains(pattern) {
                return (true, 0.90, format!(
                    "Deep semantic analysis confirmed XSS pattern: '{}'", pattern
                ));
            }
        }

        for pattern in &cmdi_patterns {
            if payload_lower.contains(pattern) {
                return (true, 0.93, format!(
                    "Deep semantic analysis confirmed Command Injection pattern: '{}'", pattern
                ));
            }
        }

        for pattern in &traversal_patterns {
            if payload_lower.contains(pattern) {
                return (true, 0.88, format!(
                    "Deep semantic analysis confirmed Path Traversal pattern: '{}'", pattern
                ));
            }
        }

        // Entropy-based anomaly detection (high entropy = possible obfuscation)
        let entropy = calculate_entropy(payload);
        if entropy > 4.5 && payload.len() > 50 {
            return (true, 0.65, format!(
                "High entropy payload detected ({:.2}), possible obfuscated attack", entropy
            ));
        }

        (false, 0.05, "No deep semantic anomalies detected.".to_string())
    }
}

/// Calculate Shannon entropy of a string
fn calculate_entropy(s: &str) -> f32 {
    let mut freq = [0u32; 256];
    let len = s.len() as f32;
    for byte in s.bytes() {
        freq[byte as usize] += 1;
    }
    let mut entropy: f32 = 0.0;
    for &count in &freq {
        if count > 0 {
            let p = count as f32 / len;
            entropy -= p * p.log2();
        }
    }
    entropy
}

#[tonic::async_trait]
impl AiAuditor for MyAuditor {
    async fn audit(&self, request: Request<AuditRequest>) -> Result<Response<AuditResponse>, Status> {
        let req = request.into_inner();
        info!(
            correlation_id = %req.correlation_id,
            payload_len = req.payload.len(),
            "Received audit request"
        );

        let (is_attack, confidence, reason) = self.analyze_payload(&req.payload);

        if is_attack {
            warn!(
                correlation_id = %req.correlation_id,
                confidence = confidence,
                reason = %reason,
                "Attack confirmed by deep analysis"
            );
        }

        Ok(Response::new(AuditResponse {
            is_attack,
            confidence,
            reason,
            metadata: std::collections::HashMap::new(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting Zentinel AI Auditor Agent (Slow Path - Heuristic Mode)...");

    let auditor = MyAuditor;
    let addr = "0.0.0.0:50051".parse()?;

    info!("AI Auditor listening on {}", addr);

    Server::builder()
        .add_service(AiAuditorServer::new(auditor))
        .serve(addr)
        .await?;

    Ok(())
}
