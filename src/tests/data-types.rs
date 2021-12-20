#[test]
fn above_below() {
    assert_eq!(super::AboveBelow::Above, from_str("above").unwrap());
    assert_eq!(super::AboveBelow::Below, from_str("below").unwrap());
}

#[test]
fn accidental() {
    assert_eq!(super::AccidentalValue::Sharp, from_str("sharp").unwrap());
    assert_eq!(
        super::AccidentalValue::Natural,
        from_str("natural").unwrap()
    );
    assert_eq!(super::AccidentalValue::Flat, from_str("flat").unwrap());
    assert_eq!(
        super::AccidentalValue::DoubleSharp,
        from_str("double-sharp").unwrap()
    );
    assert_eq!(
        super::AccidentalValue::SharpSharp,
        from_str("sharp-sharp").unwrap()
    );
    assert_eq!(
        super::AccidentalValue::FlatFlat,
        from_str("flat-flat").unwrap()
    );
    assert_eq!(
        super::AccidentalValue::NaturalSharp,
        from_str("natural-sharp").unwrap()
    );
    assert_eq!(
        super::AccidentalValue::NaturalFlat,
        from_str("natural-flat").unwrap()
    );
}

#[test]
fn accordion_middle() {
    assert!(from_str::<super::AccordionMiddle>("0").is_err());
    assert_eq!(super::AccordionMiddle::One, from_str("1").unwrap());
    assert_eq!(super::AccordionMiddle::Three, from_str("3").unwrap());
    assert!(from_str::<super::AccordionMiddle>("4").is_err());
}

#[test]
fn arrow_direction() {
    assert_eq!(super::ArrowDirection::Down, from_str("down").unwrap());
    assert_eq!(super::ArrowDirection::Left, from_str("left").unwrap());
    assert_eq!(
        super::ArrowDirection::LeftRight,
        from_str("left right").unwrap()
    );
    assert_eq!(
        super::ArrowDirection::Northeast,
        from_str("northeast").unwrap()
    );
    assert_eq!(
        super::ArrowDirection::NortheastSouthwest,
        from_str("northeast southwest").unwrap()
    );
    assert_eq!(
        super::ArrowDirection::Northwest,
        from_str("northwest").unwrap()
    );
    assert_eq!(
        super::ArrowDirection::NorthwestSoutheast,
        from_str("northwest southeast").unwrap()
    );
    assert_eq!(super::ArrowDirection::Other, from_str("other").unwrap());
    assert_eq!(super::ArrowDirection::Right, from_str("right").unwrap());
    assert_eq!(
        super::ArrowDirection::Southeast,
        from_str("southeast").unwrap()
    );
    assert_eq!(
        super::ArrowDirection::Southwest,
        from_str("southwest").unwrap()
    );
    assert_eq!(super::ArrowDirection::Up, from_str("up").unwrap());
    assert_eq!(super::ArrowDirection::UpDown, from_str("up down").unwrap());
}

#[test]
fn backward_foward() {
    assert_eq!(
        super::BackwardForward::Backward,
        from_str("backward").unwrap()
    );
    assert_eq!(
        super::BackwardForward::Forward,
        from_str("forward").unwrap()
    );
}

#[test]
fn beam_level() {
    assert!(from_str::<super::BeamLevel>("0").is_err());
    assert_eq!(super::BeamLevel::One, from_str("1").unwrap());
    assert_eq!(super::BeamLevel::Eight, from_str("8").unwrap());
    assert!(from_str::<super::BeamLevel>("9").is_err());
}
