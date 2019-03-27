use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::clone::Clone;

/// The different stats
#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Stat {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stat::Str => write!(f, "Str"),
            Stat::Dex => write!(f, "Dex"),
            Stat::Con => write!(f, "Con"),
            Stat::Int => write!(f, "Int"),
            Stat::Wis => write!(f, "Wis"),
            Stat::Cha => write!(f, "Cha"),
        }
    }
}

/// An instance of a stat, owned by a specific character
#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct StatInstance {
    stat: Stat,
    proficent: bool,
    points: u8,
}

impl StatInstance {
    pub fn new(stat: Stat, proficent: bool, points: u8) -> Self {
        Self {
            stat,
            proficent,
            points,
        }
    }
}

impl StatInstance {
    /// by default stats don't know a character's proficiency, this reifies a stat with a
    /// proficiency
    pub fn with_proficiency<'a, 'b: 'a>(&'b self, proficency_bonus: u8) -> StatCalculated<'a> {
        StatCalculated::<'a> {
            stat_instance: self,
            proficency_bonus,
        }
    }
}

/// Used to render a specific stat, once proficiency has been taken into account
pub struct StatCalculated<'a> {
    stat_instance: &'a StatInstance,
    proficency_bonus: u8,
}

impl<'a> fmt::Display for StatCalculated<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}({}) [{:+}]",
            self.stat_instance.stat,
            self.stat_instance.points,
            ((self.stat_instance.points - 10) / 2)
                + if self.stat_instance.proficent {
                    self.proficency_bonus
                } else {
                    0
                }
        )
    }
}

/// The collection of all stats that a character has
#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    stat_container: HashMap<Stat, StatInstance>,
}

impl Stats {
    pub fn new() -> Self {
        let mut stat_container = HashMap::new();

        for stat in vec![
            Stat::Str,
            Stat::Dex,
            Stat::Con,
            Stat::Int,
            Stat::Wis,
            Stat::Cha,
        ] {
            stat_container.insert(
                stat,
                StatInstance {
                    stat: stat,
                    proficent: false,
                    points: 10,
                },
            );
        }

        Self { stat_container }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn renders_constructed_skill() {
        let instance = StatInstance::new(Stat::Str, true, 11);

        assert_eq!(format!("{}", instance.with_proficiency(2)), "Str(11) [+2]");
    }

    #[test]
    fn serializes_and_deserializes_correctly() {
        let mut stats = Stats::new();

        let serialized = serde_json::to_string_pretty(&stats).unwrap();

        assert_eq!(serialized, "", "{} != {}", serialized, "");

        let deserialized: Stats = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, stats);

    }
}
