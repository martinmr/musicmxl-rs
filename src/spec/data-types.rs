#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AboveBelow {
    Above,
    Below,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccidentalValue {
    Sharp,
    Natural,
    Flat,
    DoubleSharp,
    SharpSharp,
    FlatFlat,
    NaturalSharp,
    NaturalFlat,
    QuarterFlat,
    QuarterSharp,
    ThreeQuartersFlat,
    ThreeQuartersSharp,
    SharpDown,
    SharpUp,
    NaturalDown,
    NaturalUp,
    FlatDown,
    FlatUp,
    DoubleSharpDown,
    DoubleSharpUp,
    FlatFlatDown,
    FlatFlatUp,
    ArrowDown,
    ArrowUp,
    TripleSharp,
    TripleFlat,
    SlashQuarterSharp,
    SlashSharp,
    DoubleSlashFlat,
    #[serde(rename = "sharp-1")]
    Sharp1,
    #[serde(rename = "sharp-2")]
    Sharp2,
    #[serde(rename = "sharp-3")]
    Sharp3,
    #[serde(rename = "sharp-5")]
    Sharp5,
    #[serde(rename = "flat-1")]
    Flat1,
    #[serde(rename = "flat-2")]
    Flat2,
    #[serde(rename = "flat-3")]
    Flat3,
    #[serde(rename = "flat-4")]
    Flat4,
    Sori,
    Koron,
    Other,
}

#[derive(Debug, Deserialize_repr, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum AccordionMiddle {
    One = 1,
    Two,
    Three,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArrowDirection {
    Down,
    Left,
    #[serde(rename = "left right")]
    LeftRight,
    Northeast,
    #[serde(rename = "northeast southwest")]
    NortheastSouthwest,
    Northwest,
    #[serde(rename = "northwest southeast")]
    NorthwestSoutheast,
    Other,
    Right,
    Southeast,
    Southwest,
    Up,
    #[serde(rename = "up down")]
    UpDown,
}

// TODO: arrow-style tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArrowStyle {
    Combined,
    Double,
    Filled,
    Hollow,
    Other,
    Paired,
    Single,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BackwardForward {
    Backward,
    Forward,
}

// TODO: bar-style

#[derive(Debug, Deserialize_repr, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum BeamLevel {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

// TODO: beam-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BeamValue {
    #[serde(rename = "backward hook")]
    BackwardHook,
    Begin,
    Continue,
    End,
    #[serde(rename = "forward hook")]
    ForwardHook,
}

// TODO: beater-value

// TODO: bend-shape tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BendShape {
    Angled,
    Curved,
}

// TODO: breath-mark-value
// TODO: caesura-value
// TODO: cancel-location
// TODO: circular-arrow
// TODO: clef-sign
// TODO: color
// TODO: comma-separated-text
// TODO: css-font-size
// TODO: date
// TODO: decimal
// TODO: degree-symbol-value
// TODO: degree-type-value
// TODO: distance-type
// TODO: divisions
// TODO: effect-value
// TODO: enclosure-shape
// TODO: ending-number
// TODO: fan
// TODO: fermata-shape
// TODO: fifths
// TODO: font-family
// TODO: font-size
// TODO: font-style
// TODO: font-weight
// TODO: glass-value
// TODO: glyph-type
// TODO: group-barline-value
// TODO: group-symbol-value
// TODO: handbell-value
// TODO: harmon-closed-location
// TODO: harmon-closed-value
// TODO: harmony-arrangement
// TODO: harmony-type
// TODO: hole-closed-location
// TODO: hole-closed-value
// TODO: ID
// TODO: IDREF
// TODO: integer
// TODO: kind-value
// TODO: left-center-right
// TODO: left-right
// TODO: line-end
// TODO: line-length
// TODO: line-shape
// TODO: line-type
// TODO: line-width-type
// TODO: margin-type
// TODO: measure-numbering-value
// TODO: measure-text
// TODO: membrane-value
// TODO: metal-value
// TODO: midi-128
// TODO: midi-16
// TODO: midi-16384
// TODO: millimeters
// TODO: milliseconds
// TODO: mode
// TODO: mute
// TODO: NMTOKEN
// TODO: non-negative-decimal
// TODO: nonNegativeInteger
// TODO: note-size-type
// TODO: note-type-value
// TODO: notehead-value
// TODO: number-level
// TODO: number-of-lines
// TODO: number-or-normal
// TODO: numeral-mode
// TODO: numeral-value
// TODO: octave
// TODO: on-off
// TODO: over-under
// TODO: pedal-type
// TODO: percent
// TODO: pitched-value
// TODO: positive-divisions
// TODO: positive-integer-or-empty
// TODO: positiveInteger
// TODO: principal-voice-symbol
// TODO: right-left-middle
// TODO: rotation-degrees
// TODO: semi-pitched
// TODO: semitones
// TODO: show-frets
// TODO: show-tuplet
// TODO: smufl-accidental-glyph-name
// TODO: smufl-coda-glyph-name
// TODO: smufl-glyph-name
// TODO: smufl-lyrics-glyph-name
// TODO: smufl-pictogram-glyph-name
// TODO: smufl-segno-glyph-name
// TODO: smufl-wavy-line-glyph-name
// TODO: staff-divide-symbol
// TODO: staff-line
// TODO: staff-line-position
// TODO: staff-number
// TODO: staff-type
// TODO: start-note
// TODO: start-stop
// TODO: start-stop-continue
// TODO: start-stop-discontinue
// TODO: start-stop-single
// TODO: stem-value
// TODO: step
// TODO: stick-location
// TODO: stick-material
// TODO: stick-type
// TODO: string
// TODO: string-number
// TODO: swing-type-value
// TODO: syllabic
// TODO: symbol-size
// TODO: sync-type
// TODO: system-relation
// TODO: system-relation-number
// TODO: tap-hand
// TODO: tenths
// TODO: text-direction
// TODO: tied-type
// TODO: time-only
// TODO: time-relation
// TODO: time-separator
// TODO: time-symbol
// TODO: tip-direction
// TODO: token
// TODO: top-bottom
// TODO: tremolo-marks
// TODO: tremolo-type
// TODO: trill-beats
// TODO: trill-step
// TODO: two-note-turn
// TODO: up-down
// TODO: up-down-stop-continue
// TODO: upright-inverted
// TODO: valign
// TODO: valign-image
// TODO: wedge-type
// TODO: winged
// TODO: wood-value
// TODO: xlink:actuate
// TODO: xlink:show
// TODO: xlink:type
// TODO: xml:lang
// TODO: xml:space
// TODO: yes-no
// TODO: yes-no-number
// TODO: yyyy-mm-dd
