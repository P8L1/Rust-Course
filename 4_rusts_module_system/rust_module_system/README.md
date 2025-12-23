# Rust Module System Workspace

This repository is a Cargo workspace that breaks the original modules lesson into small, ordered crates. Each lesson crate has its own `README.md` with beginner-friendly notes and a tiny runnable example.

## Workspace layout

- Each lesson lives in its own package directory: `lesson_01_...`, `lesson_02_...`, etc.
- Each package includes:
  - `README.md` for the notes.
  - `src/main.rs` for a minimal runnable entry point.
  - `src/lib.rs` where a library crate is needed for the restaurant examples.
- Files are intentionally kept small (aiming for under 200 lines).

## How to run each lesson (in order)

01. `lesson_01_why_modules` - `cargo run -p lesson_01_why_modules`
02. `lesson_02_packages_crates_modules` - `cargo run -p lesson_02_packages_crates_modules`
03. `lesson_03_creating_packages_and_crates` - `cargo run -p lesson_03_creating_packages_and_crates`
04. `lesson_04_defining_modules_tree` - `cargo run -p lesson_04_defining_modules_tree`
05. `lesson_05_paths_absolute_relative` - `cargo run -p lesson_05_paths_absolute_relative`
06. `lesson_06_privacy_rules` - `cargo run -p lesson_06_privacy_rules`
07. `lesson_07_pub_keyword` - `cargo run -p lesson_07_pub_keyword`
08. `lesson_08_super_keyword` - `cargo run -p lesson_08_super_keyword`
09. `lesson_09_struct_privacy_fields` - `cargo run -p lesson_09_struct_privacy_fields`
10. `lesson_10_enum_privacy` - `cargo run -p lesson_10_enum_privacy`
11. `lesson_11_use_and_pub_use` - `cargo run -p lesson_11_use_and_pub_use`
12. `lesson_12_nested_paths_and_glob` - `cargo run -p lesson_12_nested_paths_and_glob`
13. `lesson_13_modules_across_files` - `cargo run -p lesson_13_modules_across_files`

## Broken examples

When a lesson needs to show compiler errors (privacy, etc.), those examples appear as commented or fenced code blocks inside each lesson `README.md`. The actual source code in `src/` always compiles by default.

## Build everything

From the workspace root:

```bash
cargo build
```
