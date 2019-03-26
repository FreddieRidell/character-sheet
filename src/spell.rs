enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100
}

enum DamageType { }

struct Damage {
    my_type: DamageType
}

enum SpellType { }

struct Spell {
    my_type: SpellType,
    name: String,
}
