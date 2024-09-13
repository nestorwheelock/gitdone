

# GitDone
A simple wrapper for git written in rust that makes it faster and therefore more likely to save work.

GitDone is a lightweight command-line tool written in Rust that helps streamline your workflow by simplifying common Git commands like `add`, `commit`, and `push`. By automating these repetitive tasks, GitDone reduces friction and helps ensure you never lose your work. 

## Features

- **Automated Git Workflow**: Handles `git add .`, `git commit -m`, and `git push` in one command.
- **Message Prompts**: If no commit message is provided, GitDone will prompt you for one.
- **Efficiency**: Written in Rust, it’s designed for speed and performance with minimal overhead.
- **Cross-platform**: Works on any platform where Rust and Git are supported (Linux, macOS, Windows).

## Installation

### Prerequisites

- **Git**: GitDone requires Git to be installed and available in your system's PATH.
- **Rust**: You need to have Rust installed on your machine. You can install Rust by following the instructions on [Rust's official website](https://www.rust-lang.org/tools/install).

### Steps to Install GitDone

1. Clone the repository:
   ```bash
   git clone https://github.com/nestorwheelock/gitdone.git
   ```

2. Navigate into the project directory:
   ```bash
   cd gitdone
   ```

3. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

4. (Optional) Install it globally by moving the binary to `/usr/local/bin`:
   ```bash
   sudo mv target/release/gitdone /usr/local/bin/
   ```

Now, you can use `gitdone` from anywhere in your terminal.

## Usage

### Basic Usage

1. To commit and push changes with a provided message:
   ```bash
   gitdone "your commit message"
   ```

2. If no commit message is provided, GitDone will prompt you to enter one interactively:
   ```bash
   gitdone
   ```

### Example

```bash
# In a directory with changes to commit
gitdone "Fixed a bug and updated documentation"
```

GitDone will automatically:
- Stage all the changes (`git add .`)
- Commit them with the provided message (`git commit -m "message"`)
- Push them to the `main` (or `master`) branch (`git push origin main`)

### Configurations

GitDone assumes the branch name is `main`. If you are working on a different branch, you can either:
1. Modify the script to use the correct branch.
2. Manually switch to your branch and push to it.

## Why Use GitDone?

If you regularly use Git for version control, you’re likely familiar with the routine of adding, committing, and pushing changes. This simple workflow, while repetitive, is vital to preserving your work. GitDone automates this flow, helping you:
- Avoid forgetting to push your changes.
- Ensure commits are always accompanied by a message.
- Save time by bundling multiple Git commands into one.

## Contributing

Contributions are welcome! If you’d like to improve GitDone, feel free to fork the repository, make changes, and open a pull request.

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -m 'Add your feature'`
4. Push to the branch: `git push origin feature/your-feature`
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Future Enhancements

- **Custom Branch Support**: Add support for automatically detecting and pushing to the current branch.
- **Interactive Mode Enhancements**: More options to interactively review or select files before committing.
- **Additional Git Features**: Integration with other useful Git commands, like `git pull` or `git stash`.

---

GitDone: Simplifying your Git workflow, one commit at a time!
