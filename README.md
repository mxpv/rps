# rps

[![CI](https://github.com/mxpv/rps/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/mxpv/rps/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/mxpv/rps/status.svg)](https://deps.rs/repo/github/mxpv/rps)

Rust wrappers for AMD Render Pipeline Shaders (RPS) SDK.

*NOTE:* This is very much a work in progress.
The API is not stable until 1.0 and may change at any time.

Reading:
- [RPS SDK Github](https://github.com/GPUOpen-LibrariesAndSDKs/RenderPipelineShaders)
- [GPU Open Announce](https://gpuopen.com/learn/rps_1_0/)
- [RPS SDK License](https://github.com/GPUOpen-LibrariesAndSDKs/RenderPipelineShaders/blob/main/LICENSE.txt)

This repository contains 2 crates:

| Name | Description | Links |
| --- | --- | --- |
| [`rps`](./crates/rps) | High-level Rust wrapper for RPS SDK | [![Crates.io](https://img.shields.io/crates/v/rps.svg)](https://crates.io/crates/rps) [![Docs](https://docs.rs/rps/badge.svg)](https://docs.rs/rps) |
| [`rps-sys`](./crates/rps-sys) | Unsafe bindings to RPS SDK generated with `bindgen` | [![Crates.io](https://img.shields.io/crates/v/rps-sys.svg)](https://crates.io/crates/rps-sys) [![Docs](https://docs.rs/rps-sys/badge.svg)](https://docs.rs/rps-sys) |
