<!--
 * @Date: 2024-03-06 14:47:12
 * @LastEditTime: 2024-03-19 15:17:56
 * @Description: 
-->
# About

## 程序说明

该Rust工程文件为华中科技大学网络空间安全学院2024年春程序设计课程的课程设计成果，源代码编译后可以得到目录扫描命令行程序，可以通过提供不同参数开启不同功能选项，例如扫描时在内存中建立目录树，将扫描结果序列化到.yaml文件中保存，将扫描结果以sqlite3数据库的方式进行保存。扫描程序是多线程的，主线程会根据获取到的命令行参数决定启动多少线程，扫描线程会根据命令行参数决定将扫描到的文件信息发给哪些线程，程序的IO阻塞非常小，开启一个或多个与不开启扫描以外的附加功能在扫描耗时上只有非常小的差异.

[配置Rust环境](https://course.rs/first-try/installation.html)
[Github](https://github.com/RuijieWu/Rust_Develop/tree/main/LearnRustByPracticeAndExample/file_scanner)

编译file_scanner

``` Bash
//! pwd -> ~/.../file_scanner
cargo build
mv target/debug/file_scanner.exe ./

//直接编译并运行程序
cargo run
cargo run -- [arguments]
```

## 使用方式

### 获取帮助

``` bash
./file_scanner --help
```

### 扫描指定路径

``` bash
./file_scanner target_path [-db] [-yaml] [-tree]
```

### 读取数据文件批量扫描

``` bash
./file_scanner -read
```

### 读取数据文件模拟操作

``` bash
./file_scanner -operation
```
