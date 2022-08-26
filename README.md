# rusting

## 


```shell

# 升级 rust
rustup update

# 修改 Cargo.toml 后运行，更新库依赖
cargo update

```



## first_impression

```shell
# 创建
cargo new first_impression

# 编译
cargo build -p first_impression --release

./target/release/first_impression
```

## httpie
```shell

cargo build -p httpie --release

cargo run -p httpie -- get https://postman-echo.com/get?foo1=bar1&foo2=bar2

# get
./target/release/httpie get https://postman-echo.com/get?foo1=bar1&foo2=bar2

# post
./target/release/httpie post https://postman-echo.com/post a=1 b=2
```

## scrape_url

### run
```shell
# 在 项目根目录 执行
# -p 指定要运行的 create
cargo run -p scrape_url -- https://blog.rust-lang.org/  ./a.md

cargo build -p scrape_url --release

./target/release/scrape_url -- https://blog.rust-lang.org/  ./a.md
```


## wow_aes

```shell
# 创建
cargo new wow_aes

# 编译
cargo build -p wow_aes --release

./target/release/wow_aes
```
