use sudoku::board::BitField;

#[test]
fn test_bitfield_set_end() {
    let mut bf = BitField::new(9);
    bf.set(8, 1);
    assert_eq!(bf.get(8), 1);
}

#[test]
fn test_bitfield_set_begin() {
    let mut bf = BitField::new(9);
    bf.set(0, 3);
    assert_eq!(bf.get(0), 3);
}

#[test]
fn test_bitfield_set_after_another_set() {
    let mut bf = BitField::new(9);
    bf.set(1, 3);
    bf.set(6, 8);
    assert_eq!(bf.get(1), 3);
    assert_eq!(bf.get(6), 8);
}

#[test]
fn test_bitfield_set_zero_after_another_set() {
    let mut bf = BitField::new(9);
    bf.set(1, 3);
    bf.set(6, 7);
    bf.set(6, 0);
    assert_eq!(bf.get(6), 0);
}
