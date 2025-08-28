# Contributing to Tasky

Thank you for your interest in contributing to **Tasky**, a simple Rust CLI for managing to-do lists! We aim to make this project welcoming to beginners, especially those new to Rust or open-source. Whether you're fixing a typo, adding a feature, or improving documentation, every contribution counts. This guide will help you get started.

## Table of Contents

- [Why Contribute?](#why-contribute)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Good First Issues](#good-first-issues)
- [Code Style and Guidelines](#code-style-and-guidelines)
- [Community and Support](#community-and-support)
- [License](#license)

## Why Contribute?

Tasky is designed to be a beginner-friendly project,

- **Learn Rust**: Practice structs, file I/O, CLI parsing, and JSON handling in a small codebase.
- **Build Your Portfolio**: Your contributions (even small ones!) will be visible on GitHub.
- **Join a Welcoming Community**: We aim for 90%+ beginner contributors, so you’re in good company.
- **Make an Impact**: Add features or fixes to a tool people can use in CMD, PowerShell, or any terminal.

No prior open-source experience? No problem! We’ll guide you through your first pull request (PR).

## Getting Started

1. **Install Rust**:
   - Download and install Rust via [rust-lang.org](https://www.rust-lang.org/):curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

On Windows, this runs `rustup-init.exe`. Choose the default `stable` toolchain.

- Verify: `rustc --version` (should show ~1.70.0 or later).

2. **Set Up an Editor**:

- We recommend [VS Code](https://code.visualstudio.com/) with the `rust-analyzer` extension for code completion and error checking.

3. **Clone the Repository**:

- git clone https://github.com/MrGranday/tasky.git
- cd tasky

4. **Build and Test**:

- Build: `cargo build`
- Run: `cargo run -- add "Test task"`
- Test commands: `cargo run -- list` or `cargo run -- remove 0`
- On Windows, ensure CMD supports colors: `reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1`

## How to Contribute

Follow these steps to submit your first contribution:

1. **Find an Issue**:

- Check the [Issues](https://github.com/MrGranday/tasky/issues) tab on GitHub.
- Look for issues labeled `good first issue` or `help wanted`—perfect for beginners!
- If you have an idea, open a new issue to discuss it first.

2. **Fork the Repo**:

- Click the “Fork” button on GitHub to create your own copy.
- Clone your fork: `git clone https://github.com/yourusername/tasky.git`

3. **Create a Branch**:

- `git checkout -b my-fix-or-feature`
- Use a descriptive name (e.g., `add-due-dates`).

4. **Make Changes**:

- Edit files in `src/` (e.g., `main.rs` for code, `README.md` for docs).
- Keep changes small and focused (e.g., one feature or bug fix per PR).
- Test: `cargo run -- <your-command>` and `cargo test` (if tests exist).

5. **Commit and Push**:

- Commit: `git commit -m "Add due date support"`
- Push: `git push origin my-fix-or-feature`

6. **Submit a Pull Request**:

- Go to your fork on GitHub and click “Compare & pull request.”
- Describe your changes clearly (e.g., “Added `--done` command to mark tasks as complete”).
- Link to the issue you’re addressing (e.g., “Fixes #5”).

7. **Respond to Feedback**:

- Maintainers may suggest changes. Update your branch and push again.
- Don’t worry—we’re here to help beginners!

**Tips**:

- Run `cargo check` or `cargo clippy` to catch errors early.
- Keep PRs small to make reviews faster.
- Ask questions in the issue or PR comments if stuck.

## Good First Issues

Here are examples of beginner-friendly tasks to start with:

- **Documentation**:
- Fix typos in `README.md` or this file.
- Add usage examples (e.g., for PowerShell).
- **Features**:
- Add a `--done` command to mark tasks as complete (e.g., `tasky done 0`).
- Support due dates (e.g., `tasky add "Buy milk" --due "2025-08-30"`).
- Add priority tags (e.g., `tasky add "Call mom" --priority high`).
- Add a `clear` command to delete all tasks.
- **Output Improvements**:
- Add JSON output for PowerShell (`tasky list --json`).
- Colorize tasks by priority (e.g., red for high).
- **Bug Fixes**:
- Improve error messages (e.g., for invalid indices).
- Handle file permission errors for `tasks.json`.
- **Tests**:
- Write unit tests for new commands using `#[test]`.

Check the [Issues](https://github.com/MrGranday/tasky/issues) tab for specific tasks. Comment on an issue to claim it (e.g., “I’d like to work on this!”).

## Code Style and Guidelines

- **Rust Formatting**: Run `cargo fmt` to auto-format code (follows Rust’s standard style).
- **Linting**: Use `cargo clippy` to catch common issues.
- **Commit Messages**: Use clear messages (e.g., “Add `--done` command” or “Fix typo in README”).
- **Keep It Simple**: Tasky is meant to be minimal. Avoid complex dependencies or features unless discussed.
- **Testing**: Test your changes in CMD and PowerShell. Ensure `tasks.json` is readable/writable.
- **Windows Focus**: Ensure features work in CMD (`tasky list`) and PowerShell (`tasky list | ConvertFrom-Json` for JSON).

## Community and Support

We’re here to help! If you’re new to Rust or open source:

- **Ask Questions**: Comment on GitHub issues or PRs.
- **Join the Rust Community**:
- [Rust Discord](https://discord.gg/rust-lang) (beginner channels).
- [r/rust on Reddit](https://www.reddit.com/r/rust/).
- Share your progress on X with #rustlang or #tasky!
- **Contact Maintainers**: Reach out via GitHub issues for guidance.

No question is too small—beginners are our priority!

## License

By contributing to Tasky, you agree to license your contributions under the [MIT License](LICENSE). This keeps the project open and accessible for everyone.

---

Happy contributing! Let’s make Tasky a great tool together! 🦀
