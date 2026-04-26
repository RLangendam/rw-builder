---
name: Compile issue
about: Report / ask about a compilation issue
title: ''
labels: ''
assignees: ''

---

# Common issues

**Problem**: `WincodeBuilder`

**Quick solution**: `cargo update`

**Details**: This happens when multiple versions of the `wincode` crate are in use. Check your `Cargo.lock` file for all versions of `wincode`.
