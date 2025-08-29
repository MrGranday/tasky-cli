# Tasky CLI

A simple Rust CLI to manage to-do lists.

---

## About

**Tasky CLI** is a lightweight command-line tool written in Rust for managing to-do lists.
It supports adding, listing, removing, and marking tasks as done, with colorful output and JSON storage.

Designed for beginners, Tasky CLI is ideal for:

- Learning Rust
- Contributing to open source
- Organizing tasks on Windows (CMD/PowerShell), Linux, or macOS

See `tasky.txt` for a detailed project overview.

---

## Features

- **Add tasks:**

  ```bash
  tasky-cli add "Buy milk"
  ```

- **List tasks:** (colorized output)

  ```bash
  tasky-cli list
  ```

- **Remove tasks:**

  ```bash
  tasky-cli remove 0
  ```

- **Mark tasks as done:**

  ```bash
  tasky-cli done 0
  ```

- **JSON storage:** Tasks stored in `tasks.json`
- **Cross-platform:** Works on CMD, PowerShell, Linux, macOS

**Planned features:**

- Due dates
- Priority tags
- JSON output enhancements

---

## Installation

### 1. Install Rust

Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install) or run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- On **Windows**, follow the `rustup-init.exe` prompts.

### 2. Install Tasky CLI via Cargo

```bash
cargo install tasky-cli
```

### 3. Or build from source

```bash
git clone https://github.com/MrGranday/tasky.git
cd tasky
cargo build
cargo run -- add "Test task"
cargo run -- list
```

### 4. Windows CMD color support

```bash
reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1
```

---

## Usage

```bash
# Add a new task
tasky-cli add "Finish homework"

# List all tasks
tasky-cli list

# Mark a task as done
tasky-cli done 0

# Remove a task
tasky-cli remove 0
```

---

## Contributing

Tasky CLI welcomes beginners! We aim for 90%+ beginner contributors.

**Steps to get started:**

1. Read [`CONTRIBUTING.md`](CONTRIBUTING.md) for guidance.
2. Check issues for **good first issue tasks**, such as:

   - Adding a `--done` command
   - Supporting due dates
   - Improving documentation

3. Open a pull request — no experience needed! We’ll guide you.

---

## License

Tasky CLI is licensed under the **MIT License**.

---

## Community

- **Questions:** [GitHub Issues](https://github.com/MrGranday/tasky/issues)
- **Join discussions:** Rust Discord, r/rust
- **Share your progress:** Use `#rustlang` or `#tasky` on X (Twitter)

Happy task managing! 🦀
