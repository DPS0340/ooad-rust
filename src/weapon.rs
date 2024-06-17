pub trait WeaponBehavior {
    fn fight(&self) -> ();
}

struct SwordBehavior {}
struct KnifeBehavior {}
struct AxeBehavior {}
struct BowAndArrowBehavior {}

impl WeaponBehavior for SwordBehavior {
    fn fight(&self) {
        println!("Sword!");
    }
}
impl WeaponBehavior for KnifeBehavior {
    fn fight(&self) {
        println!("KnifeBehavior");
    }
}
impl WeaponBehavior for AxeBehavior {
    fn fight(&self) {
        println!("AxeBehavior");
    }
}
impl WeaponBehavior for BowAndArrowBehavior {
    fn fight(&self) {
        println!("AxeBehavior");
    }
}
