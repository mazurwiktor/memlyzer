use config::Config;
use config::File;

pub struct Configuration {
    pub loot_list: Vec<String>,
}

impl Configuration {
    pub fn new() -> Self {
        let mut settings = Config::default();
        settings.merge(File::with_name("Config")).unwrap();
        let loot_list = settings.get::<Vec<String>>("loot_list").unwrap();
        debug!("loot_list loaded...");

        Configuration { loot_list }
    }
}
