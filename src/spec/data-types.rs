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
// See https://docs.rs/serde-aux/latest/serde_aux/field_attributes/fn.deserialize_vec_from_string_or_vec.html

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

pub type DistanceType = String;

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

pub type GlichType = String;

// TODO: group-barline-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupBarlineValue {
    Yes,
    No,
    #[serde(rename = "Mensurstrich")]
    Mensurstrich,
}

// TODO: group-symbol-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupSymbolValue {
    Brace,
    Bracket,
    Line,
    None,
    Square,
}

// TODO: handbell-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HandbellValue {
    Belltree,
    Damp,
    Echo,
    Gyro,
    #[serde(rename = "hand martellato")]
    HandMartellato,
    #[serde(rename = "mallet lift")]
    MalletLift,
    #[serde(rename = "mallet table,")]
    MalletTable,
    Martellato,
    #[serde(rename = "martellato lift")]
    MartellatoLift,
    #[serde(rename = "muted martellato")]
    MutedMartellato,
    #[serde(rename = "pluck lift")]
    PluckLift,
    Swing,
}

// TODO: harmon-closed-location tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HarmonClosedLocation {
    Bottom,
    Left,
    Right,
    Top,
}

// TODO: harmon-closed-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HarmonClosedValue {
    Yes,
    No,
    Half,
}

// TODO: harmony-arrangement tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HarmonyArrangement {
    Horizontal,
    Vertical,
    Diagonal,
}

// TODO: harmony-type tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HarmonyType {
    Alternate,
    Explicit,
    Implied,
}

// TODO: hole-closed-location tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HoleClosedLocation {
    Bottom,
    Left,
    Right,
    Top,
}

// TODO: hole-closed-value
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HoleClosedValue {
    Yes,
    No,
    Half,
}

// TODO: ID

// TODO: IDREF

// TODO: integer

// TODO: kind-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum KindValue {
    Augmented,
    AugmentedSeventh,
    Diminished,
    DiminishedSeventh,
    Dominant,
    #[serde(rename = "dominant-11th")]
    Dominant11th,
    #[serde(rename = "dominant-13th")]
    Dominant13th,
    DominantNinth,
    #[serde(rename = "French")]
    French,
    #[serde(rename = "German")]
    German,
    HalfDiminished,
    #[serde(rename = "Italian")]
    Italian,
    Major,
    #[serde(rename = "major-11th")]
    Major11th,
    #[serde(rename = "major-13th")]
    Major13th,
    MajorMinor,
    MajorNinth,
    MajorSeventh,
    MajorSixth,
    Minor,
    #[serde(rename = "minor-11th")]
    Minor11th,
    #[serde(rename = "minor-13th")]
    Minor13th,
    MinorNinth,
    MinorSeventh,
    MinorSixth,
    #[serde(rename = "Neapolitan")]
    Neapolitan,
    None,
    Other,
    Pedal,
    Power,
    SuspendedFourth,
    SuspendedSecond,
    #[serde(rename = "Tristan")]
    Tristan,
}

// TODO: left-center-right test
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LeftCenterRight {
    Left,
    Center,
    Right,
}

// TODO: left-right tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LeftRight {
    Left,
    Right,
}

// TODO: line-end tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LineEnd {
    Up,
    Down,
    Both,
    Arrow,
    None,
}

// TODO: line-length tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LineLength {
    Short,
    Medium,
    Long,
}

// TODO: line-shape tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LineShape {
    Straight,
    Curved,
}

// TODO: line-type tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LineType {
    Dashed,
    Dotted,
    Solid,
    Wavy,
}

pub type LineWidthType = String;

// TODO: margin-type tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MarginType {
    Both,
    Even,
    Odd,
}

// TODO: measure-numbering-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MeasureNumberingValue {
    None,
    Measure,
    System,
}

pub type MeasureText = String;

// TODO: membrane-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MembraneValue {
    #[serde(rename = "bass drum")]
    BassDrum,
    #[serde(rename = "bass drum on side")]
    BassDrumOnSide,
    Bongos,
    #[serde(rename = "Chinese tomtom")]
    ChineseTomtom,
    #[serde(rename = "conga drum")]
    CongaDrum,
    Cuica,
    #[serde(rename = "goblet drum")]
    GobletDrum,
    #[serde(rename = "Indo-American tomtom")]
    IndoAmericanTomtom,
    #[serde(rename = "Japanese tomtom")]
    JapaneseTomtom,
    #[serde(rename = "military drum")]
    MilitaryDrum,
    #[serde(rename = "snare drum")]
    SnareDrum,
    #[serde(rename = "snare drum snares off")]
    SnareDrumSnaresOff,
    Tabla,
    Tambourine,
    #[serde(rename = "tenor drum")]
    TenorDrum,
    Timbales,
    Tomtom,
}

// TODO: metal-value tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MetalValue {
    Agogo,
    Almglocken,
    Bell,
    #[serde(rename = "bell plate")]
    BellPlate,
    #[serde(rename = "bell tree")]
    BellTree,
    #[serde(rename = "brake drum")]
    BrakeDrum,
    Cencerro,
    #[serde(rename = "chain rattle")]
    ChainRattle,
    #[serde(rename = "Chinese cymbal")]
    ChineseCymbal,
    Cowbell,
    #[serde(rename = "crash cymbals")]
    CrashSymbals,
    Crotale,
    #[serde(rename = "cymbal tongs")]
    CymbalTongs,
    #[serde(rename = "domed gong")]
    DomedGong,
    #[serde(rename = "finger cymbals")]
    FingerCymbals,
    Flexatone,
    Gong,
    Handbell,
    #[serde(rename = "hi-hat")]
    HiHat,
    #[serde(rename = "high-hat cymbals")]
    HighHatCymbals,
    #[serde(rename = "jaw harp")]
    JawHarp,
    #[serde(rename = "jingle bells")]
    JingleBells,
    #[serde(rename = "musical saw")]
    MusicalSaw,
    #[serde(rename = "shell bells")]
    ShellBells,
    Sistrum,
    #[serde(rename = "sizzle cymbal")]
    SizzleCymbal,
    #[serde(rename = "sleigh bells")]
    SleighBells,
    #[serde(rename = "suspended cymbal")]
    SuspendedCymbal,
    #[serde(rename = "tam tam")]
    TamTam,
    #[serde(rename = "tam tam with beater")]
    TamTamWithBeater,
    Triangle,
    #[serde(rename = "Vietnamese hat")]
    VietnameseHat,
}

// TODO: midi-128

// TODO: midi-16

// TODO: midi-16384

// TODO: millimeters

// TODO: milliseconds

pub type Mode = String;

// TODO: mute tests
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mute {
    On,
    Off,
    Bucket,
    Cup,
    Echo,
    HarmonNoStem,
    HarmonStem,
    Hat,
    Palm,
    Plunger,
    Practice,
    Solotone,
    StopHand,
    StopMute,
    Straight,
}

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
