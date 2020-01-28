# intellij_ramdisk_target

Automatically sets up build directory links for Intellij editors when using ramdrives.

## Status
This project is made for personal use and to get used to Rust. You may use this project at your own risk.

Caution is advised, as it may kill your cat or have some other surprising behaviour.

## How to use

This is written is Rust. As such you may download a binary from the releases page (Linux/x86_64 only) or build it yourself.

The program expects two arguments:
1. the full path of the directory on the ramdrive (the link destination)
1. the full path of the directory to be linked (the "build" directory, aka the link source)

### Example

IntelliJ currently has a limitation that doesn't allow creating run configurations from arbitrary binaries.
To circumvent this, create a shell script that wraps the binary:

```bash
#!/usr/bin/env bash

/path/to/intellij_ramdisk_target $@
```

This script passes all its arguments to the wrapper binary.

* In the IDE go to *Settings -> Tools -> Startup Tasks*
* Click *Add new configuration -> Shell script*
* Add the path to the shell wrapper
* Add the arguments on the format below:

```bash
intellij_ramdisk_target --projdir ~/code/some_project/target --ramdir /run/user/$UID/code/some_project/target
```


## Requirements

I'm only testing it using the latest stable Rust and on Linux/x86_64.
There is actually a tight dependency on unix, so it won't work on Windows. YMMV on Mac OS.

## Motivation
I'm sometimes using a computer with an old SSD to develop Rust projects.

In order to avoid excessive wear through write amplification, I'm putting the `target` folder on a ramdrive.

This project was born out of a need to automate creation and symlinking of the required directories while using IntelliJ.

## License
The code is released under the termes of the BSD 3-clause License.
Please see [`LICENSE`](LICENSE) for the full text of the license.
