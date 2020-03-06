use crate::render::RenderColumns;
use hypertask_engine::prelude::*;
use serde::{Deserialize, Serialize};
use simple_persist_data::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderConfig {
    pub columns: Vec<RenderColumns>,
    pub score_precision: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            score_precision: 3,
            columns: vec![
                RenderColumns::Id,
                RenderColumns::Score,
                RenderColumns::Description,
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ScoreCalculatorConfig {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for ScoreCalculatorConfig {
    fn default() -> Self {
        ScoreCalculatorConfig::Single("now @ due : -".to_string())
    }
}

impl ScoreCalculatorConfig {
    pub fn to_program(&self) -> Vec<RPNSymbol> {
        match self {
            ScoreCalculatorConfig::Single(s) => RPNSymbol::parse_program(s),
            ScoreCalculatorConfig::Multiple(ss) => RPNSymbol::parse_programs(ss),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CliConfig {
    pub filter_calculator: ScoreCalculatorConfig,
    pub score_calculator: ScoreCalculatorConfig,
    pub render: RenderConfig,
}

impl PersistableSingle for CliConfig {
    const APP_INFO: AppInfo = crate::app_info::APP_INFO;
    const APP_DATA_TYPE: AppDataType = AppDataType::UserConfig;
    const FORMAT: Format = Format::Toml;
    const NAME: &'static str = "config";

    const COMMENT: Option<&'static str> = Some("The config for the hypertask cli client, more documentation can be found by running `$ man task`");

    fn after_save(&self) -> () {
        let file_path_buf =
            Self::get_file_path().expect("could not derive file path for config.toml");
        let file_path = file_path_buf
            .to_str()
            .expect("could not derive file path for config.toml");

        println!(
            "a config file has been saved for you, you can find it at '{}'",
            file_path
        );
    }

    fn before_load() -> bool {
        let file_path_buf =
            Self::get_file_path().expect("could not derive file path for config.toml");
        let file_path = file_path_buf
            .to_str()
            .expect("could not derive file path for config.toml");

        info!("loading config from '{}'", file_path);

        true
    }

    fn after_load(&self) -> () {
        trace!("loaded config: {:?}", self);
    }
}
