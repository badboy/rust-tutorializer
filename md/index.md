## Rust Tutorial

These are the tutorial slides and exercises that were given at **BoosterConf 2017**.
Before you finish, please let me know what you thought at this
[Google form](https://docs.google.com/forms/d/e/1FAIpQLSdpcoeZHXSOmEOyp13KzOSZHoEdieI7UWDB3ulQ-VnnK4WhUw/viewform?usp=sf_link) --
your feedback helps me to improve the tutorials for others. Thanks. -- Jan-Erik


FYI: This page is at <http://home.url>.

[Slides are available online](http://home.url/slides)

Important links:

- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [Community Resources](https://community.rs/resources/)
- [The Rust Programming Language (Book), v1](https://doc.rust-lang.org/stable/book/)
- [The Rust Programming Language (Book), v2 (incomplete)](http://rust-lang.github.io/book/second-edition/index.html)

### Intro

- [Installation](install.html)
- [Hello World](src/hello_world.rs):
    - Time: 3 minutes
- [Mutability](src/mutability.rs):
    - Time: 3 minutes
- [Basic data types](src/data-types.rs):
    - Time: 3 minutes
- [Functions](src/functions.rs):
    - Time: 3 minutes

### Ownership and Borrowing

- [Ownership](src/ownership.rs):
    - Extra credit: Can you do it without copying any data?
    - Time: 10 minutes
- [Shared borrows](src/shared-borrow.rs):
    - Time: 10 minutes
- [Mutable borrows](src/mutable-borrow.rs):
    - [Hint:](hint-mutable-borrow-1.html) Getting the syntax just right can
      be a bit tricky if you've never done any Rust
      before. [If you need a hint, click here](hint-mutable-borrow-1.html).
    - Time: 10 minutes

### Structs and such

- [Structs](src/structs.rs)
    - [Hint: Here is an outline of what
      the function should do, if you get stuck.](hint-struct-1.html)
    - Time: 15 minutes

### Advanced

If you already know Rust, it's time to apply your knowledge.
Below are some ideas what to approach next.

- [Guessing Game](http://rust-lang.github.io/book/second-edition/ch02-00-guessing-game-tutorial.html)
    - A simple interactive game as a hands-on project
- [greprs - An I/O project](http://rust-lang.github.io/book/second-edition/ch12-00-an-io-project.html)
    - Buid a minimal version of a text search tool
- [Mailbox Rust tutorial](https://github.com/skade/mailbox/)
    - Building a small, networked mailbox program which uses threads and concurrency
    - Use the [Rust documentation](https://doc.rust-lang.org/std/) to find necessary methods
- [Rust Cookbook](https://github.com/brson/rust-cookbook)
    - Add examples on how to use Rust crates
    - [Brainstorm list of use cases](https://github.com/brson/rust-cookbook/issues/24)
- Build your own little tool!
    - Automate some task you do daily
