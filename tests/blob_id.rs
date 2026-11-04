use agent_vcs::object::BlobId;

#[test]
fn blob_id_matches_git_for_hello() {
    let id = BlobId::of(b"hello\n");
    assert_eq!(
        id.to_hex(),
        "2cf8d83d9ee29543b34a87727421fdecb7e3f3a183d337639025de576db9ebb4"
    );
}
