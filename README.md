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

The most popular argument parser, [clap](https://docs.rs/clap/) is a full-featured crate. It provides both a builder and derive style API and has tons of features (help generation, validation, suggestions, completion, etc.). However most people only need the simplest use-cases, which is to parse flags into struct, commands into enums, and generate help text in case of a failure. The crate [gumdrop](https://docs.rs/gumdrop/) is a reasonable alternative in this case, which only supports the derive API and lacks some advanced features, but it has less dependencies and much smaller binary output.

**Why is it so small**: clap's derive is just a convenient layer over their builder API, which makes them always build both of these crates. In gumdrop, only derive is compiled, which results in faster build times and smaller binaries.

**What is the tradeoff**: gumdrop does a commandable job replicating clap's most important features, however it still missing some advanced ones (completion, colours, suggestions). It also has some quirks (no required commands, define help everywhere, etc.), and seems to be very sparsely developed.

**When to use it**: If you only use the argument parser for simpler use-cases, gumdrop can be almost a drop-in replacement for clap. For more complicated argument parsing needs or if the user experience is important clap usually is a better choice.

**Other contenders**: [bpaf](https://docs.rs/bpaf) is a more featureful, frequently developed crate, however it has similar size issues as clap. If you don't want to use the derive API, then [pico-args](https://docs.rs/pico-args) is also good choice.

<details id="argparser">
<summary>Detailed comparison between crates</summary>

Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
bpaf-size | +244kB | +1.10s | 6
clap-size | +520kB | +1.48s | 16
gumdrop-size | +28kB | +0.94s | 6
pico-args-size | +24kB | +0.10s | 1

</details>

## serializer: serde -> miniserde

- serde: +84kB
- miniserde: +40kB (**-52.38%**)

One of the most downloaded crates, [serde](https://docs.rs/serde) is a flexible crate for serializing, often used for serializing JSON with [serde_json](https://docs.rs/serde_json). However if you only use it for JSON serialization, it is probably not worth to install multiple crates, and using [miniserde](https://docs.rs/miniserde) can be preferable. It is a simpler, JSON-only alternative without any monomorphisation or recursion.

**Why is it small**: miniserde drops some of serde's feature, most importantly it's flexibility. It can only serialize to and deserialize from JSON, and it only has one attribute to rename fields. It also avoids monomorphization, making the output much smaller.

**What is the tradeoff**: miniserde can only serialize from JSON and only to strings. It can't serialize data enums. It cannot print error location on failed parsing. Many crates depend on serde explicitly.

**When to use it**: If you only use serde to transform JSON inside your application, miniserde is a good replacement. If you need to deserialize user input, serde is a better choice, because it can print the error location. Also, if your other dependencies depend on serde (and you can't work around it), you have to use that.

**Other contenders**: Another good crate is [nanoserde](https://docs.rs/nanoserde/) which supports multiple formats, without any derive macro. It is faster compilation, but the binary size is not much smaller.

<details id="serializer">
<summary>Detailed comparison between crates</summary>

Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
miniserde-size | +40kB | +0.69s | 8
nanoserde-size | +72kB | +0.43s | 2
serde-size | +84kB | +1.90s | 9

</details>

## http-client: reqwest -> minreq

- reqwest: +2722kB
- minreq: +332kB (**-87.80%**)

For HTTP clients, the most popular crate is [reqwest](https://docs.rs/reqwest). Reqwest is an async crate, which means that it depends on a whole tokio runtime to be pulled in. As a rule of thumb, if you are aiming for small binaries and fast compilation, you should avoid async as long as you can. Especially on HTTP client, where a large number of concurrent requests are rare. A smaller alternative is [minreq](https://docs.rs/minreq) which only offers a blocking API, and focused on being minimal out-of-the-box, and adding features only when necessary.

**Why is it small**: Minreq only supports a blocking API (reqwest also has a blocking API, but it also uses the async version). Minreq has less dependencies, and they can be enabled as you go via feature flags (e.g. json, tls). Reqwest has a large number of dependencies, including hyper and tokio.

**What is the tradeoff**: Minreq only supports HTTP/1.1. It cannot send multipart files and doesn't support streaming or compression (if any of this is a dealbreaker, see *Other contenders/attohttpc*). It doesn't support any preconfiguration (e.g. Client in reqwest).

**When to use it**: If you only want to send simple requests to a server, minreq can be a viable option. If you already have tokio or you need a more featureful solution, you are better off with reqwest.

**Other contenders**: [attohttpc](https://docs.rs/attohttpc) is another option with more features but similar footprint (multipart, streaming, compression). [ureq](https://docs.rs/ureq) is a bit bigger, but better-supported blocking client.

<details id="http-client">
<summary>Detailed comparison between crates</summary>

Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
attohttpc-size | +844kB | +6.60s | 40
minreq-size | +332kB | +5.72s | 25
reqwest-blocking-size | +2670kB | +10.72s | 74
reqwest-size | +2722kB | +12.74s | 79
surf-size | +1574kB | +14.12s | 155
ureq-size | +2112kB | +10.12s | 44

</details>

## TODO: hyper -> tiny_http

hyper:
tiny_http:

**Why is it small**: it is mirroring the previous case, tiny_http is a very minimal blocking HTTP server, so it gets ahead of hyper by avoiding the tokio dependency. However it has very good support for spinning up multiple threads, so it is far from a slow, single-threaded server. If you need an HTTP server for some small feature (e.g. webhook, OAuth), it can be fine, but for real-world backends, you should use a web framework.

**What is the tradeoff**: if you are already using

**Other contenders**: 

## TODO: tracing -> log + simple_logger

## TODO: chrono -> time

## TODO: color-eyre -> anyhow