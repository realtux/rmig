# rmig 0.0.1 by [tux](https://github.com/realtux)

rmig is a re-implementation (and re-imagination) of an old c project in rust. this project aims to be
a generic database migrations manager which will support several platforms, but mysql at first.

---

### installation

in general, see releases to download the binary for your platform. if you want to compile on your own,
make sure you're using at least `1.41.0` of the rust toolchain.

```
git clone https://github.com/realtux/rmig
cd rmig
cargo build --release
```

then do something with `target/release/rmig`.

### license

rmig is available under the MIT License
