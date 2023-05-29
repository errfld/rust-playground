struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Cuboid {
    length: u32,
    width: u32,
    height: u32,
}

impl Cuboid {
    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }
    fn surface(&self) -> u32 {
        2 * self.length * self.height + 2 * self.length * self.width + 2 * self.height * self.width
    }
    fn scale(&mut self, factor: u32) -> &mut Self {
        self.length *= factor;
        self.height *= factor;
        self.width *= factor;
        self
    }
}

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
        email: user1.email.clone(),
        ..user1 //like spread operator in JS but the borrow of user1.email is given to user2, which invalidates user1.email
    };
    assert_eq!(user1.email, user2.email);
    assert_eq!(user1.active, user2.active);
    assert_eq!(user1.sign_in_count, user2.sign_in_count);
    assert_ne!(user1.username, user2.username);

    let user3 = build_user(
        String::from("some-mail@address.com"),
        String::from("manni1968"),
    );

    assert_eq!(user3.email, String::from("some-mail@address.com"));

    let mut cube1 = Cuboid {
        length: 30,
        height: 10,
        width: 30,
    };

    println!(
        "The volumen of the cuboid is {} volumetric pixels and as method {}",
        volume(&cube1),
        cube1.volume()
    );
    println!("With a surface area of: {}", cube1.surface());
    dbg!(&cube1);
    dbg!(&cube1.scale(2));
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn volume(cuboid: &Cuboid) -> u32 {
    cuboid.length * cuboid.width * cuboid.height
}
