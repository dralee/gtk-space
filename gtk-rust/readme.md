### 如何确定在glib主循环上生成async任务
当glib调用函数来自于以下任一库时，应该能生成任务task
* 来自glib生态系统
* 不要依赖于运行时，而只依赖于crates(futures-io, futures-core等)系列
* 取决于async-std或smol运行时
* 具有cargo功能，使它们依赖于async-std/smol而不是tokio

#### 如果任务IO受限，则取决于可使用的crates和要完成的工作类型
* 轻量级I/O可使用能在主循环上生成线程的glib、smol、async-std或futures特征系列。避免通过同步通道
* 繁重I/O工作可能仍受益于在单独线程/异步执行器中运行，以便主循环饱和。如不确定，建议进行基准测试。
如果最适合该工作的crates依赖于tokio,则必须使用tokio运行时生成它并通过通道进行通信。

### 配置
为了使用配置，需要创建org.tk_rs.Setting1.gschemal.xml（即APP_ID.gschemal.xml）
```xml
<?xml version="1.0" encoding="utf-8"?>
<schemalist>
  <schema id="org.gtk_rs.Settings1" path="/org/gtk_rs/Settings1/">
    <key name="is-switch-enabled" type="b">
      <default>false</default>
      <summary>Default switch state</summary>
    </key>
  </schema>
</schemalist>
```
需要将配置存放到$HOME/.local/share/glib-2.0/schemas下，并编译
```bash
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp org.gtk_rs.Settings1.gschema.xml $HOME/.local/share/glib-2.0/schemas/
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
```