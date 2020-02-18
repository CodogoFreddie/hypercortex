use crate::render::RenderColumns;
use hypertask_config_file_opener::ShellExpand;
use hypertask_engine::prelude::*;
use hypertask_task_io_operations::ProvidesDataDir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HooksConfig {
    pub after: Option<String>,
    pub on_edit: Option<String>,
    pub before: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderConfig {
    pub score_precision: u32,
    pub columns: Vec<RenderColumns>,
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
    pub task_state_dir: PathBuf,
    pub hooks: Option<HooksConfig>,
    pub render: RenderConfig,
    pub filter_calculator: ScoreCalculatorConfig,
    pub score_calculator: ScoreCalculatorConfig,
}

impl ProvidesDataDir for CliConfig {
    fn get_task_state_dir(&self) -> &PathBuf {
        &self.task_state_dir
    }
}

impl ShellExpand for CliConfig {
    fn shell_expand(&mut self) {
        let task_state_dir_str: &str = self
            .task_state_dir
            .to_str()
            .expect("could not string from task_state_dir");

        let expanded_task_state_dir = shellexpand::tilde(task_state_dir_str);

        self.task_state_dir = PathBuf::from(expanded_task_state_dir.into_owned());
    }
}
