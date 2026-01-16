# 100 Practical Rust Module-System Questions (No Code, No Hints)

**Rules for yourself while solving:**
- Do **not** copy/paste from the internet.
- You must write the code and file structure yourself.
- Each task must compile and run/build successfully where applicable.

---

## Lesson 01 — Why we don’t keep everything in one file

1. Create a small Rust project that represents a “restaurant helper” library, and split the code into multiple files so that no file exceeds ~200 lines.
2. Create a project structure where “front of restaurant” and “back of restaurant” are separate modules and are located in separate files.
3. Create a scenario where a sensitive “authentication” module exists, and ensure other modules can *use* a public entry-point function but cannot access internal helper functions directly.
4. Create a module that contains an internal implementation detail and demonstrate (by attempting a call) that other parts of the crate cannot call the internal function.
5. Refactor an existing single-file layout into multiple modules across files without changing the public API exposed by the root crate.

---

## Lesson 02 — Packages, crates, modules, and paths

6. Create a new Cargo package, identify the crate that is created by default, and explain in comments what type of crate it is.
7. Create a package that contains **one library crate** and **at least two binary crates**, and confirm Cargo can build all of them.
8. Create a package with **no library crate**, and multiple binary crates, and run each binary separately.
9. Create a library crate that exposes exactly one public function that other crates could call, and ensure the rest of the module is hidden.
10. Build a module tree with at least **3 levels deep** and ensure you can call a deeply nested function using a correct path.
11. Create two different functions in different modules that share the same name, then call both unambiguously using their paths.
12. Create a module structure that forces you to use an **absolute path** for one call and a **relative path** for another call.

---

## Lesson 03 — Creating packages and crates

13. Create a new package using Cargo, run it, and verify which file is the entry point.
14. Convert a binary-style project into one that also contains a library crate, and confirm both compile.
15. Create a package that contains exactly one library crate and exactly one binary crate, and make the binary call into the library.
16. Create a package with multiple binaries inside `src/bin/` and ensure each one compiles independently.
17. Create a binary crate inside `src/bin/` that imports and uses code from your library crate.
18. Create a package that violates the “one library crate maximum” rule and fix the structure so it becomes valid.
19. Create a package where one binary depends on a module that another binary does not use, then confirm both still compile cleanly.
20. Create a project where the library is the main “public API”, and the binary is only a thin wrapper that calls into the library.

---

## Lesson 04 — Defining modules and a module tree

21. Create a library crate that contains a parent module with two child modules, and each child has at least two functions.
22. Create a module tree that includes at least one struct, one enum, and two functions distributed across different module levels.
23. Create a parent module that contains a nested module that contains another nested module, then add a function at the deepest level.
24. Build a restaurant-style module tree that includes both “hosting” and “serving” submodules, each with multiple functions.
25. Create a module tree that you can draw as text (crate → module → submodule → functions) and ensure the code matches that tree.
26. Create a module where you intentionally place a function in the “wrong” module, then move it to the correct location without breaking compilation.
27. Create two sibling modules that need to share a helper function, and place that helper in a location that enforces good encapsulation.
28. Create a module structure where the root crate exposes exactly two public functions, and everything else is private.

---

## Lesson 05 — Paths (absolute and relative)

29. Create a nested module function and call it using an **absolute path** starting from the crate root.
30. Create the same call using a **relative path** from the current module.
31. Create a situation where a relative path fails due to being in a different module, and fix it using the correct path.
32. Create a function in the crate root and call it from a nested module using the correct path style.
33. Create two different nested modules, each calling the same function from a third module, and ensure the paths are correct for both.
34. Create a call chain where a function in a nested module calls a sibling module’s function using a valid path.
35. Create a module with a deeply nested function and write two separate public functions that reach it using different path approaches.
36. Create a case where `crate::` must be used to disambiguate a name collision, and fix the ambiguity.

---

## Lesson 06 — Module privacy rules

37. Create a parent module and child module where the parent cannot call the child’s function because of privacy, and observe the compiler error.
38. Fix the privacy error by exposing only the minimum required items, leaving all other items private.
39. Create a private child module that contains a public function, and verify whether it is accessible from outside the parent.
40. Create a module where a child can access a private item in its parent, but the parent cannot access a private item in the child.
41. Create a “front_of_restaurant” module that contains “hosting”, and ensure “eat_at_restaurant” fails until privacy is corrected.
42. Create a nested module where only one function should be callable externally, and enforce that using privacy rules.
43. Create a module design where internal helper functions remain private even though the module is public.
44. Create a crate that exposes a single public entry point and hides the entire internal module hierarchy.

---

## Lesson 07 — Making items public with `pub`

45. Create a module that is private by default, and then make it public so that code outside can see it.
46. Create a public module that contains a private function, and prove you cannot call it from outside.
47. Expose exactly one function inside a public module and keep all other functions hidden.
48. Create a multi-level module path where each level must be public for external access, and ensure the chain works.
49. Create a module where you accidentally made too much public, then lock it down so only the needed API remains public.
50. Create a restaurant library where the crate root exposes a single “eat_at_restaurant” function that internally uses public and private modules correctly.
51. Create a nested module where the parent is public but the child is private, and fix only what is necessary to reach a function.
52. Create a design where a function is public but the type it returns is private, and then resolve the compilation issues properly.

---

## Lesson 08 — The `super` keyword

53. Create a module where a nested function must call a function defined one module level above it using `super`.
54. Create a second-level nested module that must call a function two levels above by chaining `super` appropriately.
55. Create a module where `super` is used to call a crate-root function from inside a nested module.
56. Create an example where both `super::` and `crate::` would work, and demonstrate both calls.
57. Create a module where removing `super` breaks compilation, then restore the correct `super` usage.
58. Create a module that defines a function at the root and calls it from inside a submodule without making the root function public.
59. Create a “back_of_restaurant” module where an internal fix function calls a root-level serving function using `super`.

---

## Lesson 09 — Struct privacy and fields

60. Create a module containing a struct that is private by default, then attempt to construct it from outside and observe the failure.
61. Make the struct public and attempt to call an associated function that is still private, then fix it correctly.
62. Create a public struct where all fields are private, and confirm you cannot mutate a field from outside the module.
63. Make exactly one field public and keep the others private, then mutate only the public field from outside.
64. Create an associated function that acts like a “constructor” and returns an instance of the struct, then call it externally.
65. Create two different constructor-like associated functions that return different default configurations of the same struct.
66. Create a struct inside a module and expose a public constructor but keep the struct fields private, forcing controlled creation.
67. Create a struct where only the “toast” field is public and the “seasonal fruit” field cannot be accessed outside the module.
68. Create a design where the struct can be created from outside but can only be partially modified from outside due to field privacy.
69. Create a struct with at least three fields and enforce that only one can be modified externally while the others remain internal.

---

## Lesson 10 — Enum privacy

70. Create an enum inside a module and attempt to use it from outside the module while it is still private.
71. Fix the enum privacy by making the enum public, and then construct at least two different variants externally.
72. Create a public enum with at least three variants and confirm the variants are usable outside automatically.
73. Create an enum that stays private but is used internally to decide behavior, while only a public function exposes the result externally.
74. Create a module where the enum is public but the functions that handle it are private, and then expose only one safe public entry point.
75. Create a public enum and write one public function that accepts it and does something different for each variant.

---

## Lesson 11 — The `use` keyword and `pub use`

76. Create a module path that is annoying to type repeatedly, then bring the correct item into scope using `use`.
77. Use `use` with an **absolute path** to bring a nested module into scope, then call a function through the imported name.
78. Use `use` with a **relative path** starting from `self::` to bring a nested module into scope.
79. Create two different modules that each contain a `hosting` module, then use `use` in a way that avoids name collisions.
80. Create a crate where external code should be able to call `hosting::add_to_waitlist()` directly, without needing the full module path, using `pub use`.
81. Create a library where you re-export a nested module from the crate root and confirm the public API is simpler.
82. Create a crate where one internal module uses `use` for convenience, but the crate’s external API remains unchanged.
83. Create a module where you accidentally used `use` incorrectly and caused an unresolved import, then fix the path precisely.
84. Create a crate that intentionally exposes only re-exported items, hiding the internal module tree behind `pub use`.

---

## Lesson 12 — Nested paths and the glob operator

85. Add a dependency to your project and import two items from the same top-level path, then rewrite it using a nested import list.
86. Import both a module itself and one of its public items using a nested path with `self`.
87. Create a module with multiple public functions and bring them all into scope using the glob operator.
88. Create two different modules that both have a function with the same name, then demonstrate how `use` + glob can cause ambiguity.
89. Resolve a naming conflict created by glob imports without removing the imports entirely.
90. Create a scenario where nested imports make your `use` lines significantly shorter compared to separate imports.
91. Create a module where you selectively import only what you need using nested path syntax, and avoid glob.
92. Create a module that uses glob internally for convenience but keeps external API access explicit and clean.

---

## Lesson 13 — Modules across multiple files

93. Create a library crate where the crate root declares a module whose contents live in a separate file.
94. Move an existing inline module into its own file and ensure the crate still compiles without changing functionality.
95. Create a `front_of_restaurant.rs` file and ensure it defines the correct module contents expected by the crate root.
96. Split a nested module (like `hosting`) into its own file inside a folder named after the parent module, and ensure Rust finds it.
97. Create a file structure that includes `lib.rs`, a module file, and a folder module with a submodule file, and make it compile.
98. Create a module spread across multiple files where only one submodule is public, and the rest remain private.
99. Create a crate where a public function lives in a file module, but it calls private helpers located in deeper files.
100. Build a full “restaurant helper” library where:
    - modules are split across multiple files,
    - only a small public API is exposed,
    - internal details are private,
    - at least one call uses `super`,
    - at least one call uses `use`,
    - and at least one item is re-exported using `pub use`.
