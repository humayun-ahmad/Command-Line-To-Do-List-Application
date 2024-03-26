# Command Line To Do list App

1. If build error is : build error: Rust WASM toolchain not installed, please install it!

Solution:

```
rustup target add wasm32-unknown-unknown
```


```
cargo update
```

```
cargo clean
```

```
cargo build
```

---------------------------------------------------------------
If error: error[E0635]: unknown feature `stdsimd`

Solution: 

You may have 2 versions of ahash in your Cargo.lock file. This can be confirmed by running $ cargo tree -i ahash. In my case, I had 0.7.x & 0.8.x. So, it worked following this steps below:

Remove your Cargo.lock file or you can just remove either of the versions (preferably older).
Clean the target/ folder using $ cargo clean.
Build again $ cargo build.
It will work.
---------------------------------------------------------------# Command-Line-To-Do-List-Application
