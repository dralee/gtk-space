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

#### UI资源
为了实现UI/代码分离，需要创建window.ui并创建resources.gresource.xml指定需要包含的ui文件
* 需要添加资源编译工具依赖 glib-build-tools
```bash
cargo add glib-build-tools --build
```
* 根目录创建build.rs
```rust
fn main(){
    glib_build_tools::compile_resources(&["resources"], 
        "resources/resources.gresource.xml", 
        "composite_template1.gresource",);
}
```
* 使用宏注册并包含资源：gio::resources_register_include!

### css样式
运行中，可通过ctrl+shift+D 查看GtkInspector
#### gtk内置颜色可通过 @name形式进行引用
https://gitlab.gnome.org/GNOME/gtk/-/blob/b2c227e9c57839a2a4e24462a71ae0bad9a95264/gtk/theme/Default/_colors-public.scss
```css
task-row {
    background-color: @success_color;
  }
```
gtk支持的样式属性：
https://docs.gtk.org/gtk4/css-properties.html#gtk-css-properties


### 编译
```bash
cargo build -r
cargo build -r --target=x86_64-unknown-linux-musl
```
#### Libadwaita
是一个增强GTK4的库：
* 提供小部件以更好地遵循GNOME的HIG
* 提供小组件，让应用程序根据可用空间[更改其布局](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/adaptive-layouts.html)
* 集成了[Adwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/adaptive-layouts.html)样式表
* 允许使用[命名颜色](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/named-colors.html)进行时重新着色
* 添加[API](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.StyleManager.html)以以支持跨桌面深色样式首选项
```bash
cargo add libadwaita --rename adw --features v1_5
```
#### linux操作系统
* Fedora及其衍生产品
```bash
sudo dnf install libadwaita-devel
```
* Debian及其衍生产品
```bash
sudo apt install libadwaita-1-dev
```
* Arch及其衍生产品
```bash
sudo pacman -S libadwaita
```
#### macOS
```bash
brew install libadwaita
```
#### windows
* 如果使用gvsbuild
```bash
gvsbuild build libadwaita librsvg
```
* 如果使用msvc手动生成
在Windows的“开始”菜单中，搜索“x64 Native Tools Command Prompt for VS 2022”，
这将打开配置为使用MSVC x64工具终端。从那里运行以下命名：
```bash
cd /
git clone --branch libadwaita-1-3 https://gitlab.gnome.org/GNOME/libadwaita.git --depth 1
cd libadwaita
meson setup builddir -Dprefix=C:/gnome -Dintrospection=disabled -Dvapi=false
meson install -C builddir
```
##### 解决缺少图标问题
GTK<4.10碕此解决方法
* gsbuild
在命名提示符下：
```bash
xcopy /s /i C:\gtk-build\gtk\x64\release\share\icons\hicolor\scalable\apps C:\gtk-build\gtk\x64\release\share\icons\hicolor\scalable\actions
gtk4-update-icon-cache.exe -t -f C:\gtk-build\gtk\x64\release\share\icons\hicolor
```
MSVC使用手动操作
```bash
xcopy /s /i C:\gnome\share\icons\hicolor\scalable\apps C:\gnome\share\icons\hicolor\scalable\actions
gtk4-update-icon-cache.exe -t -f C:\gnome\share\icons\hicolor
```
##### 使用方式
* 最简单使用方式是将gtk::Application替换为adw::Application

adw::ApplicationWindow没有标题栏区域，当使用adw::NavigationSplitView，在左侧NavigationSplitView添加了集合视图的侧边栏
而任务视图则占据了右侧空间。使用adw::ApplicationWindow时，集合视图和任务视图有自己视图adw::HeadBar分隔符跨越整个窗口。


