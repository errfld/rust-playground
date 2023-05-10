# RUST Playground

This is just a repo to test RUST and get to know the language.

I started working my way through [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) book.

## Current state

I am at chapter 5 out of 20, which is - no surprise - around 25% already done 🙂

Thus far the content is in an easy-to-comprehend manner formulated and logically structured. The programming content is not that hard by now, but I guess this is going to change sooner than later.

Of course are a lot of concepts familiar to other programming languages, aside the strict reference ownership model, and I feel quite welcomed, thus far.

The reference ownership model reminds me of objective-c and its reference counting which, I feel - don't have any particular reason - was not that bad, just the compiler did not really care much about the reference counts.

## Notes/Questions

### struct mutability

[https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html]
Rust doesn’t allow us to mark only certain fields as mutable.

### while stmt

while has stmt behavior - which seems odd compared to loop. I guess the condition eval is rather disturbing (always have to push the expression result on the stack, wait for condition eval, and if true pop the result) - I was just wondering and started rambling about it

### create objects with spread operator

the '''..''' operator works basically like in JS but, the memory on the heap ownership for any given property is transferred to the new object.

I get the error "error[E0382]: borrow of moved value: `user1.email`" on following rust code:

```
fn main() {
    let mut user1 = User {
        username: String::from("eran_riesenfeld"),
        email: String::from("eran.riesenfeld@someaddress.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("changed-my@mail.address");

    assert_eq!(user1.email, String::from("changed-my@mail.address"));

    let user2 = User {
        username: String::from("mr_darcy"),
        ..user1 //like spread operator in JS
    };
    assert_eq!(user1.email, user2.email);
    assert_eq!(user1.active, user2.active);
    assert_ne!(user1.username, user2.username);
    assert_eq!(user1.sign_in_count, user2.sign_in_count);
}
```

Answer from ChatGippety:

The error you're seeing occurs because Rust doesn't allow multiple owners of the same memory unless explicitly specified. In this case, `user1` owns the memory for `email` on the heap.

When you create `user2` using the syntax `..user1`, Rust tries to move the ownership of `user1.email` to `user2.email`, which is not allowed. This means that `user1.email` is no longer valid and cannot be accessed after this line of code. So when you try to access `user1.email` in the assertion, you're trying to access a value that has been moved and no longer exists.

To fix this, you can clone the `email` value and create a new instance of it for `user2` like this:

```
let user2 = User {
    email: user1.email.clone(),
    ..user1
};
```

This way, both `user1` and `user2` will own their own memory for `email`, and you'll be able to access both of them without any errors.

### Automatic (de-)referencing

Whyyyy? 🥺 I really thought we had a good thing going on here. Well, tbh. it is not that bad, but I feel right now a little bit perplexed. The language feels very concise (if that is the right term in english), meaning explicit in most statement aside from type inference on variables, the compiler does not even infer return types on functions. The c-like -> dereference operator is quite superfluous tbh, but I feel tricked ;-) perhaps in a good way. At least there are no "messages" with [object method_name: param] 🙂

### Modules Cheat Sheet

My version of the [Cheat Sheet](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

- **Start from the crate root:** root files: src/lib.rs and src/main.rs
- **Declaring modules:** `mod x;` will search for the following:
  1. Inline: using `mod x {...} `
  1. file: `src/x.rs`
  1. file: `src/x/mod.rs`
- **Submodules:** No surprise uses the same logic as modules, but the module as `root``
- **Visibility:** Modules are always private, they have to be marked with `pub`
- **use:** Modules are imported using the `use` keyword. Seems to be the same Idea as in Java.

Restaurant example, module tree:

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### unused code on example

In [7.2](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) I had some unused code, and the compiler would kindly remind me of that --- always!
While this is perfect in regular code, I decided to deactivate this warning with `#[allow(dead_code)]`. I could have deactivated it for the complete crate with `#![allow(dead_code)]`.

### blanket implementation

[10.2](https://doc.rust-lang.org/stable/book/ch10-02-traits.html) is awesome, especially blanket implementations. I did an implementation for a `largest` function on any Vec. I used the examples from the book and played around with the traits and generics.

```rust
pub trait Wiener<T> {
    fn largest(&self) -> &T;
}

//Blanket implementation
impl<T: std::cmp::PartialOrd> Wiener<T> for Vec<T> {
    fn largest(&self) -> &T {
        let mut largest = &self[0];
        for item in self {
            if item > largest {
                largest = item
            }
        }
        largest
    }
}
```
