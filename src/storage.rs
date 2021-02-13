#[derive(Clone, serde::Deserialize)]
pub struct Law {
    id: i32,
    name: String,
    text: String,
    legitimacy: bool,
}

pub trait LawStorageTrait {
    fn get_by_number(&self, law_number: i32) -> Option<&Law>;
    fn len(&self) -> usize;
}

pub type LawStorageInternalType = std::collections::HashMap<i32, Law>;

pub struct LawStorage {
    database: LawStorageInternalType,
}

impl LawStorage {
    pub fn new_empty() -> Self {
        LawStorage {
            database: LawStorageInternalType::new(),
        }
    }

    pub fn new(mut initial_values: LawStorageInternalType) -> Self {
        for (key, value) in initial_values.iter_mut() {
            value.id = key.clone();
        }

        LawStorage {
            database: initial_values,
        }
    }

    pub fn len(&self) -> usize {
        return self.database.len();
    }
}

impl LawStorageTrait for LawStorage {
    fn get_by_number(&self, law_number: i32) -> Option<&Law> {
        self.database.get(&law_number)
    }

    fn len(&self) -> usize {
        return self.database.len();
    }
}
