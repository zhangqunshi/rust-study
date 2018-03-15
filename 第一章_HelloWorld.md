第一章: HelloWorld
=====

首先下载rust：https://www.rust-lang.org/zh-CN/

我下载的是windows版本，由于rust不自带连接器，所以需要安装Microsoft Visual C++ Build Tools。

我安装了Visual Studio 2017，在通用平台中勾选了"C++ 通用 Windows 平台工具"这个选项。（我猜的）

然后安装Rust的时候，rust会自己找这个连接器。

新建一个hello.rs的文件，内容如下：
```
fn main() {
    println!("Hello, world!");
}
```

编译运行：
```
rustc main.rs
./main
Hello, world!
```
