use kwindex::*;

#[test]
fn test_matches() {
    let index = KWIndex::new().extend_from_text("hello my Name name name i!s ?nick&");
    let mut c = index.count_matches("nick");
    assert_eq!(c, 1);
    c = index.count_matches("name");
    assert_eq!(c, 2);
    c = index.count_matches("Name");
    assert_eq!(c, 1);
}

#[test]
fn test_len() {
    let index = KWIndex::new().extend_from_text("hello my name i!s ?nick&");
    let mut length = index.len();
    assert_eq!(length, 4);
    let index2 = KWIndex::new().extend_from_text("hello my name is ?nick&");
    length = index2.len();
    assert_eq!(length, 5);
}

#[test]
fn test_is_empty() {
    let not_empty_index = KWIndex::new().extend_from_text("hello my name i!s ?nick&");
    let empty_index = KWIndex::new().extend_from_text("");
    let mut check = not_empty_index.is_empty();
    assert_eq!(check, false);
    check = empty_index.is_empty();
    assert_eq!(check, true);
}
