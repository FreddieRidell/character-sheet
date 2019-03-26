use std::fmt;

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
    pub fn with_proficiency<'a, 'b: 'a>(&'b self, proficency_bonus: u8) -> StatCalculated<'a> {
        StatCalculated::<'a> {
            stat_instance: self,
            proficency_bonus,
        }
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn renders_constructed_skill() {
        let instance = StatInstance::new(Stat::Str, true, 11);

        assert_eq!(format!("{}", instance.with_proficiency(2)), "Str(11) [+2]");
    }
}
