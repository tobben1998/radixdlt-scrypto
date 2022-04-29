use crate::resource::*;
use crate::rust::collections::hash_map::Iter;
use crate::rust::collections::HashMap;
use crate::rust::str;
use crate::rust::string::String;
use crate::rust::string::ToString;
use sbor::*;

/// Method authorization rules for a component
#[derive(Debug, Clone, PartialEq, Describe, TypeId, Encode, Decode)]
pub struct AccessRules {
    method_auth: HashMap<String, AccessRule>,
    default_auth: AccessRule,
}

impl AccessRules {
    pub fn new() -> Self {
        Self {
            method_auth: HashMap::new(),
            default_auth: AccessRule::DenyAll,
        }
    }

    pub fn get(&self, method_name: &str) -> &AccessRule {
        self.method_auth
            .get(method_name)
            .unwrap_or(&self.default_auth)
    }

    pub fn get_default(&self) -> &AccessRule {
        &self.default_auth
    }

    pub fn method(mut self, method_name: &str, method_auth: AccessRule) -> Self {
        self.method_auth
            .insert(method_name.to_string(), method_auth);
        self
    }

    pub fn default(mut self, method_auth: AccessRule) -> Self {
        self.default_auth = method_auth;
        self
    }

    pub fn iter(&self) -> Iter<'_, String, AccessRule> {
        let l = self.method_auth.iter();
        l
    }
}
