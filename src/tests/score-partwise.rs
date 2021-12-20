#[test]
fn movement_number() {
    let xml = "<movement-number>number</movement-number>";
    assert_eq!(
        super::MovementNumber {
            content: "number".to_string()
        },
        from_str(xml).unwrap()
    )
}

#[test]
fn movement_title() {
    let xml = "<movement-title>title</movement-title>";
    assert_eq!(
        super::MovementTitle {
            content: "title".to_string()
        },
        from_str(xml).unwrap()
    )
}
