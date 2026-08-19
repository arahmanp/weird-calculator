<div align="center">

# 🧮 Weird Calculator

**An interactive CLI calculator built with Rust — learning by tinkering.**

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/platform-Windows%20x86--64-blue)
![Status](https://img.shields.io/badge/status-active%20learning%20project-yellow)
![Version](https://img.shields.io/badge/version-v0.1-orange)

</div>

---

## 📖 About

**Weird Calculator** is a terminal-based (CLI) calculator program built as a hands-on way to sharpen **Rust** skills. The project is intentionally simple in scope, but still pays attention to terminal UX — colored output, an ASCII art logo with a gradient effect, and clean error handling for every command.

> ⚠️ **Note:** This is a **learning project**. The code will keep evolving as my understanding and skill in Rust grow. Don't be surprised if some parts aren't "best practice" yet — that's the whole point, learning as I go. 😄

---

## ✨ Features

- 🎨 ASCII art logo with a TrueColor gradient effect
- 🖥️ Interactive REPL (Read-Eval-Print Loop) interface
- ➕➖✖️➗ Basic arithmetic operations with 10-decimal precision
- 🧹 Command to clear the terminal screen
- 🚨 Argument validation with informative error handling
- 🌈 Colored output powered by the [`colored`](https://crates.io/crates/colored) crate

---

## 📋 Command List

| Command | Arguments | Description |
|---------|-----------|--------------|
| `HELP` | — | Shows all available commands |
| `CLEAR` | — | Clears the terminal screen |
| `ADD` | `A B` | Displays the sum of A and B |
| `SUBTR` | `A B` | Displays the subtraction of A and B |
| `MULTI` | `A B` | Displays the multiplication of A and B |
| `DIV` | `A B` | Displays the division of A and B |
| `EXIT` | — | Exits the program |

> `A` and `B` can be integers or decimals (parsed as `f64`).

---

## 🚀 Installation

### Windows (via Installer)

1. Download the latest installer from the [**Releases**](../../releases) page — built for the **x86-64** architecture using **Inno Setup**.
2. Run the `.exe` installer.
3. Follow the usual installation steps.
4. Launch **Weird Calculator** from the Start Menu or the shortcut created by the installer.

### Build from Source (Cross-platform)

Make sure you have [Rust & Cargo](https://www.rust-lang.org/tools/install) installed, then run:

```bash
# Clone the repository
git clone https://github.com/arahmanp/weird-calculator.git
cd weird-calculator

# Build the release version
cargo build --release

# Run it
./target/release/weird-calculator
```

---

## 💻 Usage Example

```text
> HELP
 Available commands:

    HELP            Showing all available commands
    CLEAR           Clearing the terminal screen
    ADD A B         Displaying the sum of two numbers A and B
    SUBTR A B       Displaying the subtraction of two numbers A and B
    MULTI A B       Displaying the multiplication of two numbers A and B
    DIV A B         Displaying the division of two numbers A and B
    EXIT            Exit the program

> ADD 12 8.5
20.5000000000

> DIV 10 3
3.3333333333

> EXIT
```

---

## 🛠️ Tech Stack

| Component | Description |
|-----------|--------------|
| **Language** | Rust 🦀 |
| **`colored` crate** | Terminal text styling & coloring |
| **`clearscreen` crate** | Cross-platform terminal screen clearing |
| **Installer** | Inno Setup (Windows x86-64) |

---

## 🗺️ Roadmap

Some ideas for future development as Rust skills keep growing:

- [ ] Support for advanced operations (modulus, power, square root, etc.)
- [ ] Custom environment configuration
- [ ] Unit tests for each calculation function
- [ ] Log file

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

## 👤 Author

**Andhika Rahman**
GitHub: [@arahmanp](https://github.com/arahmanp)

<div align="center">

*Built with 🦀 and curiosity*

</div>