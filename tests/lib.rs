#[macro_use]
extern crate neko;

#[test]
fn test_macros_git() {
    assert_eq!(
        account_at_rep!("https://github.com/Arukana/libnya.git"),
        Some(
            String::from("arukana@libnya"),
        )
    );
    assert_eq!(
        account_at_rep!("git@github.com:Arukana/libnya.git"),
        Some(
            String::from("arukana@libnya"),
        )
    );
    assert_eq!(
        account_at_rep!("git@github.com:Arukana/n.git"),
        Some(
            String::from("arukana@n"),
        )
    );
}

#[test]
fn test_macros_git_unvalid() {
    assert_eq!(
        account_at_rep!("https://github.com/Arukana/.git"),
        None
    );
    assert_eq!(
        account_at_rep!("https://github.com/Arukana/git"),
        None
    );
    assert_eq!(
        account_at_rep!("https://github.com/.git"),
        None
    );
    assert_eq!(
        account_at_rep!("git@github.com:Arukana/.git"),
        None
    );
    assert_eq!(
        account_at_rep!("git@github.com:Arukana/"),
        None
    );
    assert_eq!(
        account_at_rep!("git@github.com:Arukana"),
        None
    );
    assert_eq!(
        account_at_rep!("libnya.git"),
        None
    );
    assert_eq!(
        account_at_rep!(".git"),
        None
    );
    assert_eq!(
        account_at_rep!("."),
        None
    );
    assert_eq!(
        account_at_rep!(""),
        None
    );
}
