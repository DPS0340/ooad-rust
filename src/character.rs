use super::weapon::WeaponBehavior;
use crate::weapon::Weapon;

pub trait HasWeapon: Sized {
    fn use_weapon(&self) -> ()
    where
        Self: Sized;
}

pub struct Character {
    pub weapon: Weapon,
}

impl HasWeapon for Character {
    fn use_weapon(&self) {
        self.weapon.fight();
    }
}
