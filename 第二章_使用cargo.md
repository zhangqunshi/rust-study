第二章: 使用cargo
=====

cargo是rust的构建系统和包管理工具。类似npm命令或者make命令等等。主要负责：
- 构建你的代码
- 下载你代码依赖的库
- 编译这些库

安装rust之后，就可以直接使用cargo命令了。测试cargo是否已经安装:
```
cargo --version
```

## 使用cargo创建helloworld项目
运行命令：
```
cargo new hello_world --bin
```
其中hello_world是项目名称，这个命令执行后会创建一些模板代码。（--bin是指项目为可执行代码，不是库代码）

模板代码主要是src目录，里面有一个main.rs文件，还有一个Cargo.toml文件（注意：C大写），用于配置项目信息以及依赖外部的库。

进入hello_world目录后，通过命令`cargo build`就可以编译代码，编译好的代码生成在target目录下。

还有一个命令`cargo run`可以编译并且直接运行代码。

生成出来的Cargo.lock文件用于跟踪你程序的依赖, Cargo.lock你自己不需要碰这个文件。

