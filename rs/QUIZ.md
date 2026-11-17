# 🧠 Rust Theory Quiz

30 quick questions. Answer on paper first, then click to check.

> Tip: for "does this compile?" questions, actually run the snippet in a
> scratch file (`cargo new scratch && cd scratch`). Being wrong is the lesson.

---

## 1. Basics

**1. Which binding can be changed after creation?**

```rust
let a = 5;
let mut b = 5;
```

<details><summary>Answer</summary>

`b`. `let` bindings are immutable by default; `mut` opts into mutation.
`a` can still be *shadowed* by a new `let a = ...`, but that creates a new
variable.

</details>

**2. What does this print?**

```rust
let x = 5;
let x = x * 2;
let x = "now a str";
println!("{x}");
```

<details><summary>Answer</summary>

`now a str`. Shadowing creates a new `x` each time, and it may change type.
The old values are still dropped normally at the end of the scope.

</details>

**3. Does this compile?**

```rust
const LIMIT: u32 = 10;
let x = LIMIT;
```

<details><summary>Answer</summary>

Yes. `LIMIT` is a compile-time constant with an explicit type, usable anywhere
a `u32` is expected.

</details>

**4. What happens in a debug build?**

```rust
let x: u8 = 255;
let y = x + 1;
```

<details><summary>Answer</summary>

It panics: `attempt to add with overflow`. In release mode, overflow wraps
silently to `0` by default. Use `checked_add`, `wrapping_add` or `saturating_add`
when it matters.

</details>

**5. Why is `let x = 5;` sometimes `i32` and sometimes something else?**

<details><summary>Answer</summary>

Integer literals are polymorphic. The compiler infers the type from use, and
falls back to `i32` when there is no other evidence. An explicit suffix
(`5u8`) or annotation (`let x: u8 = 5`) picks another type.

</details>

---

## 2. Ownership & borrowing

**6. Does this compile?**

```rust
let s = String::from("hi");
let t = s;
println!("{s}");
```

<details><summary>Answer</summary>

No: "borrow of moved value: `s`". Assigning a `String` moves ownership to `t`;
`s` is no longer valid. Use `t = s.clone()` or `t = &s` instead.

</details>

**7. Does this compile?**

```rust
let s = String::from("hi");
let t = &s;
println!("{s} {t}");
```

<details><summary>Answer</summary>

Yes. Any number of immutable (`&`) borrows may exist at the same time. Only
*mutable* borrows must be unique.

</details>

**8. How many mutable borrows of one value can exist at once?**

<details><summary>Answer</summary>

Exactly one. While a `&mut` is alive, no other `&` or `&mut` to that value is
allowed. It is the same rule that prevents data races in safe Rust.

</details>

**9. Does this compile?**

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);
println!("{first}");
```

<details><summary>Answer</summary>

No. `first` still borrows `v` immutably when `push` needs `&mut v` (E0502).
Move the `println!` before `push`, or use `first.copied()` / `v[0]` (an `i32`
is `Copy`) if you only need the value.

</details>

**10. What is the difference between `Clone` and `Copy`?**

<details><summary>Answer</summary>

`Copy` is an implicit, bitwise duplicate that happens on assignment/passing;
the original stays usable. `Clone` is an explicit `.clone()` and may allocate
(`String`). Every `Copy` type is also `Clone`, but not the other way round.

</details>

**11. A function takes `&str`, but you have a `String`. What happens?**

<details><summary>Answer</summary>

Deref coercion: `&my_string` becomes a `&str` automatically at the call site.
That is why `fn first_word(s: &str)` accepts both `"literal"` and `&String`.
Passing by value (`String`) would move it; `&str` only borrows.

</details>

**12. Why does this compile even though the borrow "looks" too long?**

```rust
let mut s = String::from("hi");
let r = &s;
println!("{r}");
s.push('!');
```

<details><summary>Answer</summary>

Non-lexical lifetimes: `r` is last used in `println!`, so its borrow ends there.
The mutable borrow for `push` starts afterwards and therefore does not conflict.
Borrows end at their last use, not at the end of the block.

</details>

---

## 3. Structs, enums & patterns

**13. What does `#[derive(Debug)]` give you?**

<details><summary>Answer</summary>

An implementation of `std::fmt::Debug`, so the type can be printed with
`{:?}` / `{:#?}` and used in `assert_eq!` failure messages. It does not make
the type `Display` (`{}`).

</details>

**14. Does this `match` compile?**

```rust
enum Coin { Heads, Tails }
let c = Coin::Heads;
match c {
    Coin::Heads => println!("heads"),
}
```

<details><summary>Answer</summary>

No: non-exhaustive patterns (E0004), the `Coin::Tails` arm is missing. Add the
arm or a `_ => ...` catch-all. The compiler can prove the match is incomplete.

</details>

**15. What does this print, and why is the `else` branch required?**

```rust
let maybe = Some(3);
let Some(x) = maybe else { return; };
println!("{x}");
```

<details><summary>Answer</summary>

Prints `3`. This is a `let-else`: it binds `x` on success, otherwise the `else`
block must diverge (`return`, `break`, `panic!`, ...). It is handy for early
validation without deep nesting.

</details>

**16. When is `if let` better than `match`?**

<details><summary>Answer</summary>

When you only care about one pattern and want to ignore the rest, e.g.
`if let Some(v) = map.get(key) { ... }`. Use `match` when every case matters
or you need exhaustive handling.

</details>

**17. How do you match a range that also binds the value?**

```rust
match n {
    ??? => println!("teen: {age}"),
    _ => {}
}
```

<details><summary>Answer</summary>

`age @ 13..=19`. The `@` binding stores the matched value while the range
pattern tests it. Guards (`n if n < 0`) are the other common tool.

</details>

---

## 4. Strings & collections

**18. Why does `s[0]` not compile for a `String`?**

<details><summary>Answer</summary>

`String` is UTF-8 and can only be indexed by *byte* ranges that fall on char
boundaries, with `&s[0..1]`. A bare `s[0]` has no indexing impl because a
`char` may be several bytes. Use `s.chars().next()`, `s.as_bytes()[0]` (bytes)
or iterate.

</details>

**19. `"héllo".len()` — is it 5?**

<details><summary>Answer</summary>

No, it is 6 bytes (`é` is two bytes in UTF-8). `.len()` is bytes, `.chars().count()`
is scalar values, and grapheme clusters can be even more complex
(`"é"` may be `e` + combining accent).

</details>

**20. What is the difference between `String` and `&str`?**

<details><summary>Answer</summary>

`String` owns a growable UTF-8 buffer on the heap. `&str` is a borrowed
(`ptr`, `len`) view of UTF-8 bytes owned by someone else, and is `Copy`.
Prefer `&str` in function parameters and `String` for stored/mutable text.

</details>

**21. What is the difference between `.iter()`, `.iter_mut()` and `.into_iter()`?**

<details><summary>Answer</summary>

- `.iter()` → `&T` (shared borrow, collection stays usable)
- `.iter_mut()` → `&mut T` (mutate elements in place)
- `.into_iter()` → `T` (consumes the collection; a `for` loop does this
  implicitly for owned values)

</details>

**22. Why does `v.iter().map(|n| n * 2);` do nothing?**

<details><summary>Answer</summary>

Iterators are lazy. `map` builds an adapter but no values are pulled until a
consumer runs (`sum`, `collect`, `for`, `count`, ...). Rust warns
`unused_must_use` for exactly this reason.

</details>

**23. What happens when the key is missing?**

```rust
let mut m = HashMap::new();
println!("{}", m["missing"]);
```

<details><summary>Answer</summary>

It panics. `Index` assumes the key exists. Use `m.get("missing")` (returns
`Option<&V>`), `entry(...).or_insert(...)` to create it, or `m["k"]` only when
you know it is there.

</details>

**24. How do you increment a `HashMap` counter correctly?**

<details><summary>Answer</summary>

```rust
*counts.entry(word).or_insert(0) += 1;
```

`entry` gives a `&mut V`, so the `*` dereferences the stored `usize`.
`counts[word] += 1` panics for a new key.

</details>

---

## 5. Errors, generics & closures

**25. `unwrap()` vs `?` — when do you use which?**

<details><summary>Answer</summary>

`unwrap()` panics on `None`/`Err` and is fine for prototypes and genuinely
impossible failures. `?` returns the error to the caller, but the enclosing
function must return `Result` (or `Option` for `Option`). Production code
prefers `?` plus context (`map_err`, `with_context`).

</details>

**26. What is the difference between `Option<T>` and `Result<T, E>`?**

<details><summary>Answer</summary>

`Option` models absence (`Some`/`None`) with no explanation. `Result` models
success or a typed failure (`Ok`/`Err`) that can carry a message or error type.

</details>

**27. Why does `fn largest<T>(list: &[T]) -> &T` fail to compile?**

<details><summary>Answer</summary>

`T` has no capabilities. Comparing values needs a bound:
`fn largest<T: PartialOrd>(list: &[T]) -> &T`. With `PartialOrd`, `&T` values
can be compared using the operators.

</details>

**28. Do lifetime annotations change how long a value lives?**

<details><summary>Answer</summary>

No. Lifetimes never create, extend or shorten values. They only describe
relationships the compiler must verify. If a value dies too early, the fix is
usually to return an owned value or restructure the code — not to "add a
lifetime".

</details>

**29. `Fn`, `FnMut`, `FnOnce` — what is the difference?**

<details><summary>Answer</summary>

- `Fn`: can be called many times, captures by shared reference.
- `FnMut`: can be called many times, may mutate captured state.
- `FnOnce`: can be called once, consumes captured values.

Every `Fn` is also `FnMut` and `FnOnce`; the reverse is not true.

</details>

**30. What does a `move` closure capture?**

<details><summary>Answer</summary>

It captures the *used* variables by value (moving them in), instead of
borrowing. That is how closures can outlive the current scope, e.g. be sent to
another thread. After the closure, the original variables are no longer
usable unless they are `Copy` or were cloned first.

</details>
