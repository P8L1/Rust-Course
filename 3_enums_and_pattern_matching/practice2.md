# Rust Enums, Option, and Match — Practical Drill

1. Inside `main`, define an enum called `TrafficLight` with exactly three variants representing traffic light states.

2. Create three variables inside `main`, each storing a different variant of `TrafficLight`.

3. Define an enum called `IpVersion` with two variants representing IPv4 and IPv6.

4. Create one variable of type `IpVersion` set to the IPv4 variant.

5. Create one variable of type `IpVersion` set to the IPv6 variant.

6. Define a struct called `IpAddress` that stores an `IpVersion` and a `String`.

7. Create an instance of `IpAddress` representing `127.0.0.1` using IPv4.

8. Replace the `IpAddress` struct by redefining the IP address logic using **only an enum** where each variant stores its address data.

9. Define an enum where the IPv4 variant stores a `String` and the IPv6 variant stores a `String`.

10. Create a variable representing localhost using the enum from the previous question.

11. Redefine the enum so the IPv4 variant stores **four `u8` values** instead of a `String`.

12. Create a localhost value using four integers instead of a string.

13. Define an enum called `Message` with one variant that stores **no data**.

14. Add a variant to `Message` that stores a `String`.

15. Add a variant to `Message` that stores **three integers**.

16. Add a variant to `Message` that stores an **anonymous struct** with two `i32` fields.

17. Implement an associated function on `Message` that prints a sentence.

18. Call the associated function using the enum name.

19. Define an enum called `Coin` with four variants representing different coin types.

20. Write a function that takes a `Coin` and returns its value in cents using `match`.

21. Call the function once for each coin variant.

22. Modify the `Coin` enum so one variant stores another enum representing U.S. states.

23. Define the U.S. state enum with at least five states.

24. Derive `Debug` for the state enum.

25. Update the coin-value function so it prints the state when the state-holding coin variant is matched.

26. Call the function with a coin that includes a state value.

27. Create a vector of integers and print it using debug formatting.

28. Declare an `Option<i32>` variable that contains a value.

29. Declare an `Option<i32>` variable that contains no value.

30. Attempt to add an `i32` and an `Option<i32>` and observe the compiler error.

31. Fix the addition by extracting the value using `unwrap_or`.

32. Change the default value passed to `unwrap_or` and recompute the sum.

33. Write a function that takes an `Option<i32>` and returns an `Option<i32>`.

34. Inside the function, use `match` to handle both variants of `Option`.

35. If the value exists, return a new `Option` with the value increased by one.

36. If the value does not exist, return `None`.

37. Call the function with `Some(5)` and store the result.

38. Call the function with `None` and store the result.

39. Write a `match` expression that matches on a `Coin` value.

40. Ensure the `match` handles **every possible variant**.

41. Modify a `match` arm so it binds inner data to a variable.

42. Print the bound value inside the `match` arm.

43. Define an `Option<i8>` variable containing a number.

44. Define an `i8` variable.

45. Safely add both values together.

46. Rewrite the addition using a different default value.

47. Use `if let` to check whether an `Option<i32>` contains a value.

48. Print a message only if the value exists.

49. Use `if let` to check for a **specific value** inside `Some`.

50. Create an enum variant that stores mixed data types.

51. Match on that variant and extract each stored value.

52. Print each extracted value separately.

53. Create a function that accepts an enum as a parameter.

54. Inside the function, match on the enum and perform different actions.

55. Call the function using multiple enum variants.

56. Rewrite one `match` that returns values without using explicit `return`.

57. Confirm the function still returns the correct type.

58. Define an enum where at least one variant stores a tuple.

59. Match on the enum and destructure the tuple.

60. Create an `Option<String>` and safely access its contents.

61. Use `unwrap_or` to provide a fallback `String`.

62. Create an enum instance inside a nested scope.

63. Ensure the enum type is still accessible where needed.

64. Write a function that returns `None` under one condition.

65. Write a function that returns `Some` under another condition.

66. Match on the function’s return value and print different outputs.

67. Replace the `match` with `if let` for the same logic.

68. Create a scenario where forgetting a `None` case would cause a logic error.

69. Fix the logic by explicitly handling `None`.

70. Define an enum variant that stores no data and match on it.

71. Add a second variant and update all matches accordingly.

72. Attempt to remove a match arm and observe the compiler error.

73. Restore the missing match arm.

74. Create a function that accepts `Option<i32>` and never panics.

75. Remove all direct calls to `unwrap`.

76. Demonstrate safe handling using only `match` or `unwrap_or`.

77. Create an enum that models a real-world “one-of” scenario.

78. Instantiate every variant at least once.

79. Match on the enum and perform a different action per variant.

80. Ensure all enum variants are exhaustively handled.

81. Write a function returning an enum instead of a primitive type.

82. Use pattern matching to extract inner values.

83. Print debug output for a complex enum.

84. Add `#[derive(Debug)]` where required to make printing possible.

85. Verify the output matches the structure of the data.

86. Combine `Option` and a custom enum in one function.

87. Match on the `Option` first, then the enum.

88. Reverse the matching order and confirm correctness.

89. Remove unused enum variants and fix resulting errors.

90. Add a new variant and update all related logic.

91. Create an enum-based replacement for a boolean flag.

92. Refactor logic to use the enum instead of `bool`.

93. Match on the enum to control program flow.

94. Write a function that consumes an enum value.

95. Write a function that borrows an enum value.

96. Confirm ownership rules are respected.

97. Store enum values inside a vector.

98. Iterate over the vector and match each value.

99. Print a message for each enum variant encountered.

100. Review your code and ensure **no enum or Option is used without being matched or safely handled**.