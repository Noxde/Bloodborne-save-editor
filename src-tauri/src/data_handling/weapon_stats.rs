//! Weapon stats as shown in the in-game equipment menu, blood gems included.
//! The data comes from the game params, see scripts/extract_weapon_params.py

use super::{enums::Error, file::FileData, stats};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::OnceLock};

#[derive(Deserialize)]
struct Params {
    weapons: HashMap<u32, WeaponRow>,
    graphs: HashMap<u32, Graph>,
    gems: HashMap<u32, GemMods>,
}

// One EquipParamWeapon row: a weapon at a given imprint and upgrade level
#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct WeaponRow {
    physical: f64,
    arcane: f64,
    fire: f64,
    bolt: f64,
    // Scaling, 100 = 1.0
    str: f64,
    skl: f64,
    bld: f64,
    arc: f64,
    req_str: u32,
    req_skl: u32,
    req_bld: u32,
    req_arc: u32,
    durability: i32,
    bullets: u32,
    graph: u32,
    blunt: bool,
    thrust: bool,
    beasts: f64,
    kin: f64,
}

impl Default for WeaponRow {
    fn default() -> Self {
        WeaponRow {
            physical: 0.0,
            arcane: 0.0,
            fire: 0.0,
            bolt: 0.0,
            str: 0.0,
            skl: 0.0,
            bld: 0.0,
            arc: 0.0,
            req_str: 0,
            req_skl: 0,
            req_bld: 0,
            req_arc: 0,
            durability: 0,
            bullets: 0,
            graph: 0,
            blunt: false,
            thrust: false,
            beasts: 1.0,
            kin: 1.0,
        }
    }
}

// CalcCorrectGraph: how much of a weapon's scaling an attribute level gives
#[derive(Deserialize)]
struct Graph {
    stat: [f64; 5],
    grow: [f64; 5],
    exp: [f64; 5],
}

impl Graph {
    fn eval(&self, stat: u32) -> f64 {
        let stat = stat as f64;
        if stat <= self.stat[0] {
            return self.grow[0] / 100.0;
        }
        for i in 0..4 {
            if stat <= self.stat[i + 1] {
                let ratio = (stat - self.stat[i]) / (self.stat[i + 1] - self.stat[i]);
                let growth = if self.exp[i] >= 0.0 {
                    ratio.powf(self.exp[i])
                } else {
                    1.0 - (1.0 - ratio).powf(-self.exp[i])
                };
                return (self.grow[i] + (self.grow[i + 1] - self.grow[i]) * growth) / 100.0;
            }
        }
        self.grow[4] / 100.0
    }
}

// The modifiers of a gem effect, also used to add up all the equipped ones
#[derive(Deserialize, Clone)]
#[serde(default, rename_all = "camelCase")]
struct GemMods {
    phys_rate: f64,
    arcane_rate: f64,
    fire_rate: f64,
    bolt_rate: f64,
    blood_rate: f64,
    blunt_rate: f64,
    thrust_rate: f64,
    phys_add: f64,
    arcane_add: f64,
    fire_add: f64,
    bolt_add: f64,
    blood_add: f64,
    str: f64,
    skl: f64,
    bld: f64,
    arc: f64,
    beasts: f64,
    kin: f64,
    durability: i32,
    stamina: f64,
    rally: f64,
    hp_regen: i32,
    convert: u8,
    slow_poison: f64,
    rapid_poison: f64,
    condition: Option<String>,
    condition_rate: f64,
    condition_all: bool,
}

impl Default for GemMods {
    fn default() -> Self {
        GemMods {
            phys_rate: 1.0,
            arcane_rate: 1.0,
            fire_rate: 1.0,
            bolt_rate: 1.0,
            blood_rate: 1.0,
            blunt_rate: 1.0,
            thrust_rate: 1.0,
            phys_add: 0.0,
            arcane_add: 0.0,
            fire_add: 0.0,
            bolt_add: 0.0,
            blood_add: 0.0,
            str: 0.0,
            skl: 0.0,
            bld: 0.0,
            arc: 0.0,
            beasts: 1.0,
            kin: 1.0,
            durability: 0,
            stamina: 1.0,
            rally: 1.0,
            hp_regen: 0,
            convert: 0,
            slow_poison: 0.0,
            rapid_poison: 0.0,
            condition: None,
            condition_rate: 1.0,
            condition_all: false,
        }
    }
}

impl GemMods {
    fn add(&mut self, gem: &GemMods, conditional: &mut Vec<Conditional>) {
        if let Some(condition) = &gem.condition {
            match conditional
                .iter_mut()
                .find(|c| &c.condition == condition && c.all == gem.condition_all)
            {
                Some(c) => c.percent = rate_percent(percent_rate(c.percent) * gem.condition_rate),
                None => conditional.push(Conditional {
                    condition: condition.clone(),
                    all: gem.condition_all,
                    percent: rate_percent(gem.condition_rate),
                }),
            }
            return;
        }
        self.phys_rate *= gem.phys_rate;
        self.arcane_rate *= gem.arcane_rate;
        self.fire_rate *= gem.fire_rate;
        self.bolt_rate *= gem.bolt_rate;
        self.blood_rate *= gem.blood_rate;
        self.blunt_rate *= gem.blunt_rate;
        self.thrust_rate *= gem.thrust_rate;
        self.phys_add += gem.phys_add;
        self.arcane_add += gem.arcane_add;
        self.fire_add += gem.fire_add;
        self.bolt_add += gem.bolt_add;
        self.blood_add += gem.blood_add;
        self.str += gem.str;
        self.skl += gem.skl;
        self.bld += gem.bld;
        self.arc += gem.arc;
        self.beasts *= gem.beasts;
        self.kin *= gem.kin;
        self.durability += gem.durability;
        self.stamina *= gem.stamina;
        self.rally *= gem.rally;
        self.hp_regen += gem.hp_regen;
        self.slow_poison += gem.slow_poison;
        self.rapid_poison += gem.rapid_poison;
        if self.convert == 0 {
            self.convert = gem.convert;
        }
    }
}

fn params() -> &'static Params {
    static PARAMS: OnceLock<Params> = OnceLock::new();
    PARAMS.get_or_init(|| {
        serde_json::from_str(include_str!("../../resources/weapon_params.json")).unwrap()
    })
}

fn rate_percent(rate: f64) -> f64 {
    ((rate - 1.0) * 1000.0).round() / 10.0
}

fn percent_rate(percent: f64) -> f64 {
    1.0 + percent / 100.0
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attributes {
    pub str: u32,
    pub skl: u32,
    pub bld: u32,
    pub arc: u32,
}

impl Attributes {
    pub fn from_file(file: &FileData) -> Result<Attributes, Error> {
        let stats = stats::new(file).map_err(Error::IoError)?;
        let get = |name: &str| stats.iter().find(|s| s.name == name).map_or(0, |s| s.value);
        Ok(Attributes {
            str: get("Strength"),
            skl: get("Skill"),
            bld: get("Bloodtinge"),
            arc: get("Arcane"),
        })
    }
}

/// A row of the attack section, the menu shows it as "base +bonus" (base `None` is "-")
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Attack {
    pub base: Option<u32>,
    pub bonus: i64,
}

impl Attack {
    fn new(base: f64, total: f64, add: f64) -> Attack {
        Attack {
            base: Some(base as u32),
            bonus: (total.floor() + add - base) as i64,
        }
    }

    fn flat(add: f64) -> Attack {
        Attack {
            base: None,
            bonus: add as i64,
        }
    }
}

/// Effects that only apply in some situations (charge attacks, full HP...)
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Conditional {
    pub condition: String,
    pub all: bool, // All damage types or only physical
    pub percent: f64,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct WeaponStats {
    pub physical: Attack,
    pub blunt: Option<i64>,
    pub thrust: Option<i64>,
    pub blood: Attack,
    pub arcane: Attack,
    pub fire: Attack,
    pub bolt: Attack,
    pub bullets: Option<u32>,
    pub durability: i32,
    pub slow_poison: u32,
    pub rapid_poison: u32,
    pub beasts: u32,
    pub kin: u32,
    pub bonus: [&'static str; 4], // Str, Skl, Bld, Arc
    pub requirements: [u32; 4],
    pub stamina_cost: f64, // Percent change
    pub rally: f64,        // Percent change
    pub hp_regen: i32,
    pub conditional: Vec<Conditional>,
    pub converted_to: Option<&'static str>,
    pub attributes: [u32; 4], // The Str, Skl, Bld, Arc used for the calculation
}

fn grade(scaling: f64) -> &'static str {
    // MenuValueTableParam rows 100-106
    match scaling {
        s if s >= 110.0 => "S",
        s if s >= 85.0 => "A",
        s if s >= 65.0 => "B",
        s if s >= 55.0 => "C",
        s if s >= 30.0 => "D",
        s if s >= 1.0 => "E",
        _ => "-",
    }
}

/// Calculates the stats of a weapon (`weapon_id` includes imprint and upgrade level)
/// with the given gem effects equipped, for a character with `attributes`.
pub fn compute(weapon_id: u32, gem_effects: &[u32], attributes: Attributes) -> Result<WeaponStats, Error> {
    let params = params();
    let w = params
        .weapons
        .get(&weapon_id)
        .ok_or(Error::CustomError("ERROR: No stats found for this weapon."))?;
    let graph = params
        .graphs
        .get(&w.graph)
        .ok_or(Error::CustomError("ERROR: No scaling graph found for this weapon."))?;

    let mut gems = GemMods::default();
    let mut conditional = Vec::new();
    for id in gem_effects {
        if let Some(gem) = params.gems.get(id) {
            gems.add(gem, &mut conditional);
        }
    }

    let scaling = [w.str + gems.str, w.skl + gems.skl, w.bld + gems.bld, w.arc + gems.arc];
    let bonus = |index: usize, stat: u32| scaling[index] / 100.0 * graph.eval(stat);

    // Blood damage is a kind of physical damage that scales only with Bloodtinge
    let has_blood = w.bld > 0.0;
    // Fire, bolt and arcane gems convert weapons that only deal physical damage
    let convertible = w.arcane == 0.0 && w.fire == 0.0 && w.bolt == 0.0 && !has_blood;
    let convert = if convertible { gems.convert } else { 0 };
    // Firearms only show their blood damage
    let has_physical = w.physical > 0.0 && convert == 0 && !(has_blood && w.str == 0.0 && w.skl == 0.0);

    let physical_total = w.physical
        * (1.0 + bonus(0, attributes.str) + bonus(1, attributes.skl))
        * gems.phys_rate;
    let (physical, blunt, thrust) = if has_physical {
        let sub_type = |enabled: bool, rate: f64| {
            enabled.then(|| Attack::new(w.physical, physical_total * rate, gems.phys_add).bonus)
        };
        (
            Attack::new(w.physical, physical_total, gems.phys_add),
            sub_type(w.blunt, gems.blunt_rate),
            sub_type(w.thrust, gems.thrust_rate),
        )
    } else {
        (Attack::flat(gems.phys_add), None, None)
    };

    let blood = if has_blood {
        let total = w.physical * (1.0 + bonus(2, attributes.bld)) * gems.phys_rate * gems.blood_rate;
        Attack::new(w.physical, total, gems.blood_add)
    } else {
        Attack::flat(gems.blood_add)
    };

    // Arcane, fire and bolt scale with Arcane
    let elemental = |innate: f64, convert_id: u8, rate: f64, add: f64| {
        let base = if convert == convert_id { w.physical } else { innate };
        if base > 0.0 {
            Attack::new(base, base * (1.0 + bonus(3, attributes.arc)) * rate, add)
        } else {
            Attack::flat(add)
        }
    };

    Ok(WeaponStats {
        physical,
        blunt,
        thrust,
        blood,
        arcane: elemental(w.arcane, 1, gems.arcane_rate, gems.arcane_add),
        fire: elemental(w.fire, 2, gems.fire_rate, gems.fire_add),
        bolt: elemental(w.bolt, 3, gems.bolt_rate, gems.bolt_add),
        bullets: (w.bullets > 0).then_some(w.bullets),
        durability: w.durability + gems.durability,
        slow_poison: gems.slow_poison as u32,
        rapid_poison: gems.rapid_poison as u32,
        beasts: (w.beasts * gems.beasts * 100.0).round() as u32,
        kin: (w.kin * gems.kin * 100.0).round() as u32,
        bonus: scaling.map(grade),
        requirements: [w.req_str, w.req_skl, w.req_bld, w.req_arc],
        stamina_cost: rate_percent(gems.stamina),
        rally: rate_percent(gems.rally),
        hp_regen: gems.hp_regen,
        conditional,
        converted_to: match convert {
            1 => Some("Arcane"),
            2 => Some("Fire"),
            3 => Some("Bolt"),
            _ => None,
        },
        attributes: [attributes.str, attributes.skl, attributes.bld, attributes.arc],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const NO_EFFECT: u32 = 4294967295;

    fn attrs(str: u32, skl: u32, bld: u32, arc: u32) -> Attributes {
        Attributes { str, skl, bld, arc }
    }

    #[test]
    fn test_scaling_curve() {
        let graph = &params().graphs[&0];
        assert_eq!(graph.eval(1), 0.0);
        assert_eq!(graph.eval(10), 0.05);
        assert!((graph.eval(14) - 0.17).abs() < 1e-9);
        assert_eq!(graph.eval(25), 0.5);
        assert_eq!(graph.eval(50), 0.85);
        assert_eq!(graph.eval(99), 1.0);
    }

    #[test]
    fn test_chikage() {
        //Chikage +10, 10/14/50 STR/SKL/BLT (example from the community AR guide)
        let stats = compute(2001000, &[NO_EFFECT], attrs(10, 14, 50, 9)).unwrap();
        assert_eq!(stats.physical, Attack { base: Some(184), bonus: 22 });
        assert_eq!(stats.blood, Attack { base: Some(184), bonus: 172 });
        assert_eq!(stats.bonus, ["E", "B", "S", "D"]);
        assert_eq!(stats.requirements, [10, 14, 12, 0]);
        assert_eq!(stats.durability, 120);

        //3x Blood ATK UP +31.5%: 809 blood AR
        let stats = compute(2001000, &[22618, 22618, 22618], attrs(10, 14, 50, 9)).unwrap();
        assert_eq!(stats.blood.bonus + 184, 809);
        assert_eq!(stats.physical.bonus, 22);
    }

    #[test]
    fn test_beast_claw() {
        //Beast Claw +10 has C/D/-/D scaling and a thrust row, like the in-game menu
        let stats = compute(9001000, &[], attrs(50, 24, 10, 10)).unwrap();
        assert_eq!(stats.physical, Attack { base: Some(150), bonus: 101 });
        assert_eq!(stats.thrust, Some(101));
        assert_eq!(stats.blunt, None);
        assert_eq!(stats.blood, Attack { base: None, bonus: 0 });
        assert_eq!(stats.bonus, ["C", "D", "-", "D"]);
        assert_eq!(stats.requirements, [14, 12, 0, 0]);
        assert_eq!(stats.durability, 180);
        assert_eq!(stats.bullets, None);
        assert_eq!((stats.beasts, stats.kin), (100, 100));
    }

    #[test]
    fn test_gems() {
        //STR scaling +65 turns C into S, Tempering 27.2% and +15 flat physical
        let stats = compute(9001000, &[29520, 26619, 17120], attrs(50, 25, 10, 10)).unwrap();
        assert_eq!(stats.bonus[0], "S");
        let expected = (150.0 * (1.0 + 1.25 * 0.85 + 0.35 * 0.5) * 1.2719f64).floor() + 15.0;
        assert_eq!(stats.physical.bonus + 150, expected as i64);
    }

    #[test]
    fn test_conversion() {
        //A fire gem converts a physical weapon: physical becomes fire that scales with Arcane
        let stats = compute(9001000, &[24619], attrs(50, 25, 10, 50)).unwrap();
        assert_eq!(stats.converted_to, Some("Fire"));
        assert_eq!(stats.physical, Attack { base: None, bonus: 0 });
        assert_eq!(stats.thrust, None);
        let expected = (150.0 * (1.0 + 0.52 * 0.85) * 1.2719f64).floor();
        assert_eq!(stats.fire, Attack { base: Some(150), bonus: expected as i64 - 150 });

        //Blood weapons are not converted
        let stats = compute(2001000, &[24619], attrs(10, 14, 50, 50)).unwrap();
        assert_eq!(stats.converted_to, None);
        assert_eq!(stats.fire, Attack { base: None, bonus: 0 });
    }

    #[test]
    fn test_firearm() {
        //Hunter Pistol +10 only deals blood damage
        let stats = compute(14001000, &[], attrs(10, 10, 50, 10)).unwrap();
        assert_eq!(stats.physical.base, None);
        assert_eq!(stats.blood.base, Some(160));
        assert_eq!(stats.bullets, Some(1));
    }

    #[test]
    fn test_unknown_weapon() {
        assert!(compute(1, &[], attrs(10, 10, 10, 10)).is_err());
    }

    #[test]
    fn test_attributes_from_save() {
        let file = FileData::build("saves/testsave0", PathBuf::from("resources")).unwrap();
        assert_eq!(Attributes::from_file(&file).unwrap(), attrs(99, 99, 99, 99));
    }
}
