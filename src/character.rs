use super::weapon::WeaponBehavior;

pub trait HasWeapon {
    fn useWeapon(&self) -> ();
    fn setWeapon(&mut self, w: dyn WeaponBehavior) -> ();
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

impl HasWeapon for Character {
    fn useWeapon(&self) {
        self.weapon.fight();
    }
    fn setWeapon(&mut self, w: dyn WeaponBehavior) {
        self.weapon = w;
    }
}
impl HasWeapon for Queen {
    fn useWeapon(&self) {
        self.weapon.fight();
    }
    fn setWeapon(&mut self, w: dyn WeaponBehavior) {
        self.weapon = w;
    }
}
impl HasWeapon for King {
    fn useWeapon(&self) {
        self.weapon.fight();
    }
    fn setWeapon(&mut self, w: dyn WeaponBehavior) {
        self.weapon = w;
    }
}
impl HasWeapon for Troll {
    fn useWeapon(&self) {
        self.weapon.fight();
    }
    fn setWeapon(&mut self, w: dyn WeaponBehavior) {
        self.weapon = w;
    }
}
impl HasWeapon for Knight {
    fn useWeapon(&self) {
        self.weapon.fight();
    }
    fn setWeapon(&mut self, w: dyn WeaponBehavior) {
        self.weapon = w;
    }
}
