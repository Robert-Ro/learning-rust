# Rust Is The Future of JavaScript Infrastructure

Rust is a fast, reliable, and memory-efficient programming language.
Now rust is used widely in fields like **systems infrastructure**, **encryption**, **virtualization**, and **more low-level programming**.

Why is Rust now being used to replace parts of the JavaScript web ecosystem like **minification** (Terser), **transpilation** (Babel), **formatting** (Prettier), **bundling** (webpack), **linting** (ESLint), and more?

## What is Rust?

略

## From JavaScript to Rust

JavaScript is the most widely used programming language, operating on every device with a web browser. Over the past ten years, a massive ecosystem has been built around JavaScript:

- **Webpack**: developers wanted to bundle multiple JavaScript files into one.
- **Babel**: developers wanted to write modern JavaScript while supporting older browsers.
- **Terser**: developers wanted to generate the smallest possible file sizes.
- **Prettier**: developers wanted an opinionated code formatter that just worked.
- **ESLint**: developers wanted to find issues with their code before deploying.

This has worked well, but we've **reached peak optimization with JS**. This has inspired a new class of tools, designed to drastically improve the performance of building for the web.

### SWC

SWC, created in 2017, is an extensible Rust-based platform for the next generation of fast developer tools. It's used by tools like **Next.js**, **Parcel**, and **Deno**, as well as companies like Vercel, ByteDance, Tencent, Shopify, and more.

SWC can be used for **compilation**, **minification**, **bundling**, and more – and is designed to be extended. It's something you can call to perform code transformations (either built-in or custom). Running those transformations happens through higher-level tools like Next.js.

### Deno

Deno’s linter, code formatter, and docs generator are built using SWC.

### esbuild

> Competitor

### Rome

Rome is currently written in TypeScript and runs on Node.js. But they're now **working on rewriting in Rust** using **RSLint parser** and **their own visitor system for AST traversal**.

### NAPI

Rust’s integration with Node.js is better than other low-level languages.

`napi-rs` allows you to build pre-compiled Node.js add-ons with Rust. It provides an out-of-the-box solution for cross-compilation and publishing native binaries to NPM, without needing `node-gyp` or `postinstall` scripts.

You can build a Rust module that can be called directly from Node.js, without needing to create a child process like esbuild.

### Rust + WebAssembly

> c/c++, go, Rust 都有机会
> `WebAssembly` (WASM) is a portable low-level language that Rust can compile to. It runs in the browser, is interoperable with JavaScript, and is supported in all major modern browsers.

## Why Not Rust?

Rust has a **steep learning curve**. It's a lower level of abstraction than what most web developers are used to.

> Rust makes you think about dimensions of your code that matter tremendously for systems programming. It makes you think about how memory is shared or copied. It makes you think about real but unlikely corner cases and make sure that they’re handled. It helps you write code that’s incredibly efficient in every possible way. – Tom MacWright ([Source](https://macwright.com/2021/01/15/rust.html))

Further, Rust's usage in the web community is still niche. It hasn't reached critical adoption. Even though learning Rust for JavaScript tooling will be a barrier to entry, interestingly developers would rather have a [faster tool that's harder to contribute to](https://twitter.com/devongovett/status/1261379312898306048). [Fast software wins](https://craigmod.com/essays/fast_software/).

Currently, it's hard to find a Rust library or framework for your favorite services (things like working with authentication, databases, payments, and more). We need existing JavaScript tools to help us bridge the gap and incrementally adopt performance improvements.
## The Future of JavaScript Tooling
I believe Rust is the future of JavaScript tooling. [Next.js 12](http://nextjs.org/12) started our transition to fully replace Babel (transpilation) and Terser (minification) with SWC and Rust. Why?
- **Extensibility**: SWC can be used as a Crate inside Next.js, without having to fork the library or workaround design constraints.
- **Performance**: We were able to achieve ~3x faster Fast Refresh and ~5x faster builds in Next.js by switching to SWC, with more room for optimization still in progress.
- **WebAssembly**: Rust's support for WASM is essential for supporting all possible platforms and taking Next.js development everywhere.
- **Community**: The Rust community and ecosystem are amazing and only growing.

It's early days for Rust – a few important pieces are **still being figured out**:

- **Plugins**: Writing plugins in Rust isn't as approachable for many JavaScript developers. At the same time, exposing a plugin system in JavaScript could negate performance gains. A definitive solution hasn't emerged yet. Ideally, the future combines both JavaScript and Rust. If you want to write a plugin with JavaScript, it’s possible with a tradeoff for speed. **Need more performance? Use the Rust plugin API**.
- **Bundling**: One interesting area of development is `swcpack`, which is SWC's replacement for Webpack. It's still under development but could be very promising.
- **WebAssembly**: As mentioned above, the prospect of writing Rust and compiling to WASM is enticing, but there's still work to be done.

## Reference

- [links](https://leerob.io/blog/rust)
