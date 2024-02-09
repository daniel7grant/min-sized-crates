# Contributing to min-sized-crates

**Contributions to the solutions and results are very welcome.**

The goal of this project is to find crates with the smallest compiled footprint, but as close to the functionality of the original crate as possible. When contributing, make sure to be:

- **simple**: solutions should be as close to the default installation as possible, mostly following the documentation and default flags. If there is a choice, you can choose the smaller (e.g. openssl instead of rustls). The goal is to show how crates would be used in the real world by real users, not how much they can be stripped down. Code golfing is not allowed.
- **transparent**: solutions should be transparent about the possible tradeoffs and how did the crates achieved the small sizes. Core functionality should not be compromised in the search for the smallest crate, and every missing feature should be meticolously documented.
- **thorough**: solutions should be well-researched, providing a fair comparison between crates. As many crates as possible should be taken into account, to reduce bias, and to find the smallest option even if it's less known.
- **reproducible**: solutions should come with a minimal real-world example, that can run on the current stable Rust compiler. The comparisons should be as close as possible, confined to the capabilities of the crates.

Please don't make issues arguing about the results. If you disagree with any description made in this document, try to make an improvement. Be respectful.
