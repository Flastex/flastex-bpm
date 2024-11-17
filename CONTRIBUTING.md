# Contributing to Flastex

First off, thank you for considering contributing to **Flastex**! We appreciate your time and effort, and we want to make sure that contributing is a smooth and enjoyable experience for everyone involved. Below are the guidelines and steps to contribute to this project.

## Code of Conduct

By participating in this project, you agree to uphold the [Code of Conduct](CODE_OF_CONDUCT.md). Be kind, respectful, and constructive when interacting with others.

## How Can I Contribute?

### 1. Reporting Bugs

If you've found a bug, please follow these steps:
1. **Check existing issues**: Make sure the bug hasn't been reported yet.
2. **Open a new issue**: If it's new, [open an issue](https://github.com/Flastex/flastex-bpm/issues) and provide detailed information. Include:
   - A clear title and description of the bug.
   - Steps to reproduce the issue. If possible, provide a reproducible example in a fork of the project or include the sources of the model that causes the issue. This will help us address the problem more efficiently.
   - The expected behavior compared to the actual behavior.
   - Screenshots or logs (if applicable).

### 2. Suggesting Enhancements

If you have ideas for improvements or new features, we’d love to hear them! Please follow these steps:
1. **Check for existing feature requests**: Someone else may have already suggested a similar idea.
2. **Submit a feature request**: If it’s a new idea, open a new issue and select the "Feature Request" template.

### 3. Contributing Code

We welcome pull requests (PRs) for bug fixes, improvements, and new features. Here’s how you can contribute code:

#### 1. Fork the Repository
1. Fork the repository to your own GitHub account.
2. Clone your fork to your local machine:
   ```bash
   git clone git@github.com:your-username/flastex-bpm.git
3. Configure git locally to run the formatter
```bash
git config core.hooksPath .githooks
```   
3. Create a New Branch
Create a new branch starting with `bugfix/` or `feature/` followed by a brief description of the change.
Example: `feature/add-user-authentication` or `bugfix/fix-login-error`.
4. Make Your Changes
Add tests for the changes you are making. Ensure all changes are covered by appropriate unit or integration tests.
Avoid adding unnecessary dependencies unless they provide significant value.
5. Run Cargo Clippy
Fix issues reported by running
```bash
cargo clippy
```
in the modified folder(s).
6. Run Tests
Make sure all tests pass locally before submitting your pull request:
7. Commit Your Changes
Use Semantic Versioning for your commit messages:
`fix`: for bug fixes.
`feat`: for new features.
`docs`: for documentation changes.
`chore`: for housekeeping tasks.
Example: `fix: correct user authentication flow`.
8. Open a Pull Request (PR)
Push your branch to your GitHub fork and open a PR against the main repository.
Clearly describe the purpose of your changes and reference any related issues or discussions.
9. PR Review Process
Be open to feedback during the review process. Pull requests are subject to review by maintainers, and changes may be requested before merging.
PRs will undergo a squash merge, so ensure your commits are well-structured.

### 4. Asking "How to" Questions

If you need help with understanding how to use Flastex, best practices, or specific implementation details, we encourage you to ask your question on **Stack Overflow**. This helps us keep the issue tracker focused on bugs and feature requests while allowing the community to assist with general questions.

1. **Search for existing questions**: Before posting, search on Stack Overflow to see if someone has already asked a similar question.
2. **Ask a new question**: If no similar question exists, please [ask a new question](https://stackoverflow.com/questions/ask) on Stack Overflow. Be sure to:
   - Tag your question with `flastex` and any other relevant tags.
   - Provide enough detail and context for others to understand your issue, including code snippets, if applicable.
   - Mention the version of Flastex you are using.

By using Stack Overflow, you’ll help other developers who might face the same issue in the future. Additionally, we can monitor and contribute to the discussion, while maintaining the focus of GitHub issues on bugs and enhancements.
