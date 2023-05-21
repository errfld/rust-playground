//write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
fn main() {
    let s = String::from("Hello my dear");
    let first_word_position = first_pos(&s);
    let first_word = first_word(&s);
    //s.clear(); does not work because first word returns a slice of s
    println!("First word is {first_word} at {first_word_position}");

    let a = [1, 2, 3, 4, 5];
    let slice_of_a = &a[1..3];
    assert_eq!(slice_of_a, &[2, 3]); //that is a bit confusing for me:
    assert_eq!(*slice_of_a, [2, 3]);
    //assert_eq!(&[2, 3], &[2, 3]);
    //slice_of_a is a reference to index 1 of array a with a length of two.
    //I would expect, that slice_of_a is a ptr
    //but slice_of_a seems to be equal to the reference on the array contains 2 and 3
    //is there some automatic deref happening?
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

fn first_pos(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
