use super::to_binary;

#[test]
fn test_first_ten() {
    assert_eq!(to_binary(1), "00000001");
    assert_eq!(to_binary(2), "00000010");
    assert_eq!(to_binary(3), "00000011");
    assert_eq!(to_binary(4), "00000100");
    assert_eq!(to_binary(5), "00000101");
    assert_eq!(to_binary(6), "00000110");
    assert_eq!(to_binary(7), "00000111");
    assert_eq!(to_binary(8), "00001000");
    assert_eq!(to_binary(9), "00001001");
    assert_eq!(to_binary(10), "00001010");
}

