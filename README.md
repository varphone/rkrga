rkrga
=====

Rockchip RGA 库的 Rust 语言绑定.

Rockchip RGA 是一个独立的二维光栅图形加速单元。
它加速了二维图形操作，例如点/线绘制、图像缩放、旋转、位图、图像合成等。

当前基于 Rockchip RGA v1.2.x 应用接口接口实现。

本项目仅适用于 Rockchip 公司具有 RGA 模块的 SOC 平台，例如 RK33XX、RK35XX 等。

准备工作
--------

在引用或编译本项目前，你需要将 Rust 工具链更新到 `1.57` 以上的版本。

同时确认 Target: `aarch64-unknown-linux-gnu` 已经安装，若是没有可以执行以下命令安装：

```sh
rustup target add aarch64-unknown-linux-gnu
```

由于本项目的目标平台为 ARM64 架构，因此在开始是基本使用交叉编译模式，
在 x86/x86_64 平台上开发、编译，然后讲可执行文件上传到目标设备执行，
因此在交叉编译环境下编译本项目前你需要设置以下环境变量：

```sh
# 设置包含有 RgaApi.h 的目录
export RKRGA_INCLUDE_DIR=/opt/fullv/2021.02.7-rklaser1/staging/usr/include/rga
# 设置交叉编译工具链 SYSROOT 目录
export RKRGA_SYSROOT_DIR=/opt/fullv/2021.02.7-rklaser1/staging
```

另外还需要设定 Cargo 交叉编译关配置，创建 `.cargo/config.toml` 文件并写入类似于以下内容：

```toml
[build]
target = "aarch64-unknown-linux-gnu"

[target.aarch64-unknown-linux-gnu]
linker = "/opt/fullv/2021.02.7-rklaser1/host/bin/aarch64-buildroot-linux-gnu-gcc"
runner = "./target-runner.sh"
```

> 注意：以上路径仅为作者自身项目所用，你需要调整为合适自己的路径。

当以上配置都妥当后，现在可以安照 Rust 标准开发流程来进行后续工作了。

使用示例
--------

```rust
use rkrga::{Rga, RgaBuffer, RgaInfoBuilder, RgaRectBuilder, RgaTransform};
use std::sync::Arc;

// 初始化 RGA 模块
let rga = Arc::new(Rga::new().unwrap());

// 设定输入、输出图像信息
let src_rect = RgaRectBuilder::new().size(1280, 720).format(RgaPixelFormat::Rgba8888).build();
let dst_rect = RgaRectBuilder::new().size(720, 1280).format(RgaPixelFormat::Rgba8888).build();

// 分配输入、输出内存缓冲区
let src_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &src_rect).unwrap();
let dst_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &dst_rect).unwrap();

// 生成 RGA 操作信息
let src_info = RgaInfoBuilder::new().bo(&src_bo).rect(&src_rect).build();
let mut dst_info = RgaInfoBuilder::new().bo(&dst_bo).rect(&dst_rect).build();

// 执行旋转操作
rga.rotate(&src_info, &mut dst_info, RgaTransform::Rot90).unwrap();
```
