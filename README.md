# CLI Todo App

**CLI Todo App** is a slim, basic, and easy-to-use command-line tool designed to help you manage your tasks efficiently right from your terminal. Built with Rust, this lightweight application is ideal for developers who prefer working in terminal-based environments, especially those using tools like `tmux` and `nvim`.

### Key Features:

- **Slim and Lightweight**: No unnecessary bells and whistles—just a simple, focused tool to manage your to-do lists.
- **Rust-Powered**: Fast and reliable performance, with minimal system overhead.
- **Ideal for Terminal Enthusiasts**: Seamlessly integrates with `tmux`, `nvim`, or any other terminal-based workflow.

### Why Use CLI Todo App?

If you spend most of your time in a terminal, switching between multiple applications can be a hassle. With **CLI Todo App**, you can manage your tasks without leaving your development environment, keeping you in the flow and boosting your productivity.

### Usage 

Using the app is very easy. You only have to remember 4 operations. 

Use '+' to add a new todo. e.g +write rust app

Use '*' to mark toggle todo status. e.g *rust

Use '-' to delete a todo. e.g -rust

Use '!' to quit the app. 

The todos will be persisted in a file when closing. 

### Installation

#### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/) installed. If not, you can easily install it using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Install via Cargo

To install the app directly using Cargo, run the following command:

```bash
cargo install --git https://github.com/alihanekiz/todo_app
```

#### Building from Source

If you prefer to build the app from source, follow these steps:

1. Clone the repository:
   
```bash
git clone https://github.com/alihanekiz/todo_app.git
```

2. Navigate to the project directory:

```bash
cd todo_app
```

3. Build the project using Cargo:

```bash
cargo build --release
```

4. Move the compiled binary to a directory in your PATH:

```bash
mv target/release/todo_app /usr/local/bin/
```

### Contributions

I welcome contributions to **CLI Todo App**! If you'd like to contribute, please follow these steps:

1. **Fork the Repository**: Click the "Fork" button at the top right corner of this repository page to create a copy of this repository under your GitHub account.

2. **Clone Your Fork**: Clone your forked repository to your local machine.

3. **Create a Branch**: Create a new branch for your feature or bug fix.

    ```bash
    git checkout -b feature-name
    ```

4. **Make Your Changes**: Implement your changes in the codebase.

5. **Commit Your Changes**: Commit your changes with a meaningful commit message.

    ```bash
    git add .
    git commit -m "Add feature-name or Fix issue with XYZ"
    ```

6. **Push to Your Fork**: Push your changes to your forked repository.

    ```bash
    git push origin feature-name
    ```

7. **Create a Pull Request**: Go to the original repository on GitHub and create a pull request (PR) from your forked repository’s branch to the `main` branch of the original repository.

