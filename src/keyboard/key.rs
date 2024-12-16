#[repr(usize)]
#[derive(Hash)]
pub enum KeyboardKey {
    A,
}

impl KeyboardKey {
    pub fn all_keys() -> Vec<KeyboardKey> {
        vec![KeyboardKey::A]
    }
}
