# RUST Playground

This is just a repo to test RUST and get to know the language.

I started working my way through [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) book.

## Current state  

I am at chapter 5 out of 20, which is - no suprise - around 25% already done 🙂

Thus far the content is in an easy-to-comprehend manner formulated and logicly structured. The programming content is not that hard by now, but I guess this is going to change sooner than later. 

Of course are a lot of concepts familiar to other programming languages, aside the strict reference ownership modell, and I feel quite welcomed, thus far. 

The reference ownership model reminds me of objective-c and its reference counting which, I feel - don't have any particular reason - was not that bad, just the compiler did not really care much about the reference counts. 

## Notes/Questions


### struct mutability
[https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html]
Rust doesn’t allow us to mark only certain fields as mutable. 

### while stmt
while has stmt behaviour - which seems odd compared to loop. I guess the condition eval is rather disturbing (always have to push the expression result on the stack, wait for condition eval, and if true pop the result) - I was just wondering and started rambling about it

### create objects with spread operator
the '''..''' operator works basically like in JS but, the memory on the heap ownership for any given property is transfered to the new object. 

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
