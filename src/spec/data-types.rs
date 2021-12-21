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
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BarStyle {
    Dashed,
    Dotted,
    Heavy,
    HeavyHeavy,
    HeavyLight,
    LightHeavy,
    LightLight,
    None,
    Regular,
    Short,
    Tick,
}

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

// TODO: beater-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BeaterValue {
    Bow,
    #[serde(rename = "chime hammer")]
    ChimeHammer,
    Coin,
    #[serde(rename = "drum stick")]
    DrumStick,
    Finger,
    Fingernail,
    Fist,
    #[serde(rename = "guiro scraper")]
    GuiroScraper,
    Hammer,
    Hand,
    #[serde(rename = "jazz stick")]
    JazzStick,
    #[serde(rename = "knitting needle")]
    KnitttingNeedle,
    #[serde(rename = "metal hammer")]
    MetalHammer,
    #[serde(rename = "slide brush on gong")]
    SlideBrushOnGong,
    #[serde(rename = "snare stick")]
    SnareStick,
    #[serde(rename = "spoon mallet")]
    SpoonMallet,
    Superball,
    #[serde(rename = "triangle beater")]
    TriangleBeater,
    #[serde(rename = "triangle beater plain")]
    TriangleBeaterPlain,
    #[serde(rename = "wire brush")]
    WireBrush,
}

// TODO: bend-shape tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BendShape {
    Angled,
    Curved,
}

// TODO: breath-mark-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BreathMarkValue {
    Comma,
    Tick,
    Upbow,
    Salzedo,
}

// TODO: caesura-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CaesuraValue {
    Normal,
    Thick,
    Short,
    Curved,
    Single,
}

// TODO: cancel-location
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CancelLocation {
    Left,
    Right,
    BeforeBarline,
}

// TODO: circular-arrow tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CircularArrow {
    Anticlockwise,
    Clockwise,
}

// TODO: clef-sign tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ClefSign {
    #[serde(rename = "G")]
    G,
    #[serde(rename = "F")]
    F,
    #[serde(rename = "C")]
    C,
    Percussion,
    #[serde(rename = "TAB")]
    Tab,
    Jianpu,
    None,
}

// TODO: color

// TODO: comma-separated-text

// TODO: css-font-size tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CssFontSize {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
}

// TODO: date
// TODO: decimal

// TODO: degree-symbol-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DegreeSymbolValue {
    Major,
    Minor,
    Augmented,
    Diminished,
    HalfDiminished,
}

// TODO: degree-type-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DegreeTypeValue {
    Add,
    Alter,
    Substract,
}

#[allow(unused)]
type DistanceType = String;

// TODO: divisions

// TODO: effect-value
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EffectValue {
    Anvil,
    #[serde(rename = "auto horn")]
    AutoHorn,
    #[serde(rename = "bird whistle")]
    BirdWhistle,
    Cannon,
    #[serde(rename = "duck call")]
    DuckCall,
    #[serde(rename = "gun shot")]
    GunShot,
    #[serde(rename = "klaxon horn")]
    KlaxonHorn,
    #[serde(rename = "lions roar")]
    LionsRoar,
    #[serde(rename = "lotus flute")]
    LotusFlute,
    Megaphone,
    #[serde(rename = "police whistle")]
    PoliceWhistle,
    Siren,
    #[serde(rename = "slice whistle")]
    SlideWhistle,
    #[serde(rename = "thunder sheet")]
    ThunderSheet,
    #[serde(rename = "wind machine")]
    WindMachine,
    #[serde(rename = "wind whistle")]
    WindWhistle,
}

// TODO: enclosure-shape tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EnclosureShape {
    Rectangle,
    Square,
    Oval,
    Circle,
    Bracket,
    InvertedBracket,
    Triangle,
    Diamond,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
    Nonagon,
    Decagon,
    None,
}

// TODO: ending-number

// TODO: fan tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Fan {
    Accel,
    None,
    Rit,
}

// TODO: fermata-shape tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FermataShape {
    Normal,
    Angled,
    Square,
    DoubleAngled,
    DoubleSquare,
    DoubleDot,
    HalfCurve,
    Curlew,
}

// TODO: fifths
// TODO: font-family
// TODO: font-size

// TODO: font-style tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
    Normal,
    Italic,
}

// TODO: font-weight tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FontWeight {
    Normal,
    Bold,
}

// TODO: glass-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum GlassValue {
    #[serde(rename = "glass harmonica")]
    GlassHarmonica,
    #[serde(rename = "glass harp")]
    GlassHarp,
    #[serde(rename = "wind chimes")]
    WindChimes,
}

#[allow(unused)]
type GlichType = String;

// TODO: group-barline-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupBarlineValue {
    Yes,
    No,
    #[serde(rename = "Mensurstrich")]
    Mensurstrich,
}

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
