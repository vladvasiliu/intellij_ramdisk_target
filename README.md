# intellij_ramdisk_target

Automatically sets up build directory links for Intellij editors when using ramdrives.

## Status
This project is made for personal use and to get used to Rust. You may use this project at your own risk.

Caution is advised, as it may kill your cat or have some other surprising behaviour.

## How to use

This is written is Rust. As such you may download a binary from the releases page (Linux/x86_64 only) or build it yourself:

```
cargo build --release
```

## Requirements

I'm only testing it using the latest stable Rust and on Linux/x86_64.
There is actually a tight dependency on unix, so it won't work on Windows. YMMV on Mac OS.

## Motivation
I'm sometimes using a computer with an old SSD to develop Rust projects.

In order to avoid excessive wear through write amplification, I'm putting the `target` folder on a ramdrive.

This project was born out of a need to automate creation and symlinking of the required directories while using IntelliJ.