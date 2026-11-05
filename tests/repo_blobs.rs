use agent_vcs::blob::BlobId;
use agent_vcs::repo::Repo;

#[test]
fn stored_blob_is_no_longer_missing() {
    let dir = tempfile::tempdir().unwrap();
    let repo = Repo::open(dir.path()).unwrap();
    let hello = BlobId::of(b"hello\n");

    assert_eq!(repo.missing(&[hello]), vec![hello]);

    let id = repo.put_blob(&b"hello\n"[..]).unwrap(); // Can fail return as a Result
    assert_eq!(
        id.to_hex(),
        "2cf8d83d9ee29543b34a87727421fdecb7e3f3a183d337639025de576db9ebb4"
    );
    assert_eq!(repo.missing(&[hello]), vec![]);
}
