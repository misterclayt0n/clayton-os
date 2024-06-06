# simple operating system built in rust
### first steps
#### add a bare metal enviroment target triple

```zsh
rustup target add thumbv7em-none-eabihf
```

#### compile 
compile without a target operating system

```zsh
cargo build --target thumbv7em-none-eabihf
```

#### roadmap
- [x] build a minimal freestanding rust binary
- [ ] build a minimal freestanding rust binary into a minimal operating system kernel
