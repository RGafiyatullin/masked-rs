
use crate::*;

#[test]
fn display_should_not_disclose_the_value() {
    let string = "very-much-sensitive";
    let masked = string.masked();
    let display_string = format!("{:?}", string);
    let display_masked = format!("{:?}", masked);

    assert_eq!(display_string.find(string), Some(1));
    assert_eq!(display_masked.find(string), None);
}
