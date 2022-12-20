# move pprint

<a href="https://crates.io/crates/move-pprint">
    <img src="https://img.shields.io/crates/v/move-pprint.svg" alt="Version">
</a>

With `debug::print(&b"move debug made easy")` in code.

Before 🤯

```
$ move test
...
[debug] 0x6d6f7665206465627567206d6164652065617379
```

After 😇

```
$ move test | move-pprint
...
[debug] move debug made easy
```

## Install

```
cargo install move-pprint
```
