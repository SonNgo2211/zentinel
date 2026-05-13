# Zentinel AI Auditor (Slow Path)

Standalone AI service for deep semantic security analysis. This service acts as the "Second Opinion" for the Zentinel ecosystem.

## Features
- **Deep Analysis**: Uses heavy ONNX models (DistilBERT/Transformers) for semantic attack detection.
- **gRPC API**: High-performance interface for Control Plane integration.
- **Project-Aware**: Supports multi-tenant auditing with project-specific context.
- **Rust Core**: Built with Rust for maximum performance and memory safety.

## Getting Started

### Prerequisites
- Rust 1.82+
- Protobuf Compiler (`protoc`)
- ONNX Runtime libraries

### Installation
```bash
cargo build --release
```

### Running
```bash
# Start the auditor on port 50051
./target/release/zentinel-agent-ai-auditor
```

## gRPC API
The auditor exposes a single service `zentinel.ai.v1.AiAuditor`:

- `Audit(AuditRequest) -> AuditResponse`: Analyzes a payload and returns an attack probability score (0.0 to 1.0) and a semantic reasoning string.

## Model Management
Models should be placed in the `/app/models/` directory inside the container.
- `deep_waf_model.onnx`: The primary semantic analysis model.
- The service automatically reloads if the model file is updated (requires restart in the current version).

## Performance Optimization
For production use, it is recommended to run this agent on hardware with AVX-512 support or a dedicated GPU.
- Set `ORT_STRATEGY=parallel` for multi-threaded inference.
- Use `ExecutionProvider::cuda()` if a GPU is available (requires rebuilding with `cuda` feature).

## Monitoring
The agent provides standard tracing logs. Integrate with OpenTelemetry for distributed tracing to correlate audits with the original WAF events.
