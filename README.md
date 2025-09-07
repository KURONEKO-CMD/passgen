# passgen

A simple interactive command-line password generator written in Rust.

一个简单的交互式命令行密码生成器，使用 Rust 编写。

## English

### Features

- Customizable password length.
- Optionally include uppercase letters.
- Optionally include lowercase letters.
- Optionally include digits.
- Optionally include special symbols (`!@#$%^&*()_+-=[]{}|;:,.<>?`).

### Usage

1. Clone or download this repository.
2. Change into the project directory:
   ```bash
   cd passgen
   ```
3. Run the program. `cargo` will handle dependencies, compile, and run:
   ```bash
   cargo run
   ```
   Follow the interactive prompts to set your preferences.

### Build

If you want to build a release binary manually:

```bash
cargo build --release
```

The executable will be located at `target/release/passgen`.

## 中文

### 功能

- 自定义密码长度。
- 可选择是否包含大写字母。
- 可选择是否包含小写字母。
- 可选择是否包含数字。
- 可选择是否包含特殊符号（`!@#$%^&*()_+-=[]{}|;:,.<>?`）。

### 如何使用

1. 克隆或下载项目。
2. 进入项目目录：
   ```bash
   cd passgen
   ```
3. 运行项目。`cargo` 将会自动处理依赖、编译和运行：
   ```bash
   cargo run
   ```
   程序启动后，根据命令行提示输入您的要求即可。

### 如何构建

如果您想手动构建可执行文件：

```bash
cargo build --release
```

可执行文件将位于 `target/release/passgen`。
