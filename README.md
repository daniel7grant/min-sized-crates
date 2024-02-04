# min-sized-crates

Based on [min-sized-rust](https://github.com/johnthagen/min-sized-rust), a project aimed to make Rust crates smaller, this project is looking to compare sizes added with crates. The goal is to promote alternative smaller crates to popular, larger packages.

## Baseline

All examples are compiled with --release and strip enabled. To calculate valid sizes for the crates we look at the size increase from the baseline default "Hello World" crate (355kB). This means if a crate builds to 500kB, then it counts as a +145kB increase.

For all examples, these versions are used (but you are encouraged to repeat these experiment on an OS and version that is important to you):

Rust version: 1.75.0 (82e1608df 2023-12-21)
Arch: x86_64
OS: GNU/Linux

## argparser: clap -> gumdrop

- clap: +520kB
- gumdrop: +28kB (**-94.6%**)

The most popular argument parser, [clap](https://docs.rs/clap/) is a full-featured crate. It provides both a builder and derive style API and has tons of features (help generation, validation, suggestions, completion, etc.). However most people only need the simplest use-cases, which is to parse into enums, and generate help text in case of a failure. The crate [gumdrop](https://docs.rs/gumdrop/) is a reasonable alternative in this case, which only supports a derive API

**Why is it so small**: clap's derive is just a convenient layer over their builder API, which makes them always build both of these crates. In gumdrop, only derive is compiled, which results in faster build times and smaller binaries.

**What is the tradeoff**: gumdrop does a commandable job replicating clap's most important features, however it still missing some advanced ones (completion, colours, suggestions). It also has some quirks (no required commands, define help everywhere, etc.), and seems to be very sparsely developed.

**When to use it**: If you only use the argument parser for simpler use-cases, gumdrop can be almost a drop-in replacement for clap. For more complicated argument parsing needs, if the user experience is important clap usually is a better choice.

**Other contenders**: [bpaf](https://docs.rs/bpaf) is a more featureful, frequently developed crate, however it has similar size issues as clap. If you don't want to use the derive API, then [pico-args](https://docs.rs/pico-args) is also good.

<details id="argparser">
<summary>Detailed comparison between crates</summary>

Name | Size | Time | Dependency count
---|:-:|:-:|:-:
bpaf-size | +244kB | +1.09s | 6
clap-size | +520kB | +1.50s | 16
gumdrop-size | +28kB | +0.95s | 6
pico-args-size | +24kB | +0.10s | 1

</details>

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