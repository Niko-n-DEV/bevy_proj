#![allow(unused)]
use bevy::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CraftResult {
    pub output: String,
    pub count:  usize,
}