pub trait WeaponBehavior: Sized {
    fn fight(&self) -> ()
    where
        Self: Sized;
}

pub struct Weapon {
    pub name: &'static str,
}

impl WeaponBehavior for Weapon {
    fn fight(&self) {
        println!("{}!", self.name);
    }
}
