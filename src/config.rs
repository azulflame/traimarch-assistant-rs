use std::collections::BTreeMap;
use std::sync::Once;
use shuttle_runtime::SecretStore;

pub enum SecretType {
    Token,
    GuildId,
    StaffBots,
}
static mut CONFIG: Option<SecretStore> = None;
static INIT_CONFIG: Once = Once::new();
impl SecretType {
    fn to_string(&self) -> String {
        match self {
            SecretType::Token => "TOKEN",
            SecretType::GuildId => "GUILD_ID",
            SecretType::StaffBots => "STAFF_BOTS",
        }
            .to_string()
    }
}

pub fn load_config(ss: SecretStore) {
    unsafe {
        INIT_CONFIG.call_once(|| {
            CONFIG = Some(ss.clone());
        })
    }
}

pub fn get_config_val(secret_type: SecretType) -> String {
    unsafe {
        INIT_CONFIG.call_once(|| {
            CONFIG = Some(SecretStore::new(BTreeMap::new()));
        });
        CONFIG
            .as_ref()
            .unwrap()
            .get(secret_type.to_string().as_str())
            .unwrap()
    }
}