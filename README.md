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

## Why Tasky CLI?

In a world of bloated productivity apps, **Tasky CLI** stays out of your way.

- **Speed:** Add and check tasks in seconds without leaving your terminal.
- **Context Preservation:** If you're already coding, don't break your flow by switching to a web browser or GUI app.
- **Privacy & Portability:** Your tasks are stored in a simple `tasks.json` file on your machine. No cloud, no tracking.
- **Visual Clarity:** Uses high-contrast colors to help you distinguish between pending, completed, and overdue tasks at a glance.

## When to use it?

- **Daily Standups:** Quickly list what you finished yesterday and what's on for today.
- **Feature Checklists:** Track small sub-tasks during a complex refactor.
- **Learning Rust:** A perfect project to read and understand how Rust handles CLI arguments and file I/O.
- **Quick Reminders:** "Pay bills", "Commit changes", or "Buy coffee" — captured in 5 seconds.

## For the Non-CLI Developer

You don't need to be a "terminal wizard" to use Tasky. If you usually avoid the command line, Tasky is the perfect "gateway tool":

- **Human Readable:** Commands are simple English (`add`, `list`, `done`).
- **Zero Configuration:** Install it, and it just works. No config files to mess with initially.
- **Bridge the Gap:** It helps you get comfortable with the terminal in a low-stakes, highly productive way.
- **Scriptable:** Once you're ready, you can easily use it in your own automation scripts or CI/CD pipelines.

---

## Features

- **Add tasks:**

  ```bash
  tasky-cli add "Buy milk"
  # With due date (YYYY-MM-DD)
  tasky-cli add "Submit report" "2026-01-15"
  ```

- **List tasks:** (colorized output, overdue tasks in red)

  ```bash
  tasky-cli list
  ```

- **Edit tasks:**

  ```bash
  tasky-cli edit 0 "Buy oat milk"
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

# Add a task with a due date
tasky-cli add "Pay bills" "2026-02-01"

# List all tasks
tasky-cli list

# Edit a task
tasky-cli edit 0 "Finish math homework"

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
