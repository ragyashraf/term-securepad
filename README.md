
# 🛡️ term-securepad

A beginner-friendly, terminal-based notepad app built with Rust — featuring basic password protection, auto-saving, and a clean editing experience.

This was one of my first serious Rust projects. It helped me learn how to handle terminal input, file saving, and simple state management while exploring the power of Rust's safety and speed.

---

## ✨ Features

- ⌨️ Simple text editor in your terminal
- 🔒 Password protection on launch
- 💾 Auto-saves your work every 5 seconds
- 📂 Saves notes to a local file (`notes.txt`)
- 📍 Cursor movement & basic editing support

---

## 🧠 Why I Built It

> "I wanted to get hands-on with Rust by creating something useful. This project helped me understand file I/O, loops, conditionals, and how to structure a Rust app in a practical way."

---

## 🚀 How to Run

### Prerequisites
- Rust installed (https://rustup.rs)

### Run it locally

```bash
# Clone the repo
git clone https://github.com/yourusername/term-securepad
cd term-securepad

# Run the app
cargo run
```

---

## 📦 Crates Used

- `crossterm` – for terminal rendering and input
- `serde` & `serde_json` – for saving settings or content (if applicable)
- `std::fs` – native file system interactions

---

## 📌 To-Do / Future Ideas

- Replace plain text password with hashed + salted storage (Argon2 or bcrypt)
- Improve text editing UX
- Add support for multiple note files
- Encrypt saved notes

---

## 👤 Author

**Ragy Ashraf**  
Discord Bots ⚙️ | AI ⚡ | Games 🎮 | Learning Rust 🦀  
[GitHub →](https://github.com/ragyashraf)

---

> _"I'm still learning Rust, but this project gave me a solid starting point to explore real-world tools."_

---

🪪 License
MIT License
