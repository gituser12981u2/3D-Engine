# Contributing to 3D Engine

This document outlines the process for contributing and sets some guidelines to maintain code quality and consistency.

## How to Contribute

1. Fork the repository.
2. Create a new branch from `main` for your work.
3. Make your changes, adhering to the coding conventions described below.
4. Write or update tests as necessary.
5. Update documentation as needed.
6. Commit your changes, using meaningful commit messages that follow our commit message guidelines.
7. Push your branch to your fork.
8. Submit a pull request to the `main` branch of our repository.

## Coding Conventions

- Follow Rust's official style guidelines
- Use `cargo fmt` to format your code before committing.
- Ensure your code passes `cargo clippy` without warnings.
- Write clear, self-explanatory code. Use comments only when necessary to explain complex algorithms or non-obvious decisions

## Commit Message Guidelines

This project follows the Conventional Commits specification. Each commit message should be structured as follows:

```bash
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types include:

- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Changes that do not affect the meaning of the code
- refactor: A code change that neither fixes a bug nor adds a feature
- perf: A code change the improves performance
- test: Adding missing tests or correcting existing tests
- chore: Changes to the build process or auxiliary tools and libraries

**Note**- Everything in the commit should be written in the imperative mood.

## Pull Request Process

1. Ensure your code adheres to the coding conventions outlined above.
2. Update the README.md with details of changes to the interface, if applicable.
3. Your pull request will be reviewed by at least one maintainer. Be prepared to make changes if requested.

## Reporting Bugs

1. Ensure the bug was not already reported by searching on GitHub under Issues.
2. If you are unable to find an open issue addressing the problem, open a new one. Be sure to include a title and clear description, as much relevant information as possible, and a code sample or an executable test case demonstrating the expected behavior that is not occurring.

## Any Contribution that are made will be under the MIT Software License

When code is submitted to this project its understood to be under the same MIT License that covers the project.
