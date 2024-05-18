#![allow(unused)]
use bevy::{
    prelude::*,
    ecs::system::EntityCommands
    // reflect::{FromReflect, GetTypeRegistration},
    // utils::HashMap,
};
// use serde::{Deserialize, Serialize};
// use std::{fmt::Debug, fmt::Display, hash::Hash, iter::Sum, ops::Add};
// // use strum::IntoEnumIterator;

// // Attribute

// pub trait AttributeType:
//     Component
//     + Clone
//     + Eq
//     + Hash
//     + Display
//     + Default
//     + Reflect
//     + FromReflect
//     + GetTypeRegistration
//     // + IntoEnumIterator
// {}

// #[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, FromReflect, Serialize, Deserialize)]
// #[reflect(Component)]
// pub struct Attributes<A: AttributeType> {
//     pub list: HashMap<A, u8>,
// }
// impl<A: AttributeType> Attributes<A> {
//     pub fn get(&self, attribute_type: &A) -> u8 {
//         *self.list.get(attribute_type).unwrap_or(&0)
//     }

//     pub fn with_all(value: u8) -> Self {
//         Self {
//             list: HashMap::from_iter(A::iter().map(|a| (a, value))),
//         }
//     }
// }

// impl<A: AttributeType> Default for Attributes<A> {
//     fn default() -> Self {
//         Self::with_all(0)
//     }
// }

// impl<A: AttributeType> Add for Attributes<A> {
//     type Output = Attributes<A>;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self {
//             list: HashMap::from_iter(self.list.iter().map(|(t, v)| (t.clone(), *v + rhs.get(t)))),
//         }
//     }
// }
// impl<A: AttributeType> Sum for Attributes<A> {
//     fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
//         iter.fold(Attributes::default(), |acc, a| acc + a)
//     }
// }
// impl<A: AttributeType> Display for Attributes<A> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             self.list
//                 .iter()
//                 .map(|(attribute_type, &amount)| format!("{} +{}", attribute_type, amount))
//                 .fold("".to_string(), |acc, x| format!("{}, {}", x, acc))
//         )
//     }
// }

#[derive(Reflect, Default, Component, Clone, Debug, Copy)]
pub struct CurrentHealth(pub i32);

#[derive(Reflect, Default, Component, Clone, Debug, Copy)]
#[reflect(Component)]
pub struct MaxHealth(pub i32);

#[derive(Reflect, Default, Component, Clone, Debug, Copy)]
#[reflect(Component)]
pub struct Attack(pub i32);

#[derive(Reflect, Default, Component, Clone, Debug, Copy)]
#[reflect(Component)]
pub struct Durability(pub i32);

#[derive(Reflect, Default, Component, Clone, Debug, Copy)]
#[reflect(Component)]

pub struct AttackCooldown(pub f32);
#[derive(Reflect, Default, Component, Clone, Debug, Copy)]
#[reflect(Component)]
pub struct InvincibilityCooldown(pub f32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct CritChance(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct CritDamage(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct BonusDamage(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct HealthRegen(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct Healing(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct Thorns(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct Dodge(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct Speed(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct Lifesteal(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct Defence(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct XpRateBonus(pub i32);

#[derive(Default, Component, Clone, Debug, Copy)]
pub struct LootRateBonus(pub i32);


#[derive(Event, Debug, Clone, Default)]
pub struct AttributeChangeEvent;

pub struct AttributeModifier {
    pub modifier: String,
    pub delta: i32,
}

#[derive(Resource, Default, Bundle)]
pub struct BlockAttributeBundle {
    pub health: CurrentHealth,
}

#[derive(Component, PartialEq, Clone, Reflect, Default, Debug)]
pub struct ItemAttributes {
    pub health: i32,
    pub attack: i32,
    pub durability: i32,
    pub max_durability: i32,
    pub attack_cooldown: f32,
    pub invincibility_cooldown: f32,
    pub crit_chance: i32,
    pub crit_damage: i32,
    pub bonus_damage: i32,
    pub health_regen: i32,
    pub healing: i32,
    pub thorns: i32,
    pub dodge: i32,
    pub speed: i32,
    pub lifesteal: i32,
    pub defence: i32,
    pub xp_rate: i32,
    pub loot_rate: i32,
}

impl ItemAttributes {
    pub fn get_tooltips(&self) -> Vec<String> {
        let mut tooltips: Vec<String> = vec![];
        if self.health != 0 {
            tooltips.push(format!("+{} HP", self.health));
        }
        if self.defence != 0 {
            tooltips.push(format!("+{} Defence", self.defence));
        }
        if self.attack != 0 {
            tooltips.push(format!("+{} DMG", self.attack));
        }
        if self.attack_cooldown != 0. {
            tooltips.push(format!("{:.2} Hits/s", 1. / self.attack_cooldown));
        }
        if self.crit_chance != 0 {
            tooltips.push(format!("+{}% Crit", self.crit_chance));
        }
        if self.crit_damage != 0 {
            tooltips.push(format!("+{}% Crit DMG", self.crit_damage));
        }
        if self.bonus_damage != 0 {
            tooltips.push(format!("+{} DMG", self.bonus_damage));
        }
        if self.health_regen != 0 {
            tooltips.push(format!("+{} HP Regen", self.health_regen));
        }
        if self.healing != 0 {
            tooltips.push(format!("+{}% Healing", self.healing));
        }
        if self.thorns != 0 {
            tooltips.push(format!("+{}% Thorns", self.thorns));
        }
        if self.dodge != 0 {
            tooltips.push(format!("+{}% Dodge", self.dodge));
        }
        if self.speed != 0 {
            tooltips.push(format!("+{}% Speed", self.speed));
        }
        if self.lifesteal != 0 {
            tooltips.push(format!("+{} Lifesteal", self.lifesteal));
        }
        if self.xp_rate != 0 {
            tooltips.push(format!("+{}% XP", self.xp_rate));
        }
        if self.loot_rate != 0 {
            tooltips.push(format!("+{}% Loot", self.loot_rate));
        }

        tooltips
    }

    pub fn get_stats_summary(&self) -> Vec<(String, String)> {
        let mut tooltips: Vec<(String, String)> = vec![];
        tooltips.push(("HP:       ".to_string(), format!("{}", self.health)));
        tooltips.push((
            "Att:      ".to_string(),
            format!("{}", self.attack + self.bonus_damage),
        ));
        tooltips.push(("Defence:  ".to_string(), format!("{}", self.defence)));
        tooltips.push(("Crit:     ".to_string(), format!("{}", self.crit_chance)));
        tooltips.push(("Crit DMG: ".to_string(), format!("{}", self.crit_damage)));
        tooltips.push(("HP Regen: ".to_string(), format!("{}", self.health_regen)));
        tooltips.push(("Healing:  ".to_string(), format!("{}", self.healing)));
        tooltips.push(("Thorns:   ".to_string(), format!("{}", self.thorns)));
        tooltips.push(("Dodge:    ".to_string(), format!("{}", self.dodge)));
        tooltips.push(("Speed:    ".to_string(), format!("{}", self.speed)));
        tooltips.push(("Leech:    ".to_string(), format!("{}", self.lifesteal)));

        // tooltips.push(format!("XP: {}", self.xp_rate));
        // tooltips.push(format!("Loot: {}", self.loot_rate));

        tooltips
    }

    pub fn get_durability_tooltip(&self) -> String {
        format!("{}/{}", self.durability, self.max_durability)
    }

    pub fn add_attribute_components(&self, entity: &mut EntityCommands) {
        if self.health > 0 {
            entity.insert(MaxHealth(self.health));
        }
        if self.attack_cooldown > 0. {
            entity.insert(AttackCooldown(self.attack_cooldown));
        } else {
            entity.remove::<AttackCooldown>();
        }
        entity.insert(Attack(self.attack));
        entity.insert(CritChance(self.crit_chance));
        entity.insert(CritDamage(self.crit_damage));
        entity.insert(BonusDamage(self.bonus_damage));
        entity.insert(HealthRegen(self.health_regen));
        entity.insert(Healing(self.healing));
        entity.insert(Thorns(self.thorns));
        entity.insert(Dodge(self.dodge));
        entity.insert(Speed(self.speed));
        entity.insert(Lifesteal(self.lifesteal));
        entity.insert(Defence(self.defence));
        entity.insert(XpRateBonus(self.xp_rate));
        entity.insert(LootRateBonus(self.loot_rate));
    }

    pub fn change_attribute(&mut self, modifier: AttributeModifier) -> &Self {
        match modifier.modifier.as_str() {
            "health" =>                 self.health += modifier.delta,
            "attack" =>                 self.attack += modifier.delta,
            "durability" =>             self.durability += modifier.delta,
            "max_durability" =>         self.max_durability += modifier.delta,
            "attack_cooldown" =>        self.attack_cooldown += modifier.delta as f32,
            "invincibility_cooldown" => self.invincibility_cooldown += modifier.delta as f32,
            _ => warn!("Got an unexpected attribute: {:?}", modifier.modifier),
        }
        self
    }

    pub fn combine(&self, other: &ItemAttributes) -> ItemAttributes {
        ItemAttributes {
            health:                 self.health + other.health,
            attack:                 self.attack + other.attack,
            durability:             self.durability + other.durability,
            max_durability:         self.max_durability + other.max_durability,
            attack_cooldown:        self.attack_cooldown + other.attack_cooldown,
            invincibility_cooldown: self.invincibility_cooldown + other.invincibility_cooldown,
            crit_chance:            self.crit_chance + other.crit_chance,
            crit_damage:            self.crit_damage + other.crit_damage,
            bonus_damage:           self.bonus_damage + other.bonus_damage,
            health_regen:           self.health_regen + other.health_regen,
            healing:                self.healing + other.healing,
            thorns:                 self.thorns + other.thorns,
            dodge:                  self.dodge + other.dodge,
            speed:                  self.speed + other.speed,
            lifesteal:              self.lifesteal + other.lifesteal,
            defence:                self.defence + other.defence,
            xp_rate:                self.xp_rate + other.xp_rate,
            loot_rate:              self.loot_rate + other.loot_rate,
        }
    }
}
