# tracing-ohos

Composable tracing layer which logs to logcat using the [OpenHarmony NDK]'s
`OH_LOG_Print` function. The provided tag will be capped at 23 bytes.
Logging events resulting in messages longer than 4000 bytes will result in
multiple log lines in logcat. This avoids running into logcat's truncation
behaviour.

This crate is mainly based on the [tracing-android] crate.

License: MIT OR Apache-2.0

[OpenHarmony NDK]: https://developer.huawei.com/consumer/en/doc/harmonyos-guides-V5/hilog-guidelines-ndk-V5#available-apis
[tracing-android]: https://crates.io/crates/tracing-android

## Example
Constructs a `Layer` with the given `tag`.
```rust
 use tracing_subscriber::layer::SubscriberExt;
 use tracing_subscriber::util::SubscriberInitExt;

 let ohrs_writer_layer = tracing_ohos::layer(0x0000, "TAG")?;

 tracing_subscriber::registry()
    .with(ohrs_writer_layer)
    .with(filter)
    .init();
```