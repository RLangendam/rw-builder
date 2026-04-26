# Contributing to rw-builder

Thank you for your interest in improving `rw-builder`! This document outlines the development process and quality standards for this project.

## Development Environment

We recommend developing on **Linux** using **Visual Studio Code**.

### Recommended Setup

-   **VSCode Extensions**: Install the recommended extensions for **Rust** (rust-analyzer), **Spell Checking**, and **Coverage Gutters**.
-   **Tooling**: Make sure you have the following tools installed for testing and coverage:
    
    Bash
    
    ```
    rustup component add llvm-tools
    cargo install cargo-all-features grcov
    ```
    

## Quality Control Checklist

Before submitting a Pull Request, ensure the following steps are completed:

1.  **Correctness**: All tests must pass.
    
    -   Run `cargo test --all-features`.
    -   Run `cargo all-features test` to verify all relevant feature combinations.
2.  **Linting**:
    
    -   `cargo check` and `cargo clippy` must be clean.
3.  **Formatting**:
    
    -   Run `cargo fmt` (or enable "Format on Save" in VSCode).
4.  **Coverage**:
    
    -   Run `./test_coverage.sh` and ensure coverage has not decreased.
    -   The report is available locally at `coverage/html/index.html`.
5.  **Spelling**:
    
    -   Check for typos (facilitated by the 'Code Spell Checker' VSCode plugin).

## Roadmap & Areas for Contribution

We are actively seeking help in the following areas:

-   **New Builders**: Adding more Sources (`FileBuilder`, etc.) and Sinks (beyond `WincodeBuilder`).
-   **Buffering**: Improving integration with `BufRead` and `BufWrite` traits to leverage specialized functionality.
-   **Embedded Support**: Implementing `no_std` compatibility for use in embedded programming.
-   **Testing**:
    
    -   Transitioning tests into specialized folders as the project grows.
    -   Adding **Doc-tests** to provide live examples within the documentation.

> **Note on Features**: By default, all features are disabled. While developing, you may find it convenient to temporarily uncomment the default features line in `Cargo.toml`.