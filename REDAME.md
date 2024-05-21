

接口划分:
- 路由的注册更新删除 100%
  - 路由管理器
    - 普通路由检索
    - path参数路由检索
  - 路由分类
    - page路由
      - templage
    - api路由
  - method分类
  - 前缀树
- 静态资源 100%
- 实际路由转发接口 100%
  - 路由信息获取 v
  - 路由转发 v
  - 动态js脚本嵌入 v
  - 动态路由 v

更新计划:
- 功能实现 v
- 实现https v
- 基础0.0.2版本已实现 v
- other。。。待实现


打包相关:
暂时无法在win下直接使用`cargo build -r --target x86_64-unknown-linux-musl`, 进行交叉编译
这里提供两个方案: 
1. 直接到linux环境进行编译为linux二进制
2. 使用zig进行交叉编译，步骤如下
   1. 安装zig
      1. 安装Scoop
         1. 打开Windows Powershell
         2. 在PowerShell命令控制台执行: `iex (new-object net.webclient).downloadstring('https://get.scoop.sh')`
      2. 使用scoop安装zig: 在PowerShell命令控制台执行: `scoop install zig`
   2. 安装cargo-zigbuild
      1. `cargo install cargo-zigbuild`
   3. 使用zigbuild进行编译
      1. 注意需要调整`.cargo/config.toml`
         1. ```toml
         [target.x86_64-unknown-linux-musl]
         linker = "rust-lld"
         rustflags = ["-C", "linker-flavor=ld.lld"]
         ```
         改为
         ```toml
         [target.x86_64-unknown-linux-musl]
         ```
      2. 执行编译命令: `cargo zigbuild -r --target x86_64-unknown-linux-musl`
   4. 成功编译