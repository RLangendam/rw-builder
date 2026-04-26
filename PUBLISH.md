# Release and Publication Guide

This document outlines the process for releasing a new version of `rw-builder` to [crates.io](https://crates.io/crates/rw-builder).

## 1. SemVer Versioning Guidelines
We strictly follow [Semantic Versioning (SemVer)](https://semver.org/). Before bumping the version in `Cargo.toml`, determine the type of change:

| Change Type | Version Bump | Description |
| :--- | :--- | :--- |
| **Major** | `x.0.0` | Incompatible API changes. |
| **Minor** | `0.x.0` | New functionality in a backward-compatible manner. |
| **Patch** | `0.0.x` | Backward-compatible bug fixes. |

## 2. Pre-Release Checklist
Run these steps in order on the `master` branch:

- [ ] **Update Dependencies**: Run `cargo update` and verify the build still works.
- [ ] **Verify Quality**: Run the full suite from `CONTRIBUTING.md` (Clippy, Tests, Coverage).
- [ ] **Audit Documentation**:
    * Run `cargo doc --all-features --open`.
    * Ensure the `target/doc/rw_builder/index.html` renders correctly and all new features are documented.
- [ ] **Update Version**: Bump the version in `Cargo.toml`.
- [ ] **Changelog**: Update `CHANGELOG.md` (see below).

## 3. Changelog Management
Maintain a clear history of changes. Each release should have a section in `CHANGELOG.md` containing:
* **Added**: For new features.
* **Changed**: For changes in existing functionality.
* **Deprecated**: For soon-to-be removed features.
* **Removed**: For now removed features.
* **Fixed**: For any bug fixes.
* **Security**: In case of vulnerabilities.

## 4. Publishing to Crates.io
Once the `master` branch is ready and all local checks pass:

1.  **Dry Run**: Verify the package structure without uploading.
    ```bash
    cargo publish --dry-run
    ```
2.  **Upload**:
    ```bash
    cargo publish
    ```

## 5. Post-Release Procedures
After a successful publication:

1.  **Git Tagging**: Tag the commit to mark the release.
    ```bash
    git tag -a v0.1.x -m "Release v0.1.x"
    git push origin v0.1.x
    ```
2.  **GitHub Release**: Create a new Release on GitHub based on the tag, copying the relevant notes from the `CHANGELOG.md`.