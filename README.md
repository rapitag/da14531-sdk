# DA14531 SDK Rust bindings

[![Crates.io](https://img.shields.io/crates/d/da14531-sdk.svg)](https://crates.io/crates/da14531-sdk)
[![Crates.io](https://img.shields.io/crates/v/da14531-sdk.svg)](https://crates.io/crates/da14531-sdk)
[![Released API docs](https://docs.rs/da14531-sdk/badge.svg)](https://docs.rs/da14531-sdk)

This crate is a wrapper for the manufacturer's SDK. In order to build it, you need to first download the SDK version `6.0.16.1144`.
In order to compile this crate you need to set the `SDK_PATH` environment variable to the SDK path, e.g. `SDK_PATH=/path/to/my/sdk/6.0.16.1144`

Parts of the SDK where completely reimplemented in Rust.


This is highly experimental and still in development at the moment, use with caution!