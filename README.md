# Exact Nearest Neighbor

This repo implements exact nearest neighbor search on the [sift datasets](http://corpus-texmex.irisa.fr/).
It requires rust-nightly, because it uses the [portable-simd](https://github.com/rust-lang/portable-simd) crate.
The code can be run natively on "Apple Silicon" as well.

It matches the speed of [Faiss](https://github.com/facebookresearch/faiss)' IndexFlatL2 when running in single element mode.
Note that Faiss uses hand-rolled SIMD instructions, while this repository keeps that part much simpler (see the l2 function in [main.rs](src/main.rs)).

For optimized running:
```
RUSTFLAGS="-C target-cpu=native" cargo run --release
```

For downloading the datasets:
```
mkdir data
curl ftp://ftp.irisa.fr/local/texmex/corpus/siftsmall.tar.gz | tar -xzC data
curl ftp://ftp.irisa.fr/local/texmex/corpus/sift.tar.gz | tar -xzC data
```
