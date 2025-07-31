use std::collections::BTreeMap;
use crate::output::formatting::LevelOfDetail;

pub struct Description {
    content_mapping: BTreeMap<LevelOfDetail, String>
}

impl Description {
    pub fn describe(self, level_of_detail: &LevelOfDetail) -> Option<String> {
        self.content_mapping.get(level_of_detail).cloned()
    }
}
