Tasky
A simple Rust CLI to manage to-do lists.
About
Tasky is a lightweight command-line tool written in Rust for managing to-do lists. It supports adding, listing, and removing tasks with colorful output and JSON storage. Designed for beginners, Tasky is ideal for learning Rust, contributing to open source, and organizing tasks in Windows (CMD/PowerShell), Linux, or macOS.
See tasky.txt for a detailed project overview.
Features

Add tasks: tasky add "Buy milk"
List tasks: tasky list (colorized output)
Remove tasks: tasky remove 0
Stores tasks in tasks.json
Cross-platform: Works in CMD, PowerShell, Linux, macOS
Planned: Mark tasks as done, due dates, priority tags, JSON output

Installation

Install Rust: rust-lang.orgcurl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

On Windows, follow rustup-init.exe prompts.
Clone the repo:git clone https://github.com/MrGranday/tasky.git
cd tasky

Build and test:cargo build
cargo run -- add "Test task"
cargo run -- list

For Windows CMD colors:reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1

Contributing
Tasky welcomes beginners! We aim for 90%+ beginner contributors. To get started:

Read CONTRIBUTING.md for step-by-step guidance.
Check issues for good first issue tasks, like:
Adding a --done command
Supporting due dates
Improving docs

No experience? We’ll help you with your first pull request!

License
Tasky is licensed under the MIT License.
Community

Ask questions: GitHub Issues
Join: Rust Discord or r/rust
Share: Use #rustlang or #tasky on X

Happy task managing! 🦀
