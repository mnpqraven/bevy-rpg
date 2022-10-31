# bevy-rpg

[figma sketch](https://www.figma.com/file/AooauSNSNEOETl55xjjVr2/struct?node-id=1%3A4900)

#### Local build

- add this to your `.cargo/config.toml`

```
[target.x86_64-pc-windows-msvc]
rustflags = ["-Zshare-generics=n"]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```
