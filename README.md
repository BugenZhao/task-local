# task-local

[![Crates.io](https://img.shields.io/crates/v/task-local.svg)](https://crates.io/crates/task-local)
[![Documentation](https://docs.rs/task-local/badge.svg)](https://docs.rs/task-local)
[![CI Status](https://github.com/BugenZhao/task-local/workflows/CI/badge.svg)](https://github.com/BugenZhao/task-local/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/task-local.svg)](LICENSE)

Task-local storage for asynchronous tasks, extracted from the `tokio::task_local` module.

This crate provides a way to store task-local values across `.await` points without requiring the Tokio runtime.

## Overview

Task-local storage allows you to store and access data that is local to the current asynchronous task. Unlike thread-local storage, task-local values are preserved across `.await` points within the same task.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
task-local = "0.1.1"
```

## Example

```rust
use task_local::task_local;

task_local! {
    static NUMBER: u32;
}

async fn example() {
    NUMBER.scope(1, async {
        // The value 1 is accessible within this async block
        assert_eq!(NUMBER.get(), 1);

        // It's also accessible across .await points
        some_async_function().await;
        assert_eq!(NUMBER.get(), 1);

        // You can nest scopes
        NUMBER.scope(2, async {
            assert_eq!(NUMBER.get(), 2);
        }).await;

        // After the nested scope, the original value is restored
        assert_eq!(NUMBER.get(), 1);
    }).await;
}

async fn some_async_function() {
    // The task-local value is still accessible here
    assert_eq!(NUMBER.get(), 1);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
