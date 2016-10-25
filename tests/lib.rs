#[macro_use]
extern crate neko;

#[test]
fn test_macros_git() {
    assert_eq!(
        parse_name!("https://github.com/Arukana/libnya.git"),
        Some((
            String::from("arukana@libnya"),
            String::from("arukana"),
            String::from("libnya")
        ))
    );
    assert_eq!(
        parse_name!("git@github.com:Arukana/libnya.git"),
        Some((
            String::from("arukana@libnya"),
            String::from("arukana"),
            String::from("libnya")
        ))
    );
    assert_eq!(
        parse_name!("git@github.com:Arukana/n.git"),
        Some((
            String::from("arukana@n"),
            String::from("arukana"),
            String::from("n")
        ))
    );
}


#[test]
fn test_macros_git_unvalid() {
    assert_eq!(
        parse_name!("https://github.com/Arukana/.git"),
        None
    );
    assert_eq!(
        parse_name!("https://github.com/Arukana/git"),
        None
    );
    assert_eq!(
        parse_name!("https://github.com/.git"),
        None
    );
    assert_eq!(
        parse_name!("git@github.com:Arukana/.git"),
        None
    );
    assert_eq!(
        parse_name!("git@github.com:Arukana/"),
        None
    );
    assert_eq!(
        parse_name!("git@github.com:Arukana"),
        None
    );
    assert_eq!(
        parse_name!("libnya.git"),
        None
    );
    assert_eq!(
        parse_name!(".git"),
        None
    );
    assert_eq!(
        parse_name!("."),
        None
    );
    assert_eq!(
        parse_name!(""),
        None
    );
}
