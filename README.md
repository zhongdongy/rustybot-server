## Development

Windows 上面使用 `paho-mqtt` 可能会遇到 OpenSSL 依赖问题, 需要按照以下步骤解决:

1. 前往 <https://slproweb.com/products/Win32OpenSSL.html> 下载安装最新版本的 Win64 OpenSSL 安装包, 比如 [这个链接](https://slproweb.com/download/Win64OpenSSL-3_1_0.msi).
2. 安装后, 设置 `OPENSSL_DIR` 这个系统环境变量为: `C:\Program Files\OpenSSL-Win64`.
3. 前往 <https://cmake.org/download/> 下载安装最新版本的 CMake, 比如 [这个链接](https://github.com/Kitware/CMake/releases/download/v3.26.3/cmake-3.26.3-windows-x86_64.msi).
4. 安装 CMake 的过程中一定要勾选设置系统环境变量.

Linux (Ubuntu) 需要安装 `build-essential`, `libssl-dev`, `pkg-config`, `cmake` 之后才可以执行 `cargo build`.

## Run

```bash
RUST_BACKTRACE=1 RUST_LOG=info cargo run
```

```powershell
$env:RUST_LOG='debug'; $env:RUST_BACKTRACE=1; cargo run; $env:RUST_LOG='';
```

### Configuration

`auth.yml` is used when authenticating user requests.

```yaml
users:
- id: haoqiy
  key: 89d5xxxx-xxxx-xxxx-xxxx-xxxx7d405249
  note: "Da ge"
```