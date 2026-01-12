# Rust Practice: Declaring Enums + Initialising Them (20 practical tasks)

**Rules for the student (implicit):**

- Do everything in a single Rust file with a `fn main()`.
- Each task should compile.
- Focus on _declaring_ enums and _creating values_ (initialising) of those enums.
- You may create small helper structs as needed.

---

1. **Mode switch (no data)**

   - Declare an enum that represents three app modes (for example: “off”, “idle”, “active”).
   - In `main`, create one variable in each mode.
   - Print (or `dbg!`) the three variables.

2. **Traffic permission (no data + rename pressure)**

   - Declare an enum for a 3-state permission/decision system.
   - In `main`, create a variable named `decision` set to the _middle_ state (not first, not last).
   - Add a `match` that prints a different short message for each state.

3. **Coin with value (simple data)**

   - Declare an enum for three coin types where **exactly one** variant stores a numeric “value in cents”.
   - In `main`, initialise:
     - one coin variant that stores no extra data
     - the coin variant that stores the value
   - Use `match` to extract the cents when it exists.

4. **Login result (tuple data)**

   - Declare an enum representing a login attempt outcome:
     - one variant means success and stores a user id (integer)
     - one variant means failure and stores an error code (integer)
     - one variant means “locked” and stores no data
   - In `main`, initialise one value of each variant.

5. **Sensor reading (struct-like variant fields)**

   - Declare an enum with two variants:
     - one represents a temperature reading with **named fields** for “value” and “unit”
     - one represents “not available”
   - In `main`, create a temperature reading in a unit of your choice.

6. **League capacity (same shape across variants)**

   - Declare an enum called `League` with 3 variants.
   - Every variant stores: **player count** and **capacity** (two numbers).
   - In `main`, initialise one variant where player count is less than capacity.
   - Use `match` to bind both numbers and print them.

7. **HTTP-ish response (variant stores a struct)**

   - Declare a struct that represents a response “meta” (at least: status code + message).
   - Declare an enum where one variant stores that struct and another variant stores no data.
   - In `main`, initialise the variant that stores the struct.

8. **Payment method (mix of data + no data)**

   - Declare an enum for payment method with:
     - one variant for cash (no data)
     - one variant for card that stores last 4 digits
     - one variant for bank transfer that stores a reference string
   - In `main`, initialise all three.

9. **User role (enum inside a struct field)**

   - Declare an enum for user roles (3+ roles, no data).
   - Declare a struct `User` with fields: username (string) and role (your enum).
   - In `main`, create a `User` instance with a non-default role.

10. **Delivery status (enum inside a struct, plus data)**

- Declare an enum for a package delivery status where:
  - one variant stores a location string
  - one variant stores a timestamp number
  - one variant stores no data
- Declare a struct `Package` that has an id and a status (your enum).
- In `main`, initialise a `Package` where the status stores a location.

11. **Shape descriptor (tuple variant + struct-like variant)**

- Declare an enum for shapes where:
  - one variant stores width and height
  - one variant stores radius
  - one variant stores named fields for “base” and “height”
- In `main`, initialise one of each.

12. **Message bus (owned vs borrowed data)**

- Declare an enum representing a message where:
  - one variant stores an owned `String`
  - one variant stores a borrowed `&'static str`
  - one variant stores no data
- In `main`, initialise all three and print them.

13. **Game event (variant stores another enum)**

- Declare an enum `Direction` with 4 directions (no data).
- Declare an enum `Event` where one variant stores a `Direction`, and at least one other variant stores no data.
- In `main`, initialise an `Event` that stores a `Direction`.

14. **File operation (named fields in enum variant)**

- Declare an enum representing an operation on a file:
  - one variant stores named fields: path and bytes_count
  - one variant stores named fields: path and reason
  - one variant stores no data
- In `main`, initialise the variant with path and bytes_count.

15. **Discount rule (Option-like without using Option)**

- Declare an enum that represents “has discount” vs “no discount”.
- The discount variant stores a numeric percent.
- In `main`, initialise both cases and use `match` to print the percent only when present.

16. **Parse outcome (Result-like without using Result)**

- Declare an enum representing either:
  - a successfully parsed number (stores an integer)
  - or a failure (stores an error message)
- In `main`, create one of each and match to print either the number or the message.

17. **Complex variant data: struct inside enum + enum inside struct**

- Declare an enum `Priority` (low/medium/high).
- Declare a struct `Task` with fields: title (string) and priority (`Priority`).
- Declare an enum `QueueItem` where one variant stores a `Task` and another variant stores no data.
- In `main`, initialise a `QueueItem` that stores a `Task` with high priority.

18. **Inventory item (enum variant stores multiple different types)**

- Declare an enum for inventory items where:
  - one variant stores a sku (string) and quantity (number)
  - one variant stores a sku (string) and a “backorder expected days” (number)
  - one variant stores no data
- In `main`, initialise the “backorder” variant.

19. **API endpoint selection (enum drives behaviour)**

- Declare an enum for selecting an API endpoint where:
  - one variant stores a base url (string)
  - one variant stores a region code (2–3 letters)
  - one variant stores no data and implies “local”
- In `main`, create a value for each variant and match to print what it implies.

20. **State machine snapshot (nested + named fields)**

- Declare an enum `Step` with 4–6 steps (no data).
- Declare a struct `Snapshot` with fields: step (`Step`), attempts (number), and note (string).
- Declare an enum `RunState` where:
  - one variant stores a `Snapshot`
  - one variant stores named fields for “reason” and “step”
- In `main`, initialise both variants (one with `Snapshot`, one with named fields).