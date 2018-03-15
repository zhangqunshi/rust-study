第一章: HelloWorld
=====

首先下载rust：https://www.rust-lang.org/zh-CN/

我下载的是windows版本，由于rust不自带连接器，所以需要安装Microsoft Visual C++ Build Tools。

我安装了Visual Studio 2017，在通用平台中勾选了"C++ 通用 Windows 平台工具"这个选项。（我猜的）

然后安装Rust的时候，rust会自己找这个连接器。
```
Welcome to Rust!

This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin
directory, located at:

  C:\Users\kris\.cargo\bin

This path will then be added to your PATH environment variable by modifying the
HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and these changes will
be reverted.

Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```
直接回车, 安装时间可能会比较长。


新建一个hello.rs的文件，内容如下：
```
fn main() {
    println!("Hello, world!");
}
```

编译运行：
```
rustc hello.rs
./hello
Hello, world!
```

注意：如果出现error: no default toolchain configured错误，就运行
```
rustup install stable
rustup default stable
```
基本都是网络断了导致的。


Rust下载安装的速度会让人发疯~~~，太慢了。还不如把所有东西都放在一个安装包里面。
