mod tests;

use serde::{Serialize, Deserialize};
use serde_repr::*;

include!("spec/data-types.rs");
include!("spec/score-partwise.rs");
