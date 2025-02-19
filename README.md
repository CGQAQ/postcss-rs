# postcss-rs

🚀 Fast and 100% API compatible postcss replacer, built in Rust

> ⚠️ DO NOT USE. STILL WORK IN PROGRESS.

## Performance Improvement 

Tokenize [bootstrap.css](./assets/bootstrap.css) (Compare with Node.js v16.13.0):

```bash
js:   0.11s user 0.02s system 126% cpu 0.102 total
rust: 0.00s user 0.00s system  66% cpu 0.006 total

# tokenize bootstrap-reboot.css               ~45x
js:   tokenizer/small(7K)                  3.063ms
rust: tokenizer/small(7K)                  0.068ms

# tokenize bootstrap.css                      ~26x
js:   tokenizer/fairly_large(201K)        25.672ms
rust: tokenizer/fairly_large(201K)         0.979ms
```

🎉 Welcome to contribute, here is a guide:

```bash
git checkout main
cargo bench -- --save-baseline main
```

Create a baseline, Then do some optimization work here.

```bash
git checkout feature  ## `feature` is your branch 
```

Compare against the `main` baseline.

```bash
cargo bench -- --baseline main
```

## License

[postcss-rs](https://github.com/justjavac/postcss-rs) is released under the
MIT License. See the [LICENSE](./LICENSE) file in the project root directory for details.

## ACKNOWLEDGEMENT

**The project is based on the wonderful work of Andrey Sitnik([@ai](https://github.com/ai)) 's [postcss](https://github.com/postcss/postcss), which is under MIT License(See [HERE](./POSTCSS.LICENSE)), Great thanks to Mr. Sitnik and other contributors' work.**
