mod character;
mod weapon;

fn main() {}

#[macro_export]
macro_rules! assert_stdout_eq {
    ($test:expr, $expected:literal) => {{
        use gag::BufferRedirect;
        use std::io::Read;

        let mut buf = BufferRedirect::stdout().unwrap();

        $test;

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        drop(buf);

        assert_eq!(&output[..], $expected);
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use character::HasWeapon;
    use weapon::Weapon;

    #[test]
    fn check_character_weapons() {
        let mut king = character::Character {
            weapon: Weapon { name: "Sword" },
        };
        assert_stdout_eq!(king.use_weapon(), "Sword!\n");
        king.weapon = Weapon { name: "Axe" };
        assert_stdout_eq!(king.use_weapon(), "Axe!\n");
    }
}
