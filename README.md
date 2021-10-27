# rusting



## scrape_url

### run
```shell
# 在 项目根目录 执行
# -p 指定要运行的 create
cargo run -p scrape_url -- https://blog.rust-lang.org/  ./a.md
```


## httpie
```shell
cargo build -p httpie --release

# get
./target/release/httpie get https://postman-echo.com/get?foo1=bar1&foo2=bar2

# post
./target/release/httpie post https://postman-echo.com/post a=1 b=2
```
