trait WeaponBehavior {
    fn fight(&self) -> ();
}

struct Weapon<T: WeaponBehavior> {
    weapon: T
}

impl<T: WeaponBehavior> Weapon<T> {
    fn fight(&self) {
        self.weapon.fight();
    }
}

let SwordBehavior: 'static Weapon = Weapon {
    weapon: WeaponBehavior {
        fn fight(&self) {

        }
    }
};

struct SwordBehavior {}
struct KnifeBehavior {}
struct AxeBehavior {}
struct BowAndArrowBehavior {}

impl SwordBehavior {
    fn fight(&self) {
        println!("Sword!");
    }
}
impl KnifeBehavior {
    fn fight(&self) {
        println!("KnifeBehavior");
    }
}
impl AxeBehavior {
    fn fight(&self) {
        println!("AxeBehavior");
    }
}
impl BowAndArrowBehavior {
    fn fight(&self) {
        println!("BowAndArrowBehavior");
    }
}

pub trait HasWeapon {
    weapon: 
    fn useWeapon(&self) -> {
        self.we
    };
}

pub struct Character {
    weapon: dyn WeaponBehavior,
}
pub struct Queen {
    weapon: dyn WeaponBehavior,
}
pub struct King {
    weapon: dyn WeaponBehavior,
}
pub struct Troll {
    weapon: dyn WeaponBehavior,
}
pub struct Knight {
    weapon: dyn WeaponBehavior,
}

fn main() {
    println!("Hello, world!");
}
