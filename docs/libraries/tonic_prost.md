# tonic / prost

`tonic` and `prost` form the gRPC communication stack in StrataForge. They are used to bridge the Rust-based desktop application with the Python-based AI microservice.

## Overview

In StrataForge, we use gRPC for communication between our desktop workstation and specialized microservices. This provides a strongly-typed, high-performance interface for complex data structures like seismic slices and AI-detected fault masks.

### Why tonic/prost?
- **Efficiency:** gRPC uses the binary Protocol Buffers (protobuf) format, which is much faster than JSON for large binary data.
- **Language Agnostic:** Protobuf allows us to define our data structures once and generate code for both Rust and Python.
- **Async Support:** `tonic` is built on top of `tokio` and `tower`, providing a modern async API for networking.

## Usage in Project

The communication contract is defined in `crates/sf_core/proto/analysis.proto`.

### Client Implementation
Located in `crates/sf_app/src/ai_client.rs`:

```rust
use anyhow::Result;
use tonic::transport::Channel;

pub mod strataforge {
    pub mod analysis {
        // Automatically include generated code from proto
        tonic::include_proto!("strataforge.analysis");
    }
}

use strataforge::analysis::detection_client::DetectionClient;
use strataforge::analysis::SliceRequest;

pub struct AiClient {
    client: DetectionClient<Channel>,
}

impl AiClient {
    pub async fn connect(addr: String) -> Result<Self> {
        let client = DetectionClient::connect(addr).await?;
        Ok(Self { client })
    }

    pub async fn detect_faults(&mut self, data: Vec<u8>, width: u32, height: u32) -> Result<(Vec<u8>, f32)> {
        let request = tonic::Request::new(SliceRequest { data, width, height });
        let response = self.client.detect_faults(request).await?.into_inner();
        Ok((response.mask, response.confidence))
    }
}
```

## Key Concepts

- **Protocol Buffers (Protobuf):** A language-neutral, platform-neutral, extensible mechanism for serializing structured data.
- **Service:** A set of RPC methods defined in a `.proto` file.
- **tonic:** A gRPC client and server implementation for Rust.
- **prost:** A protobuf implementation for Rust that generates high-quality, idiomatic code.
- **tonic-build:** A build-time tool that converts `.proto` files into Rust source code (configured in `sf_app/build.rs`).

## Resources

- [Official tonic Website](https://tonic.rs/)
- [tonic Documentation (docs.rs)](https://docs.rs/tonic)
- [prost Documentation (docs.rs)](https://docs.rs/prost)
- [Protocol Buffers Specification](https://protobuf.dev/)
- [gRPC Specification](https://grpc.io/docs/what-is-grpc/introduction/)
