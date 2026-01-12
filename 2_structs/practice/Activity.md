# Rust Practical Test — Structs (50 code questions)

*Rules for the learner*

- Every answer must be *Rust code*.
- Unless a question says otherwise, write your solution as a single main.rs file.
- No external crates.
- Keep your code compiling (if you include “wrong” lines, comment them out).
- Use only what appears in the notes: *structs, **instances, **field access, **field mutation via mut, **field init shorthand, **struct update syntax, **tuple structs, *impl methods with &self*, and **associated functions* (Type::function()).

---

## Questions

### Structs: declare + instantiate + access + mutate

1. Declare a struct named Drone with *three named fields*: color, altitude, model.  
   In main, create a *mutable* Drone instance, print its altitude, then change the altitude and print again.

2. Declare a struct named Student with fields: name, email, active, login_count.  
   In main, create a Student instance where you *assign the fields in a different order* than you declared them.

3. Using the Student struct from Q2:  
   In main, create a mutable Student, then *update the email field* to a new value.

4. Declare a struct named GameAccount with fields: username, email, streak, premium.  
   In main, create a mutable instance and update *two different fields* (one text field and one numeric field).

5. Declare a struct named City with fields: name, population, coastal.  
   In main, create a City and store the value of the *population field* in a separate variable, then print it.

6. Declare a struct named Battery with fields: brand, capacity, is_charged.  
   In main, create a mutable Battery and flip is_charged from false to true.

7. Declare a struct named Book with fields: title, pages, published.  
   In main, create a Book and print the pages and published fields.

8. Declare a struct named Profile with fields: username, email, active, sign_in_count.  
   In main, create a Profile and then create a new variable that stores *only* the active field.

9. Declare a struct named Plane with fields: model, altitude, in_flight.  
   In main, create a mutable Plane and update altitude twice (two separate assignments).

10. Declare a struct named Phone with fields: brand, model, battery_percent.  
    In main, create a mutable Phone and set battery_percent to a lower value.

11. Declare a struct named PodcastEpisode with fields: title, duration_minutes, explicit.  
    In main, create an instance and print the duration_minutes.

12. Declare a struct named Ticket with fields: event_name, seat_number, used.  
    In main, create a mutable Ticket and mark it as used.

13. Declare a struct named Wallet with fields: owner, balance_cents, frozen.  
    In main, create a mutable Wallet and decrease the balance by a fixed amount.

14. Declare a struct named SensorReading with fields: device_id, value, ok.  
    In main, create an instance and store ok into a separate variable.

15. Declare a struct named Movie with fields: title, rating, released.  
    In main, create a mutable Movie then update the rating.

---

### Functions that return struct instances (including field init shorthand)

16. Declare a struct named User with fields: username, email, sign_in_count, active.  
    Write a function build_user(email, username) -> User that returns a User instance with:

    - active set to true
    - sign_in_count set to 1  
      Use *field init shorthand* for email and username.  
      In main, call it and store the result in a variable.

17. Using the User struct: write a function build_inactive_user(email, username) -> User that returns a user with:

    - active set to false
    - sign_in_count set to 0  
      Use field init shorthand for the two text fields.

18. Using the User struct: write a function build_user_with_count(email, username, count) -> User that sets:

    - sign_in_count to count
    - active to true  
      Use field init shorthand where possible.

19. Declare a struct named Rectangle with fields: height, width, lenght (spelled exactly like that).  
    Write a function build_tank(height, width, lenght) -> Rectangle using field init shorthand.

20. Declare a struct named Login with fields: email, username, success, attempts.  
    Write a function new_successful_login(email, username) -> Login that sets success to true and attempts to 1.

21. Declare a struct named Order with fields: id, email, paid, items_count.  
    Write a function new_unpaid_order(id, email, items_count) -> Order with paid set to false.

22. Declare a struct named Device with fields: model, owner_email, active, boot_count.  
    Write a function register_device(owner_email, model) -> Device that sets active to true and boot_count to 1, using field init shorthand where possible.

23. Declare a struct named Account with fields: username, email, active, sign_in_count.  
    Write a function make_account(email, username) -> Account using field init shorthand and default values (active true, sign_in_count 1).  
    In main, create two different accounts by calling the function twice.

24. Using the Account struct: write a function toggle_active(account: Account, new_active: bool) -> Account that returns a new Account with the active field set to new_active and all other fields copied from the input using struct update syntax (see notes).

25. Declare a struct named Session with fields: username, email, active, sign_in_count.  
    Write a function new_session(email, username) -> Session using field init shorthand and set sign_in_count to 1 and active to true.

---

### Struct update syntax (..other_instance)

26. Declare a User struct (same fields as Q16).  
    In main, create user_a. Then create user_b where you set a new email and username, but reuse the remaining fields from user_a using *struct update syntax*.

27. Using the User struct: create user_a, then create user_b that changes *only* email and uses ..user_a for the rest.

28. Using the User struct: create user_a, then create user_b that changes *only* active and uses ..user_a for the rest.

29. Declare a struct named Stats with fields: wins, losses, draws, active.  
    In main, create s1, then create s2 where you change wins and reuse all other fields from s1 using ..s1.

30. Declare a struct named Box3D with fields: height, width, lenght.  
    In main, create b1, then create b2 with a different height but reuse the other fields via struct update syntax.

31. Declare a struct named Profile with fields: username, email, sign_in_count, active.  
    In main, create p1. Then create p2 that changes username and email, but keeps sign_in_count and active from p1 using update syntax.

32. Declare a struct named Config with fields: host, port, use_tls, retries.  
    In main, create c1, then create c2 that changes port and use_tls but reuses remaining fields from c1.

---

### Tuple structs (same field layout, different types)

33. Declare tuple structs Color(i32, i32, i32) and Point(i32, i32, i32).  
    In main, create one Color and one Point.

34. Using Color and Point: write a function print_red_component(c: Color) that prints the first number.  
    Call it correctly with a Color.

35. Using Color and Point: write a function move_up(p: Point) -> Point that returns a new Point with the second number increased by 1.  
    Call it in main and store the result.

36. Using Color and Point: write a function same_shape_but_not_same_type(c: Color) -> Point that creates and returns a Point using the three numbers from c.  
    (Hint: you’ll need to access tuple struct fields by index.)

37. Using Color and Point: write a function swap_first_two(p: Point) -> Point that swaps the first and second numbers.

38. Using Color and Point: write a function sum_channels(c: Color) -> i32 that returns the sum of all three numbers.

---

### Methods on structs (impl with &self)

39. Declare a struct Rectangle with fields: height, width, lenght (exact spelling).  
    Write an impl Rectangle block that defines:

    - volume(&self) -> u32
    - area(&self) -> u32  
      In main, create a rectangle and print both results.

40. Using the Rectangle from Q39: add a method bigger_than(&self, other: &Rectangle) -> bool that compares areas.  
    In main, create two rectangles and print the result of rect1.bigger_than(&rect2).

41. Using the Rectangle from Q39: add a method is_flat(&self) -> bool that returns true if any one dimension is zero.  
    In main, test it with one rectangle that is flat and one that is not.

42. Using the Rectangle from Q39: add a method double_height(&self) -> Rectangle that returns a new rectangle with height multiplied by 2 (width and lenght unchanged).

43. Using the Rectangle from Q39: add a method same_volume_as(&self, other: &Rectangle) -> bool that returns true if volumes are equal.

44. Using the Rectangle from Q39: add a method can_fit_inside(&self, other: &Rectangle) -> bool that returns true if *each* dimension of self is less than or equal to the matching dimension of other.

45. Using the Rectangle from Q39: add a method summary(&self) -> u32 that returns height + width + lenght.

46. Using the Rectangle from Q39: write code in main that calls area() and volume() on a rectangle instance and stores them in variables before printing.

---

### Associated functions (no self) + calling with Type::function(...)

47. Declare a struct Rectangle_2D with fields: lenght, width.  
    Implement an associated function build(size) -> Rectangle_2D that sets both fields to size.  
    In main, create a rectangle with Rectangle_2D::build(...).

48. Using Rectangle_2D: add a method area(&self) -> u32.  
    In main, create a square via Rectangle_2D::build(…) and print its area.

49. Using Rectangle_2D: implement an associated function build_custom(lenght, width) -> Rectangle_2D.  
    In main, build two different rectangles using Rectangle_2D::build_custom(...).

50. Using Rectangle_2D: write an associated function from_existing(existing: Rectangle_2D, new_size: u32) -> Rectangle_2D that returns a new Rectangle_2D where width is new_size but lenght is copied from existing using struct update syntax.

---

### Methods

51. Declare struct Counter { value: u32 }.  
    Implement methods:

    - get(&self) -> u32 returns the current value
    - plus(&self, add: u32) -> u32 returns self.value + add  
      In main, create a Counter and call both methods.

52. Declare struct NameTag { name: String }.  
    Implement a method name_len(&self) -> usize that returns the length of the name.  
    In main, create a NameTag and print tag.name_len().

53. Declare struct Rectangle_2D { lenght: u32, width: u32 }.  
    Implement *two* methods in one impl block:

    - area(&self) -> u32
    - is_square(&self) -> bool  
      In main, create one square and one non-square and print is_square() for each.

54. Using Rectangle_2D: implement a method bigger_area_than(&self, other: &Rectangle_2D) -> bool that compares areas.  
    In main, create two rectangles and print the comparison result.

55. Using Rectangle_2D: implement a method scaled(&self, factor: u32) -> Rectangle_2D that returns a new rectangle with both dimensions multiplied by factor.  
    In main, scale a rectangle and print the new area.

56. Using Rectangle_2D: implement a method add_width(&self, delta: u32) -> Rectangle_2D that returns a new rectangle with width increased by delta (lenght unchanged).  
    In main, call the method and print the old and new widths.

57. Using Rectangle_2D: implement a method same_dimensions_as(&self, other: &Rectangle_2D) -> bool that returns true only if lenght and width match exactly.  
    In main, test it with matching and non-matching rectangles.

58. Using Rectangle_2D: implement a method max_side(&self) -> u32 that returns whichever is larger: lenght or width.  
    In main, print the result for two different rectangles.

59. Using Rectangle_2D: implement a method min_side(&self) -> u32 that returns whichever is smaller: lenght or width.  
    In main, print the result.

60. Using Rectangle_2D: implement a method fits_inside(&self, other: &Rectangle_2D) -> bool where self fits inside other if both sides are <= the corresponding sides.  
    In main, test both a fitting and a non-fitting case.

61. Declare struct Rectangle { height: u32, width: u32, lenght: u32 }.  
    Implement methods:

    - volume(&self) -> u32
    - surface_area(&self) -> u32  
      In main, print both results.

62. Using the Rectangle from Q61: implement a method bigger_volume_than(&self, other: &Rectangle) -> bool.  
    In main, create two rectangles and print the result.

63. Using the Rectangle from Q61: implement a method same_surface_area_as(&self, other: &Rectangle) -> bool by calling self.surface_area() and other.surface_area().  
    In main, test it with two rectangles that do and do not match.

64. Using the Rectangle from Q61: implement a method with_height(&self, new_height: u32) -> Rectangle that returns a new rectangle where only the height changes.  
    In main, create a rectangle, call with_height, and print old vs new heights.

65. Using the Rectangle from Q61: implement a method summary(&self) -> u32 that returns height + width + lenght.  
    In main, store the summary in a variable and print it.

66. Using the Rectangle from Q61: implement a method bigger_than_by_area(&self, other: &Rectangle) -> bool where you compare *surface areas*.  
    In main, call it as rect1.bigger_than_by_area(&rect2).

67. Declare struct Rectangle_2D { lenght: u32, width: u32 }.  
    Implement an *associated function* build(size: u32) -> Rectangle_2D that makes a square.  
    Also implement a method area(&self) -> u32.  
    In main, build a square with Rectangle_2D::build(…) and call .area().

68. Using Rectangle_2D: implement an associated function build_custom(lenght: u32, width: u32) -> Rectangle_2D.  
    In main, create two rectangles using Rectangle_2D::build_custom(...) and compare their areas using a method.

69. “Method vs function” practice (code only):  
    Declare struct Wallet { balance_cents: u64 }.  
    Implement:

    - a method is_rich(&self, threshold: u64) -> bool
    - a *free function* is_rich_wallet(w: &Wallet, threshold: u64) -> bool  
      In main, call both and print both results.

70. “Calling methods with references” practice:  
    Declare struct Rectangle_2D { lenght: u32, width: u32 }.  
    Implement a method bigger_than(&self, other: &Rectangle_2D) -> bool that compares areas.  
    In main, create a and b, then call a.bigger_than(&b) and print the result.  
    (Make sure the method signature takes &Rectangle_2D for the other rectangle.)