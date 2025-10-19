use crate::log::{Severity, Level};
use crate::ignore;

#[derive(Copy, Clone)]
pub struct ModuleMeta {
    pub severity: Severity,
    pub level: Level
}

pub struct ModuleMetaManager {
    meta: super::types::MetaStorage<&'static str, ModuleMeta>,
    allow_unregistered: bool
}

impl ModuleMetaManager {
    pub const fn new() -> Self {
        Self {
            // meta: BTreeMap::new(),
            meta: super::types::MetaStorage::new(),
            allow_unregistered: false
        }
    }

    pub fn register(&mut self, name: &'static str, severity: Severity, level: u8) {
        ignore!(self.meta.insert(name, ModuleMeta { severity, level }));
    }

    pub fn get(&self, name: &str) -> Option<&ModuleMeta> {
        self.meta.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut ModuleMeta> {
        self.meta.get_mut(name)
    }

    pub fn set_severity(&mut self, name: &str, severity: Severity) {
        if let Some(meta) = self.get_mut(name) {
            meta.severity = severity;
        }
    }

    pub fn set_level(&mut self, name: &str, level: u8) {
        if let Some(meta) = self.get_mut(name) {
            meta.level = level;
        }
    }

    // pub fn for_each(&self, f: &dyn Fn(&str, &ModuleMeta)) {
    //     self.meta.iter().for_each(|(name, meta)| f(name, meta));
    // }
    
    pub fn iter(&self) -> impl Iterator<Item=(&&'static str, &ModuleMeta)> {
        self.meta.iter()
    }
    
    pub fn allow_unregistered(&self) -> bool {
        self.allow_unregistered
    }

    pub fn set_allow_unregistered(&mut self, value: bool) {
        self.allow_unregistered = value;
    }
}


impl crate::object::Object for ModuleMetaManager {}
