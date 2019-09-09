use objectbox;

#[test]
fn version() {
    use objectbox::version;

    println!("{}", objectbox::version::info());

    fn verify_version_format(v: version::Version) {
        assert_eq!(
            format!("{}", v),
            format!("{}.{}.{}", v.major, v.minor, v.patch)
        );
    }

    verify_version_format(version::lib());
    verify_version_format(version::rust());
}
