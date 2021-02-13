use crate::entity;

pub trait LawStorageTrait {
    fn add(&mut self, law: entity::Law);
    fn get_by_number(&self, law_number: entity::LawNumber) -> Option<&entity::Law>;
    fn len(&self) -> usize;
}

pub type LawStorageInternalType = std::collections::HashMap<entity::LawNumber, entity::Law>;

pub struct LawStorage {
    database: LawStorageInternalType,
}

impl LawStorage {
    pub fn new_empty() -> Self {
        LawStorage {
            database: LawStorageInternalType::new(),
        }
    }
}

impl LawStorageTrait for LawStorage {
    fn add(&mut self, law: entity::Law) {
        self.database.insert(law.id, law);
    }

    fn get_by_number(&self, law_number: entity::LawNumber) -> Option<&entity::Law> {
        self.database.get(&law_number)
    }

    fn len(&self) -> usize {
        return self.database.len();
    }
}
