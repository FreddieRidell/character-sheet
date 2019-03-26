use super::dice::Dice;
use super::stats::Stat;

pub enum PointComponent {
    Dice(Dice),
    Proficency,
    Stat(Stat),
}
