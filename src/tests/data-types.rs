// TODO: automate these tests so they are easier to write.
#[allow(unused)]
struct DataTypeTest<'a, T>
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    cases: Vec<(Option<T>, String)>,
    phantom: PhantomData<&'a T>,
}

#[allow(unused)]
impl<'a, T> DataTypeTest<'a, T>
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    fn run_tests(&self) {
        for (expected, input) in self.cases.iter() {
            // Test deserialization.
            let de_result = serde_plain::from_str::<T>(input.as_str());
            match expected {
                None => assert!(de_result.is_err()),
                Some(val) => assert_eq!(*val, de_result.unwrap()),
            }

            // Test serialization.
            if expected.is_some() {
                let se_result = serde_plain::to_string(expected.as_ref().unwrap());
                assert_eq!(*input, se_result.unwrap());
            }
        }
    }
}

#[test]
fn above_below() {
    let tests = DataTypeTest::<super::AboveBelow> {
        cases: vec![
            (Some(super::AboveBelow::Above), "above".to_string()),
            (Some(super::AboveBelow::Below), "below".to_string()),
        ],
        phantom: PhantomData,
    };
    tests.run_tests();
}

#[test]
fn accidental() {
    let tests = DataTypeTest::<super::AccidentalValue> {
        cases: vec![
            (Some(super::AccidentalValue::Sharp), "sharp".to_string()),
            (Some(super::AccidentalValue::Natural), "natural".to_string()),
            (Some(super::AccidentalValue::Flat), "flat".to_string()),
            (
                Some(super::AccidentalValue::DoubleSharp),
                "double-sharp".to_string(),
            ),
            (
                Some(super::AccidentalValue::SharpSharp),
                "sharp-sharp".to_string(),
            ),
            (
                Some(super::AccidentalValue::FlatFlat),
                "flat-flat".to_string(),
            ),
            (
                Some(super::AccidentalValue::NaturalSharp),
                "natural-sharp".to_string(),
            ),
            (
                Some(super::AccidentalValue::NaturalFlat),
                "natural-flat".to_string(),
            ),
        ],
        phantom: PhantomData,
    };
    tests.run_tests();
}

#[test]
fn accordion_middle() {
    let tests = DataTypeTest::<super::AccordionMiddle> {
        cases: vec![
            (None, "0".to_string()),
            (Some(super::AccordionMiddle::One), "1".to_string()),
            (Some(super::AccordionMiddle::Two), "2".to_string()),
            (Some(super::AccordionMiddle::Three), "3".to_string()),
            (None, "4".to_string()),
        ],
        phantom: PhantomData,
    };
    tests.run_tests();
}

#[test]
fn arrow_direction() {
    let tests = DataTypeTest::<super::ArrowDirection> {
        cases: vec![
            (Some(super::ArrowDirection::Down), "down".to_string()),
            (Some(super::ArrowDirection::Left), "left".to_string()),
            (
                Some(super::ArrowDirection::LeftRight),
                "left right".to_string(),
            ),
            (
                Some(super::ArrowDirection::Northeast),
                "northeast".to_string(),
            ),
            (
                Some(super::ArrowDirection::NortheastSouthwest),
                "northeast southwest".to_string(),
            ),
            (
                Some(super::ArrowDirection::Northwest),
                "northwest".to_string(),
            ),
            (
                Some(super::ArrowDirection::NorthwestSoutheast),
                "northwest southeast".to_string(),
            ),
            (Some(super::ArrowDirection::Other), "other".to_string()),
            (Some(super::ArrowDirection::Right), "right".to_string()),
            (
                Some(super::ArrowDirection::Southeast),
                "southeast".to_string(),
            ),
            (
                Some(super::ArrowDirection::Southwest),
                "southwest".to_string(),
            ),
            (Some(super::ArrowDirection::Up), "up".to_string()),
            (Some(super::ArrowDirection::UpDown), "up down".to_string()),
        ],
        phantom: PhantomData,
    };
    tests.run_tests();
}

#[test]
fn backward_foward() {
    let tests = DataTypeTest::<super::BackwardForward> {
        cases: vec![
            (
                Some(super::BackwardForward::Backward),
                "backward".to_string(),
            ),
            (Some(super::BackwardForward::Forward), "forward".to_string()),
        ],
        phantom: PhantomData,
    };
    tests.run_tests();
}

#[test]
fn beam_level() {
    let tests = DataTypeTest::<super::BeamLevel> {
        cases: vec![
            (None, "0".to_string()),
            (Some(super::BeamLevel::One), "1".to_string()),
            (Some(super::BeamLevel::Two), "2".to_string()),
            (Some(super::BeamLevel::Three), "3".to_string()),
            (Some(super::BeamLevel::Four), "4".to_string()),
            (Some(super::BeamLevel::Five), "5".to_string()),
            (Some(super::BeamLevel::Six), "6".to_string()),
            (Some(super::BeamLevel::Seven), "7".to_string()),
            (Some(super::BeamLevel::Eight), "8".to_string()),
            (None, "9".to_string()),
        ],
        phantom: PhantomData,
    };
    tests.run_tests();
}
