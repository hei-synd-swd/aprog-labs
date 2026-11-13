---
id          = "conc_rc_weak"
name        = "Rc and Weak"
language    = "rust"
difficulty  = 3
description = "Share a value between several owners with Rc and observe it with Weak."
topics      = ["rc", "weak", "reference counting", "strong_count", "upgrade"]
---

# Rc and Weak

A text document is edited in several windows. Every editor window **owns** the document, so it stays in memory as long as one editor is open. A preview window only **observes** it: the preview must not keep a closed document alive.

## Your Task

Replace the three `todo!()`. Do not change the signatures, `main` or the tests.

**`add_owner(document: &Rc<String>) -> Rc<String>`** returns one more owner of the same document.

1. Return `Rc::clone(document)`. Do not create a new `Rc` with `Rc::new`, that would be a second, independent document.

**`observe(document: &Rc<String>) -> Weak<String>`** returns a non-owning handle to the document.

1. Return `Rc::downgrade(document)`.

**`read(observer: &Weak<String>) -> Option<String>`** returns a copy of the text if the document still exists.

1. Call `observer.upgrade()`. It returns an `Option<Rc<String>>`.
2. Turn the `Rc<String>` inside the option into a `String` with `.map(|document| document.to_string())`.
3. Return the resulting `Option<String>`.

```
let document = Rc::new(String::from("hello"));
let editor = add_owner(&document);   // strong count: 2
let preview = observe(&document);    // strong count: 2, weak count: 1

read(&preview)                       → Some("hello")
drop(document); read(&preview)       → Some("hello")    the editor still owns it
drop(editor);   read(&preview)       → None             no owner left
```

## Expected Result

- `add_owner` raises the strong count by one and points to the **same** allocation as the original.
- `observe` raises the weak count by one and leaves the strong count unchanged.
- `read` returns `Some(text)` as long as at least one owner exists, and `None` afterwards.
- `main` prints `strong = 2, weak = 1`, then the text twice, then `None`.
