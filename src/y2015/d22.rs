use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Default)]
struct Entity {
    hp: i32,
    armor: i32,
    dmg: i32,
    mana: i32,

    effects: [u32; EffectKind::__NONE__ as usize],
}

impl Entity {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        fn latter_i32(input: &str) -> i32 {
            i32::from_str_radix(input.split_once(": ").unwrap().1, 10).unwrap()
        }

        Self {
            hp: latter_i32(lines.next().unwrap()),
            dmg: latter_i32(lines.next().unwrap()),
            ..Default::default()
        }
    }
    fn update_effects(&mut self) {
        for i in 0..self.effects.len() {
            if self.effects[i] > 0 {
                let kind = EffectKind::from_usize(i).unwrap();
                kind.on_round(self);
                self.effects[i] -= 1;
                if self.effects[i] == 0 {
                    kind.on_end(self);
                }
            }
        }
    }
    fn add_effect(&mut self, effect: Effect) {
        if self.effects[effect.kind as usize] == 0 {
            effect.kind.on_start(self);
            self.effects[effect.kind as usize] = effect.timer;
        }
    }
    fn deal_damage(&mut self, dmg: Damage) {
        if dmg.physical > 0 {
            self.hp -= (dmg.physical - self.armor).max(1);
        } else {
            self.hp -= dmg.physical;
        }
        self.hp -= dmg.magical;
        self.mana -= dmg.mana;
    }
    fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}

#[derive(Clone, Copy, Default)]
struct Damage {
    physical: i32,
    magical: i32,
    mana: i32,
}

impl Damage {
    const fn physical(physical: i32) -> Self {
        Self {
            physical,
            magical: 0,
            mana: 0,
        }
    }
    const fn mana(mana: i32) -> Self {
        Self {
            physical: 0,
            magical: 0,
            mana,
        }
    }
    const fn magical(magical: i32) -> Self {
        Self {
            physical: 0,
            magical,
            mana: 0,
        }
    }
    const fn none() -> Self {
        Self {
            physical: 0,
            magical: 0,
            mana: 0,
        }
    }
}

#[derive(Clone, Copy, FromPrimitive)]
enum EffectKind {
    Poision,
    Recharge,
    Shield,
    __NONE__,
}

impl EffectKind {
    fn on_start(&self, e: &mut Entity) {
        match self {
            EffectKind::Shield => e.armor += 7,
            _ => {}
        }
    }
    fn on_end(&self, e: &mut Entity) {
        match self {
            EffectKind::Shield => e.armor -= 7,
            _ => {}
        }
    }
    fn on_round(&self, e: &mut Entity) {
        match self {
            EffectKind::Poision => e.deal_damage(Damage::magical(3)),
            EffectKind::Recharge => e.deal_damage(Damage::mana(-101)),
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
struct Effect {
    kind: EffectKind,
    timer: u32,
}

impl Effect {
    const fn new(timer: u32, kind: EffectKind) -> Self {
        Self { kind, timer }
    }
}

enum SpellKind {
    Instant(Damage, Damage),
    SelfEffect(Effect),
    EnemyEffect(Effect),
}

struct Spell {
    #[allow(dead_code)]
    name: &'static str,
    mana: i32,
    kind: SpellKind,
}

impl Spell {
    const fn new(name: &'static str, mana: i32, kind: SpellKind) -> Self {
        Self { name, mana, kind }
    }

    fn cast(&self, caster: &mut Entity, enemy: &mut Entity) {
        caster.deal_damage(Damage::mana(self.mana));
        match self.kind {
            SpellKind::Instant(caster_dmg, enemy_dmg) => {
                caster.deal_damage(caster_dmg);
                enemy.deal_damage(enemy_dmg);
            }
            SpellKind::SelfEffect(effect) => caster.add_effect(effect),
            SpellKind::EnemyEffect(effect) => enemy.add_effect(effect),
        }
    }
}

const SPELLS: [Spell; 5] = [
    Spell::new(
        "Magic Missile",
        53,
        SpellKind::Instant(Damage::none(), Damage::magical(4)),
    ),
    Spell::new(
        "Drain",
        73,
        SpellKind::Instant(Damage::magical(-2), Damage::magical(2)),
    ),
    Spell::new(
        "Shield",
        113,
        SpellKind::SelfEffect(Effect::new(6, EffectKind::Shield)),
    ),
    Spell::new(
        "Poison",
        173,
        SpellKind::EnemyEffect(Effect::new(6, EffectKind::Poision)),
    ),
    Spell::new(
        "Recharge",
        229,
        SpellKind::SelfEffect(Effect::new(5, EffectKind::Recharge)),
    ),
];

pub fn solution_1(input: &str) -> String {
    let boss = Entity::from_str(input);

    fn simulate_all(caster: &Entity, boss: &Entity, turn: u32) -> Option<i32> {
        if turn > 10 {
            return None;
        }
        SPELLS
            .iter()
            .filter_map(|spell| {
                let mut caster = caster.clone();
                let mut boss = boss.clone();
                caster.update_effects();
                boss.update_effects();
                if boss.is_dead() {
                    return Some(spell.mana);
                }
                if spell.mana > caster.mana {
                    return None;
                }
                spell.cast(&mut caster, &mut boss);
                if boss.is_dead() {
                    return Some(spell.mana);
                }
                caster.update_effects();
                boss.update_effects();
                if boss.is_dead() {
                    return Some(spell.mana);
                }
                caster.deal_damage(Damage::physical(boss.dmg));
                if caster.is_dead() {
                    return None;
                }

                simulate_all(&caster, &boss, turn + 1).map(|spent| spent + spell.mana)
            })
            .min()
    }
    simulate_all(
        &Entity {
            hp: 50,
            mana: 500,
            ..Default::default()
        },
        &boss,
        0,
    )
    .unwrap()
    .to_string()
}

pub fn solution_2(input: &str) -> String {
    let boss = Entity::from_str(input);

    fn simulate_all(caster: &Entity, boss: &Entity, turn: u32) -> Option<i32> {
        if turn > 10 {
            return None;
        }
        SPELLS
            .iter()
            .filter_map(|spell| {
                let mut caster = caster.clone();
                let mut boss = boss.clone();
                caster.hp -= 1;
                if caster.is_dead() {
                    return None;
                }
                caster.update_effects();
                boss.update_effects();
                if boss.is_dead() {
                    return Some(spell.mana);
                }
                if spell.mana > caster.mana {
                    return None;
                }
                spell.cast(&mut caster, &mut boss);
                if boss.is_dead() {
                    return Some(spell.mana);
                }
                caster.update_effects();
                boss.update_effects();
                if boss.is_dead() {
                    return Some(spell.mana);
                }
                caster.deal_damage(Damage::physical(boss.dmg));
                if caster.is_dead() {
                    return None;
                }

                simulate_all(&caster, &boss, turn + 1).map(|spent| spent + spell.mana)
            })
            .min()
    }
    simulate_all(
        &Entity {
            hp: 50,
            mana: 500,
            ..Default::default()
        },
        &boss,
        0,
    )
    .unwrap()
    .to_string()
}
