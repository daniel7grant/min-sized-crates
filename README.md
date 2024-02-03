# min-sized-crates

Based on [min-sized-rust](https://github.com/johnthagen/min-sized-rust), a project aimed to make Rust crates smaller, this project is looking to compare sizes added with crates. The goal is to promote alternative smaller crates to popular, larger packages.

## Baseline

All examples are compiled with --release and --strip enabled. To calculate valid sizes for the crates we look at the size increase from the baseline default "Hello World" crate (355kB).

For all examples, these versions are used (but you are encouraged to repeat these experiment on an OS and version that is important to you):

Rust version: 1.75.0 (82e1608df 2023-12-21)
Arch: x86_64
OS: GNU/Linux

## clap -> gumdrop

clap: +520kB
gumdrop: +28kB (-94.6%)

**Why is it so small**:

**What is the tradeoff**: gumdrop does a commandable job replicating clap's most important features, however it still missing some advanced ones (completion, colours, groups). It also seems to be very sparsely developed.

**Other contenders**: bpaf is a more featureful, frequently developed crate. If you don't want derive, then picoargs is also good.

## TODO: serde -> miniserde

serde:
miniserde:

**Why is it small**: miniserde drops some of serde's feature, most importantly it's flexibility. It can only serialize to and deserialize from JSON, and it only ha one attribute to rename fields. This makes the output much smaller.

**What is the tradeoff**: only JSON can be used, no advanced renaming, flattening, cannot print error location on failed parsing. If you only use serde to transform JSON inside your application, it can be fine.

**Other contenders**: nanoserde.

## TODO: reqwest -> ureq

reqwest:
ureq:

**Why is it small**: reqwest is an async crate (depends on tokio) while ureq is completely blocking. As a rule of thumb, if you are aiming for small binaries and fast compilation, you should avoid async as long as you can.

**What is the tradeoff**:

**Other contenders**:

## TODO: hyper -> tiny_http

hyper:
tiny_http:

**Why is it small**: it is mirroring the previous case, tiny_http is a very minimal blocking HTTP server, so it gets ahead of hyper by avoiding the tokio dependency. However it has very good support for spinning up multiple threads, so it is far from a slow, single-threaded server. If you need an HTTP server for some small feature (e.g. webhook, OAuth), it can be fine, but for real-world backends, you should use a web framework.

**What is the tradeoff**: if you are already using

**Other contenders**: 

## TODO: tracing -> log + simple_logger

## TODO: chrono -> time

## TODO: color-eyre -> anyhow