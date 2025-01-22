# min-sized-crates

Based on [min-sized-rust](https://github.com/johnthagen/min-sized-rust), a project aimed to make Rust crates smaller, this project is looking to compare sizes added with crates. The goal is to promote alternative smaller crates to popular, larger packages.

Please note that this repository only compares crates based on one metric, which is compiled size. It tries to give as much information as possible about the tradeoffs, but your needs may vary. If you have any suggestions, feel free to [contribute](./CONTRIBUTING.md).

## Baseline

All examples are compiled with `--release` and `strip` enabled. To calculate valid sizes for the crates we look at the size increase from the baseline default "Hello World" crate (<span id="information/baseline-size">350kB</span>). This means if a crate builds to 500kB, then it counts as a <span id="information/baseline-example-size">+150kB</span> increase.

**Note**: compile times and even sizes can vary between OSs, so these are mostly for indication. You are encouraged to repeat these experiments on an OS and version that is important to you. For all examples, these versions are used:

- Rust version: <span id="information/rust-version">rustc 1.84.0 (9fc6b4312 2025-01-07)</span>
- Arch: <span id="information/arch">x86_64</span>
- OS: <span id="information/os">Linux-6.12.10-arch1-1-x86_64-with-glibc2.40</span>

If you want to reproduce the results, you need [Python](https://www.python.org/downloads/). Run this script to print results and generate a `README.results.md` with data on your device:

```sh
python build.py
```

This script will cache the results. To run the script again without caching:

```sh
FORCE_NO_CACHE=true python build.py
```

## Results

### argparser: clap -> gumdrop

- clap: <span id="argparser/clap-size">+536kB</span>
- gumdrop: <span id="argparser/gumdrop-size">+32kB</span> (**<span id="argparser/gumdrop-size/clap-size">-94.03%</span>**)

The most popular argument parser, [clap](https://docs.rs/clap/) is a full-featured crate. It provides both a builder and derive style API and has tons of features (help generation, validation, suggestions, completion, etc.). However most people only need the simplest use-cases, which is to parse flags into struct, commands into enums, and generate help text in case of a failure. The crate [gumdrop](https://docs.rs/gumdrop/) is a reasonable alternative in this case, which only supports the derive API and lacks some advanced features, but it has less dependencies and much smaller binary output.

**Why is it so small**: `clap`'s derive is just a convenient layer over their builder API, which makes them always build both of these crates. In `gumdrop`, only derive is compiled, which results in faster build times and smaller binaries.

**What is the tradeoff**: `gumdrop` does a commandable job replicating `clap`'s most important features, however it still missing some advanced ones (completion, colours, suggestions). It also has some quirks (no required commands, define help everywhere, etc.), and seems to be very sparsely developed.

**When to use it**: If you only use the argument parser for simpler use-cases, `gumdrop` can be almost a drop-in replacement for `clap`. For more complicated argument parsing needs or if the user experience is important `clap` usually is a better choice.

**Other contenders**: [bpaf](https://docs.rs/bpaf) is a more featureful, frequently developed crate, however it has similar size issues as `clap`. If you don't want to use the derive API, then [pico-args](https://docs.rs/pico-args) is also good choice.

<details id="argparser">
<summary>Detailed comparison between crates</summary>


Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
bpaf-size | +248kB | +3.30s | 6
clap-size | +536kB | +3.12s | 17
gumdrop-size | +32kB | +1.72s | 6
pico-args-size | +20kB | +0.21s | 1
</details>

### serializer: serde -> miniserde

- serde: <span id="serializer/serde-size">+92kB</span>
- miniserde: <span id="serializer/miniserde-size">+44kB</span> (**<span id="serializer/miniserde-size/serde-size">-52.18%</span>**)

One of the most downloaded crates, [serde](https://docs.rs/serde) is a flexible crate for serializing, often used for serializing JSON with [serde_json](https://docs.rs/serde_json). However if you only use it for JSON serialization, it is probably not worth to install multiple crates, and using [miniserde](https://docs.rs/miniserde) can be preferable. It is a simpler, JSON-only alternative without any monomorphisation or recursion.

**Why is it small**: `miniserde` drops some of `serde`'s feature, most importantly it's flexibility. It can only serialize to and deserialize from JSON, and it only has one attribute to rename fields. It also avoids monomorphization, making the output much smaller.

**What is the tradeoff**: `miniserde` can only serialize from JSON and only to strings. It can't serialize data enums. It cannot print error location on failed parsing. Many crates depend on `serde` explicitly.

**When to use it**: If you only use `serde` to transform JSON inside your application, `miniserde` is a good replacement. If you need to deserialize user input, `serde` is a better choice, because it can print the error location. Also, if your other dependencies depend on `serde` (and you can't work around it), you have to use that.

**Other contenders**: Another good crate is [nanoserde](https://docs.rs/nanoserde/) which supports multiple formats, without any derive macro. It is faster compilation, but the binary size is not much smaller.

<details id="serializer">
<summary>Detailed comparison between crates</summary>


Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
miniserde-size | +44kB | +1.74s | 8
nanoserde-size | +72kB | +0.85s | 2
serde-size | +92kB | +4.37s | 10
</details>

### logging: tracing -> log + simple_logger

For libraries:
- tracing: <span id="logging/tracing-lib-size">+72kB</span>
- log: <span id="logging/log-lib-size">+4kB</span> (**<span id="logging/log-lib-size/tracing-lib-size">-94.45%</span>**)

For applications:
- tracing + tracing-subscriber: <span id="logging/tracing-size">+213kB</span>
- log + simple_logger: <span id="logging/simple-logger-size">+64kB</span> (**<span id="logging/simple-logger-size/tracing-size">-70.02%</span>**)

The new default for logging in Rust is [tracing](https://docs.rs/tracing). It has spans to describe events that has a beginning and the end, and can output structured logging (JSON format). It also has a great [ecosystem of crates](https://docs.rs/tracing/latest/tracing/#related-crates), for example exporting to OpenTelemetry or Loki. However these features are not needed for every application, and for these the original logging crate [log](https://docs.rs/log) is often a good alternative. Especially for applications, `log` with [simple_logger](https://docs.rs/simple_logger) can be much smaller than `tracing` with [tracing-subscriber](https://docs.rs/tracing-subscriber).

**Why is it small**: `log` is a very lightweight crate, that only provides the API for logging. Implementation logic is up to the loggers, and there are crates like `simple_logger` that are aiming for a minimal solution with very few dependencies. It also doesn't have key-value pairs, further simplifying the code.

**What is the tradeoff**: `log` doesn't have any capability to structured logging, no key-value pairs. It also doesn't support spans. Because of this, it cannot connect to OpenTelemetry-compatible collectors.

**When to use it**: If you only want to print messages with a log-level, `log` can be a good choice. Especially in libraries, `log` can be better (if there is no need for spans), because `tracing`-using applications can use [tracing-log](https://docs.rs/tracing-log) for compatibility. If you are writing an application that needs spans, structured logging or uses OpenTelemetry, `tracing` is a better option.

**Other contenders**: [simplelog](https://docs.rs/simplelog) is also a good logger, that can be configured with a wide variety of settings, but the defaults are not particularly good. If you want to use structured logging with JSON output, you can try [slog](https://docs.rs/slog), but it isn't much smaller than `tracing-subscriber` with `json` enabled.

<details id="logging">
<summary>Detailed comparison between crates</summary>


Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
env-logger-size | +1264kB | +3.53s | 16
log-lib-size | +4kB | +0.11s | 1
simple-logger-size | +64kB | +1.54s | 13
simplelog-size | +68kB | +1.64s | 12
slog-size | +236kB | +2.68s | 19
tracing-json-size | +341kB | +3.51s | 25
tracing-lib-size | +72kB | +2.73s | 10
tracing-size | +213kB | +3.07s | 19
</details>

### regex: regex -> regex-lite

- regex: <span id="regex/regex-size">+1804kB</span>
- regex-lite: <span id="regex/regex-lite-size">+108kB</span> (**<span id="regex/regex-lite-size/regex-size">-94.01%</span>**)

Unlike other languages, in Rust regular expressions are not part of the standard library but powered by a third-party crate [regex](https://docs.rs/regex). This is a crate optimized for runtime performance and correctness [even at the cost of build performance](https://docs.rs/regex/latest/regex/#crate-features). It's a good tradeoff for most cases, however it can lead to slow compilation and large binaries. To address this, BurntSushi, the original author of `regex` introduced a lightweight alternative crate [regex-lite](https://docs.rs/regex-lite). This aims to be a drop-in replacement, as correct as the original `regex` ([passing the original tests](https://www.reddit.com/r/rust/comments/14ralde/comment/jqrlmsd/)) but without performance optimizations.

**Why is it small**: It was specifically written with compile times in mind with zero dependencies and a smaller codebase.

**What is the tradeoff**: The main tradeoff is [lower performance](https://github.com/BurntSushi/rebar#summary-of-search-time-benchmarks). Apart from this, it can only search string slices, has worse error handling and has only partial support for Unicode. For a full comparison, see [Differences with the regex crate](https://docs.rs/regex-lite/latest/regex_lite/#differences-with-the-regex-crate).

**When to use it**: If regex parsing is not on your hot path and Unicode is not needed, you can drop in `regex-lite` for `regex` to get a smaller binary.

**Other contenders**: [onig](https://docs.rs/onig) is a crate that binds to the Oniguruma C library. Linking statically is much larger than `regex-lite` (however still an improvement over `regex`), but if you are sure that your users have `libonig` installed it can be [dynamically linked](https://github.com/rust-onig/rust-onig#linking).

<details id="regex">
<summary>Detailed comparison between crates</summary>

Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
fancy-regex-size | +2104kB | +4.56s | 7
onig-size | +526kB | +6.57s | 7
regex-lite-size | +108kB | +0.69s | 1
regex-no-unicode-size | +524kB | +2.15s | 3
regex-size | +1804kB | +3.62s | 5
</details>

### http-client: reqwest -> minreq

- reqwest: <span id="http-client/reqwest-size">+2563kB</span>
- minreq: <span id="http-client/minreq-size">+348kB</span> (**<span id="http-client/minreq-size/reqwest-size">-86.42%</span>**)

For HTTP clients, the most popular crate is [reqwest](https://docs.rs/reqwest). `reqwest` is an async crate, which means that it depends on a whole `tokio` runtime to be pulled in. As a rule of thumb, if you are aiming for small binaries and fast compilation, you should avoid async as long as you can. Especially on HTTP client, where a large number of concurrent requests are rare. A smaller alternative is [minreq](https://docs.rs/minreq) which only offers a blocking API, and focused on being minimal out-of-the-box, and adding features only when necessary.

**Why is it small**: `minreq` only supports a blocking API (`reqwest` also has a blocking API, but it also uses the async version). `minreq` has less dependencies, and they can be enabled as you go via feature flags (e.g. json, tls). `reqwest` has a large number of dependencies, including `hyper` and `tokio`.

**What is the tradeoff**: `minreq` only supports HTTP/1.1. It cannot send multipart files and doesn't support streaming or compression (if any of this is a dealbreaker, see *Other contenders/attohttpc*). It doesn't support any preconfiguration (e.g. `Client` in `reqwest`).

**When to use it**: If you only want to send simple requests to a server, `minreq` can be a viable option. If you already have `tokio` or you need a more featureful solution, you are better off with `reqwest`.

**Other contenders**: [attohttpc](https://docs.rs/attohttpc) is another option with more features but similar footprint (multipart, streaming, compression). [ureq](https://docs.rs/ureq) is a bit bigger, but better-supported blocking client. If you are fine with external linking and a more cumbersome API, you can also use [curl](https://docs.rs/curl), which gives even better size improvements.

<details id="http-client">
<summary>Detailed comparison between crates</summary>

All crates were run with default options or as presented in the documentation. If there was a choice `native-tls` was chosen instead of `rustls` (except `ureq`, where the default is `rustls`).


Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
attohttpc-size | +804kB | +8.24s | 65
curl-size | +140kB | +4.93s | 21
minreq-size | +348kB | +5.73s | 27
reqwest-blocking-size | +1609kB | +10.53s | 95
reqwest-size | +2563kB | +12.91s | 107
surf-size | +1530kB | +14.07s | 175
ureq-size | +2116kB | +11.39s | 63
</details>

### http-server: hyper -> tiny_http

- hyper: <span id="http-server/hyper-size">+748kB</span>
- tiny_http: <span id="http-server/tiny-http-size">+360kB</span> (**<span id="http-server/tiny-http-size/hyper-size">-51.91%</span>**)

If you need a small HTTP-server for some minimal use-case (webhook, runtime configuration), a popular choice is the low-level HTTP library [hyper](https://docs.rs/hyper). However this is a full-featured async crate and it has a lot of dependencies, including `tokio`. A minimal, blocking HTTP server is [tiny_http](https://docs.rs/tiny_http). Despite being synchronous, `tiny_http` has very good support for spinning up multiple threads, so it is far from a slow, single-threaded server.

**Why is it small**: Mirroring the previous case, `tiny_http` is a blocking HTTP server, so it gets ahead of `hyper` by avoiding the `tokio` dependency. `tiny_http` is carefully written to use the least amount of dependencies, without SSL at the moment only 4.

**What is the tradeoff**: The main tradeoff is that `tiny_http` only supports HTTP/1.1. For high concurrent requests, the blocking / threadpool API is slower, so for performance, async might be preferable.

**When to use it**: If you need an HTTP server for some small feature (e.g. webhook, OAuth), `tiny_http` is a good choice. If you want to build a website, you should use a web framework, like [axum](https://docs.rs/axum).

**Other contenders**: If you are looking for a minimal framework, [rouille](https://docs.rs/rouille), which builds on `tiny_http`, is a good option. If you want to use a framework a popular choice is [axum](https://docs.rs/axum).

<details id="http-server">
<summary>Detailed comparison between crates</summary>


Name | Size | Compile time | Dependency count
---|:-:|:-:|:-:
axum-size | +992kB | +8.87s | 54
hyper-size | +748kB | +8.55s | 47
rouille-size | +436kB | +8.05s | 87
tiny-http-size | +360kB | +0.95s | 5
</details>

