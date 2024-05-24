#![allow(unused)]
use bevy::prelude::*;

use crate::core::Attributes::AttributeChangeEvent;

#[derive(Component, Debug)]
pub struct Level {
    pub level: u8,
    pub xp: u32,
    pub next_level_xp: u32,
}

pub const LEVEL_REQ_XP: [u32; 10] = [60, 100, 150, 200, 350, 625, 920, 1250, 1770, 2500];

impl Level {
    pub fn new(level: u8) -> Self {
        Level {
            level,
            xp: 0,
            next_level_xp: LEVEL_REQ_XP[if level >= LEVEL_REQ_XP.len() as u8 {
                LEVEL_REQ_XP.len() - 1
            } else {
                level as usize
            }],
        }
    }

    pub fn add_xp(&mut self, xp: u32) {
        self.xp += xp;
        if self.xp >= self.next_level_xp {
            self.level += 1;
            self.xp = self.xp - self.next_level_xp;
            self.next_level_xp = LEVEL_REQ_XP[self.level as usize];
        }
        println!("EXP: {:?} LEVEL: {:?}", self.xp, self.level);
    }
}

#[derive(Component, Clone, Debug)]
pub struct SkillPoints {
    pub count: u8,
}

pub fn handle_level_up(
    mut player: Query<(&Level, &mut SkillPoints), Changed<Level>>,
    mut next_level: Local<u8>,
) {
    for (player_level, mut sp) in player.iter_mut() {
        if player_level.level == 1 {
            *next_level = 2;
            return;
        }
        if player_level.level == *next_level {
            sp.count += 1;
            *next_level += 1;
        }
    }
}

// Stats

#[derive(Component, Clone, Debug)]
pub struct Stats {
    pub str: i32,   // strength
    pub dex: i32,   // dexterity
    pub end: i32,   // endurance
    pub int: i32,   // intelligence
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            str: 0,
            dex: 0,
            end: 0,
            int: 0,
        }
    }
}

pub fn send_attribute_event_on_stats_update(
    mut att_event: EventWriter<AttributeChangeEvent>,
        stats: Query<&Stats, Changed<Stats>>,
) {
    if stats.get_single().is_ok() {
        att_event.send(AttributeChangeEvent);
    }
}