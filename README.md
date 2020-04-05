# termecho

Send commands and write data to terminals (`/dev/tty` and `/dev/pts` devices).


## Examples

`cd` multiple terminals at once when switching to a different project:

```sh
$ sudo termecho /dev/pts/2 /dev/pts/3 -- cd ~/path/to/my/project
```

Write text to a `vim` buffer open on another terminal:

```sh
$ sudo termecho -n /dev/pts/2 -- Hello from $(tty)
```

Close `vim` on another terminal:

```sh
$ sudo termecho /dev/pts/2 -- :wq
```

## Security

There are [known security risks](https://undeadly.org/cgi?action=article;sid=20170701132619)
with the `TIOCSTI` ioctl syscall and caution is advised. This is intended for
**personal use** and not for automated production use cases. Please do not
run arbitrary commands through `termecho` without inspection (this is generally
good practice anyway).

## Building

`termecho` is written in Rust, so a [Rust installation](https://www.rust-lang.org/)
is required for compilation.

```sh
$ git clone https://github.com/SteadBytes/termecho
$ cd termecho
$ cargo build --release
$ ./target/release/termecho --version
termecho 0.1.0
```

## Compatibility

`termecho` uses the [`TIOCSTI` ioctl](http://man7.org/linux/man-pages/man4/tty_ioctl.4.html)
and as such is only available on platforms that support this syscall.


