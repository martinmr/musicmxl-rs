mod tests;

use serde::{Deserialize, Serialize};
use serde_repr::*;

include!("spec/data-types.rs");
include!("spec/score-partwise.rs");
