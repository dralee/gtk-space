### tokio 异步平台
reqwest是该平台下一个组件
```bash
cargo add reqwest@0.12 --features rustls-tls --no-default-features
```
* add the tokio by supporting the reqwest
```bash 
cargo add tokio@1 --features rt-multi-thread 
```
