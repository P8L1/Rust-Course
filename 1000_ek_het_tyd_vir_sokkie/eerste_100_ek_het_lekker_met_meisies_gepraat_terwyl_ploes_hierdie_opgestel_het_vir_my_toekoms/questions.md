# Rust Structs & Enums — 100 Method + Associated Function Questions (Text-Based)

**Rules reminder (for the learner):**
- Do **every question inside `main()`**, and keep each question inside its **own `{ ... }` block**.
- **Every question is independent.** Declare a **new struct/enum name** each time.
- Only focus on **methods** and **associated functions** (the stuff you put in `impl` blocks).
- Keep logic simple (no loops / no complex code).

---

## Questions

1. Declare a struct called **Person** with fields for a name (text) and an age (number).  
   Implement an **associated function** `new(name, age)` that returns a new `Person`.  
   In `main`, create one person with `Person::new(...)` and print the name.

2. Declare a struct called **Book** with a title (text) and pages (number).  
   Implement a **method** `pages_left(current_page)` that returns how many pages remain.  
   Create a book, call the method, and print the result.

3. Declare a struct called **BankAccount** with an owner (text) and a balance (number).  
   Implement a **method** `is_rich()` that returns `true` if the balance is greater than 1_000.  
   Create an account and print the result of `is_rich()`.

4. Declare a struct called **Circle** with a radius (number).  
   Implement a **method** `diameter()` that returns radius × 2.  
   Create a circle, call the method, and print the diameter.

5. Declare a struct called **Rectangle** with height and width (numbers).  
   Implement a **method** `area()` that returns height × width.  
   Create a rectangle and print its area.

6. Declare a struct called **Rectangle2** with height and width (numbers).  
   Implement a **method** `bigger_than(other_rectangle)` that returns `true` if `self.area()` is greater than the other’s area.  
   Create two rectangles and print whether the first is bigger than the second.

7. Declare a struct called **Password** with a value (text).  
   Implement a **method** `length()` that returns the number of characters in the password text.  
   Create a password and print its length.

8. Declare a struct called **Email** with an address (text).  
   Implement an **associated function** `example()` that returns a valid example email address instance.  
   Call `Email::example()` and print the address.

9. Declare a struct called **Student** with a name (text) and score (number).  
   Implement a **method** `passed()` that returns `true` if score is at least 50.  
   Create a student and print `passed()`.

10. Declare a struct called **Temperature** with a value (number).  
    Implement a **method** `is_freezing()` that returns `true` if value is 0 or less.  
    Create a temperature and print `is_freezing()`.

11. Declare a struct called **Wallet** with coins (number).  
    Implement an **associated function** `empty()` that returns a wallet with zero coins.  
    Call it and print the coins count.

12. Declare a struct called **Car** with brand (text) and speed (number).  
    Implement a **method** `is_fast()` that returns `true` if speed is greater than 120.  
    Create a car and print `is_fast()`.

13. Declare a struct called **Triangle** with base and height (numbers).  
    Implement a **method** `area()` that returns (base × height) / 2.  
    Create a triangle and print its area.

14. Declare a struct called **Laptop** with model (text) and battery_percent (number).  
    Implement a **method** `needs_charging()` that returns `true` if battery is below 20.  
    Create a laptop and print the result.

15. Declare a struct called **GamePlayer** with username (text) and level (number).  
    Implement a **method** `is_pro()` that returns `true` if level is 50 or higher.  
    Create a player and print `is_pro()`.

16. Declare a struct called **Shoe** with size (number) and color (text).  
    Implement an **associated function** `black(size)` that returns a black shoe of the given size.  
    Create one using `Shoe::black(...)` and print its color.

17. Declare a struct called **Sandwich** with bread_type (text) and calories (number).  
    Implement a **method** `is_healthy()` that returns `true` if calories are below 400.  
    Create a sandwich and print the result.

18. Declare a struct called **Movie** with name (text) and minutes (number).  
    Implement a **method** `hours()` that returns minutes converted to hours as a whole number (rounded down).  
    Create a movie and print `hours()`.

19. Declare a struct called **Counter** with value (number).  
    Implement an **associated function** `start_at(value)` that returns a counter starting at that value.  
    Create one and print the value.

20. Declare a struct called **Distance** with meters (number).  
    Implement a **method** `to_km()` that returns kilometers as a decimal number (meters / 1000).  
    Create a distance and print `to_km()`.

21. Declare a struct called **Dog** with name (text) and age (number).  
    Implement a **method** `is_puppy()` that returns `true` if age is less than 2.  
    Create a dog and print `is_puppy()`.

22. Declare a struct called **LightBulb** with watts (number).  
    Implement a **method** `is_bright()` that returns `true` if watts is at least 60.  
    Create a bulb and print the result.

23. Declare a struct called **Box3D** with height, width, length (numbers).  
    Implement a **method** `volume()` that multiplies the three values.  
    Create a box and print the volume.

24. Declare a struct called **Box3D_2** with height, width, length (numbers).  
    Implement both methods: `volume()` and `surface_area()` (keep it simple: surface area = 2*(hw + hl + wl)).  
    Create a box and print both results.

25. Declare a struct called **Tank** with height, width, length (numbers).  
    Implement a **method** `bigger_than(other_tank)` that compares volumes and returns a boolean.  
    Create two tanks and print the comparison.

26. Declare a struct called **Username** with a value (text).  
    Implement a **method** `starts_with_letter(letter)` that returns `true` if it starts with the given letter.  
    Create a username and print the result.

27. Declare a struct called **Note** with text (text).  
    Implement an **associated function** `from(text)` that returns a new note.  
    Create a note and print its text.

28. Declare a struct called **Speed** with km_per_hour (number).  
    Implement a **method** `to_meters_per_second()` using the formula km/h ÷ 3.6 (keep it simple).  
    Create a speed and print the result.

29. Declare a struct called **Pizza** with slices (number).  
    Implement a **method** `slices_left(eaten)` returning how many slices remain.  
    Create a pizza, call the method, and print the answer.

30. Declare a struct called **TV** with size_in_inches (number) and smart (true/false).  
    Implement a **method** `is_big()` that returns `true` if size is at least 55.  
    Create a TV and print `is_big()`.

31. Declare a struct called **Ticket** with id (number) and is_valid (true/false).  
    Implement a **method** `can_enter()` that returns the value of `is_valid`.  
    Create a ticket and print `can_enter()`.

32. Declare a struct called **Dice** with sides (number).  
    Implement an **associated function** `standard()` that returns a dice with 6 sides.  
    Create it and print sides.

33. Declare a struct called **Profile** with username (text) and active (true/false).  
    Implement a **method** `is_active()` that returns the active field.  
    Create a profile and print `is_active()`.

34. Declare a struct called **MessageBox** with messages_count (number).  
    Implement a **method** `is_empty()` that returns `true` if messages_count is 0.  
    Create one and print `is_empty()`.

35. Declare a struct called **Window** with width and height (numbers).  
    Implement a **method** `is_square()` that returns `true` if width equals height.  
    Create a window and print `is_square()`.

36. Declare a struct called **Square** with size (number).  
    Implement an **associated function** `build(size)` that returns a square.  
    Create one and print its area using a method `area()`.

37. Declare a struct called **Clock** with hours (number) and minutes (number).  
    Implement a **method** `total_minutes()` that returns hours * 60 + minutes.  
    Create a clock time and print total minutes.

38. Declare a struct called **Point2D** with x and y (numbers).  
    Implement a **method** `is_origin()` that returns `true` if both are zero.  
    Create one and print the result.

39. Declare a struct called **ScoreBoard** with team_a (number) and team_b (number).  
    Implement a **method** `winner()` that returns text: `"A"`, `"B"`, or `"Draw"`.  
    Create a scoreboard and print the winner.

40. Declare a struct called **CoinJar** with pennies (number).  
    Implement a **method** `value_in_cents()` that returns the total cents (same as pennies for simplicity).  
    Create a jar and print the value.

41. Declare a struct called **Level** with current (number).  
    Implement an **associated function** `one()` that returns a level at 1.  
    Create it and print current.

42. Declare a struct called **Video** with title (text) and seconds (number).  
    Implement a **method** `minutes()` that returns seconds / 60 (rounded down).  
    Create a video and print minutes.

43. Declare a struct called **FuelTank** with liters (number).  
    Implement a **method** `is_full()` that returns `true` if liters is at least 50.  
    Create a tank and print `is_full()`.

44. Declare a struct called **Phone** with brand (text) and storage_gb (number).  
    Implement a **method** `is_high_storage()` that returns `true` if storage is 256 or more.  
    Create a phone and print result.

45. Declare a struct called **Paper** with width and height (numbers).  
    Implement a **method** `area()` and print it.

46. Declare a struct called **Microphone** with name (text) and is_muted (true/false).  
    Implement a **method** `can_record()` that returns `true` if not muted.  
    Create a mic and print `can_record()`.

47. Declare a struct called **Exam** with total_marks (number) and achieved_marks (number).  
    Implement a **method** `percentage()` that returns achieved / total * 100 (keep integer math simple).  
    Create an exam and print percentage.

48. Declare a struct called **Coupon** with code (text) and discount_percent (number).  
    Implement a **method** `is_big_discount()` that returns `true` if discount is 30 or more.  
    Create a coupon and print the result.

49. Declare a struct called **Battery** with percent (number).  
    Implement a **method** `status_text()` returning `"Low"`, `"Medium"`, or `"High"` based on percent (simple thresholds).  
    Create a battery and print the status.

50. Declare a struct called **Room** with name (text) and people (number).  
    Implement a **method** `is_crowded()` returning `true` if people is more than 10.  
    Create and print the result.

51. Declare a struct called **Lock** with locked (true/false).  
    Implement a **method** `is_locked()` that returns the locked value.  
    Create a lock and print it.

52. Declare a struct called **Playlist** with songs (number).  
    Implement a **method** `has_many_songs()` returning `true` if songs is at least 20.  
    Create and print the result.

53. Declare a struct called **Cookie** with name (text) and price (number).  
    Implement a **method** `is_expensive()` returning `true` if price is above 20.  
    Create and print.

54. Declare a struct called **Monitor** with width_px and height_px (numbers).  
    Implement a **method** `total_pixels()` returning width × height.  
    Create and print.

55. Declare a struct called **Chapter** with title (text) and questions (number).  
    Implement a **method** `is_long()` returning `true` if questions is at least 15.  
    Create and print.

56. Declare a struct called **Passport** with country (text) and expires_in_years (number).  
    Implement a **method** `is_expiring_soon()` returning `true` if expires_in_years is 1 or less.  
    Create and print.

57. Declare a struct called **Guitar** with strings (number).  
    Implement an **associated function** `standard()` that returns 6 strings.  
    Create and print.

58. Declare a struct called **Fridge** with temperature (number).  
    Implement a **method** `safe_for_food()` returning `true` if temperature is between 0 and 5 (inclusive).  
    Create and print.

59. Declare a struct called **Bread** with slices (number).  
    Implement an **associated function** `new_loaf()` returning a loaf with 20 slices.  
    Create and print.

60. Declare a struct called **Device** with name (text) and online (true/false).  
    Implement a **method** `is_online()` returning the online value.  
    Create and print.

---

## Tuple Struct Questions (still methods + associated functions)

61. Declare a tuple struct called **ColorRGB** that stores three numbers (red, green, blue).  
    Implement an **associated function** `white()` returning (255, 255, 255).  
    Create and print the values using debug formatting.

62. Declare a tuple struct called **Point3D** that stores three numbers (x, y, z).  
    Implement a **method** `sum()` that returns x + y + z.  
    Create and print `sum()`.

63. Declare a tuple struct called **TemperaturePair** storing two numbers (min, max).  
    Implement a **method** `range()` returning max - min.  
    Create and print.

64. Declare a tuple struct called **NameTag** storing two texts (first, last).  
    Implement a **method** `full_name()` returning the combined name as a single text with a space.  
    Create and print.

65. Declare a tuple struct called **BoxSize** storing three numbers (height, width, length).  
    Implement a **method** `volume()` multiplying the three.  
    Create and print.

66. Declare a tuple struct called **PairNumbers** storing two numbers.  
    Implement an **associated function** `same(value)` that sets both numbers to the same value.  
    Create and print.

67. Declare a tuple struct called **Money** storing one number (amount).  
    Implement a **method** `is_zero()` returning `true` if amount equals zero.  
    Create and print.

68. Declare a tuple struct called **LetterGrade** storing one character.  
    Implement a **method** `is_pass()` returning `true` if grade is A, B, or C.  
    Create and print.

69. Declare a tuple struct called **Pixel** storing one number.  
    Implement an **associated function** `black()` returning 0.  
    Create and print.

70. Declare a tuple struct called **UsernameTag** storing one text.  
    Implement a **method** `is_short()` returning `true` if the text length is less than 5.  
    Create and print.

---

## More Struct Method + Associated Function Questions (Rectangle-style)

71. Declare a struct called **Cuboid** with height, width, length (numbers).  
    Implement methods `volume()` and `surface_area()` (2*(hw + hl + wl)).  
    Create and print both.

72. Declare a struct called **CuboidCompare** with the same fields.  
    Implement a **method** `bigger_than(other)` that compares volumes.  
    Create two and print the comparison.

73. Declare a struct called **Square2D** with size (number).  
    Implement an **associated function** `build(size)` and a **method** `area()`.  
    Create and print area.

74. Declare a struct called **Rectangle2D** with length and width (numbers).  
    Implement an **associated function** `build_square(size)` that returns a rectangle where length == width.  
    Create and print the fields.

75. Declare a struct called **Rectangle2D_Compare** with length and width.  
    Implement a **method** `bigger_area_than(other)` using an internal `area()` method.  
    Create two and print.

76. Declare a struct called **FishTank** with height, width, length.  
    Implement a **method** `volume()` and print it.

77. Declare a struct called **ShippingBox** with height, width, length.  
    Implement a **method** `can_hold(other_box)` returning `true` if your volume is bigger.  
    Create two and print.

78. Declare a struct called **PictureFrame** with height and width.  
    Implement a **method** `perimeter()` returning 2*(h + w).  
    Create and print.

79. Declare a struct called **Carpet** with length and width.  
    Implement a **method** `needs_glue()` returning `true` if area is over 20 (simple).  
    Create and print.

80. Declare a struct called **GardenPlot** with length and width.  
    Implement an **associated function** `square(size)` returning a plot where length and width are both size.  
    Create and print the area using a method.

---

## Enum Questions (methods + associated functions in `impl`)

81. Declare an enum called **TrafficLight** with variants Red, Yellow, Green.  
    Implement a **method** `is_stop()` that returns `true` only for Red.  
    Create one light and print `is_stop()`.

82. Declare an enum called **DoorState** with variants Open and Closed.  
    Implement an **associated function** `default_closed()` returning Closed.  
    Call it and print something about it (use debug formatting).

83. Declare an enum called **PowerMode** with variants On and Off.  
    Implement a **method** `is_on()` returning true only for On.  
    Create a value and print the result.

84. Declare an enum called **LoginResult** with variants Success and Failed.  
    Implement an **associated function** `success()` returning Success.  
    Call and print it (debug formatting is fine).

85. Declare an enum called **Direction** with variants Left and Right.  
    Implement an **associated function** `left()` returning Left.  
    Call and print.

86. Declare an enum called **Mood** with variants Happy, Sad, Angry.  
    Implement a **method** `is_happy()` returning true only for Happy.  
    Create an instance and print.

87. Declare an enum called **Message** with variants Quit, Write(text), Move { x, y }.  
    Implement an **associated function** `some_function()` that prints `"Let's get Rusty"` (exact text).  
    Call it with `Message::some_function()`.

88. Declare an enum called **Sound** with variants Beep and Buzz.  
    Implement a **method** `as_text()` returning `"beep"` or `"buzz"` as text.  
    Create one and print `as_text()`.

89. Declare an enum called **IpVersion** with variants V4(address data) and V6(address data).  
    Implement an **associated function** `localhost_v4()` that returns the V4 variant set to represent 127.0.0.1 (your choice of format).  
    Create it and print it (debug formatting).

90. Declare an enum called **IpVersion2** with variants V4 and V6 (no data stored).  
    Implement a **method** `is_v6()` that returns true only for V6.  
    Create both and print results.

91. Declare an enum called **PaymentStatus** with variants Pending, Paid, Failed.  
    Implement a **method** `is_done()` returning true only for Paid.  
    Create a status and print.

92. Declare an enum called **FileState** with variants Exists and Missing.  
    Implement an **associated function** `missing()` returning Missing.  
    Call and print.

93. Declare an enum called **Role** with variants Admin, User, Guest.  
    Implement a **method** `can_edit()` returning true only for Admin.  
    Create one role and print.

94. Declare an enum called **GameState** with variants Running and Paused.  
    Implement an **associated function** `start()` returning Running.  
    Call and print.

95. Declare an enum called **Alarm** with variants On and Off.  
    Implement a **method** `is_on()` returning true only for On.  
    Create and print.

96. Declare an enum called **Connection** with variants Connected(text) and Disconnected.  
    Implement a **method** `has_ip()` returning true only for Connected(...).  
    Create a connected value and print.

97. Declare an enum called **Answer** with variants Yes and No.  
    Implement an **associated function** `yes()` returning Yes.  
    Call and print.

98. Declare an enum called **Quality** with variants Low, Medium, High.  
    Implement a **method** `is_high()` returning true only for High.  
    Create and print.

99. Declare an enum called **Season** with variants Summer, Autumn, Winter, Spring.  
    Implement an **associated function** `default()` returning Summer.  
    Call and print.

100. Declare an enum called **StateSwitch** with variants Enabled and Disabled.  
     Implement a **method** `is_enabled()` returning true only for Enabled.  
     Create one and print.

---
