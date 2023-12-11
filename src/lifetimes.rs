fn get_longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// "export const fn = () => {}" equivalent in rust
pub fn test_lifetimes() {
    let str1 = String::from("a very loooong string");
    {
        let str2 = String::from("short string");
        let longest = get_longest(&str1, &str2);
        println!("the longest one is: '{longest}'");
    }

    // // a fail example below:
    // let longest: &str;
    // {
    //     let str2 = String::from("short string");
    //     longest = get_longest(&str1, &str2); // here the rustc will complaint: `str2` does not live long enough, borrowed value does not live long enough
    // }
    // println!("the longest one is: '{longest}'");
}
