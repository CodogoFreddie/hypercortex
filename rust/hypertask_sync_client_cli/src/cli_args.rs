use clap::Clap;
use std::path::PathBuf;

/// Syncing server to replicate hypertask tasks with clients over HTTP
#[derive(Clap, Clone)]
pub struct CliArgs {
    /// Directory containing tasks
    #[clap(long, env = "HYPERTASK_DATA_DIR")]
    pub data_dir: PathBuf,

    /// Should the server daemonise
    #[clap(short, long)]
    pub daemonize: bool,

    /// The hostname that the server will listen under
    #[clap(short, long, env = "HYPERTASK_SERVER_URL")]
    pub server_url: Option<String>,

    /// The authorisation secret that must be passed by the client.
    /// The server will generate one if you do not specify
    #[clap(short, long, env = "HYPERTASK_SYNC_SECRET", hide_env_values = true)]
    pub sync_secret: Option<String>,

    /// File to divert stdout to
    #[clap(long)]
    pub std_out_file: Option<PathBuf>,

    /// File to divert stderr to
    #[clap(long)]
    pub std_err_file: Option<PathBuf>,

    /// File to store PID in
    #[clap(long)]
    pub pid_file: Option<PathBuf>,

    /// Rate at which to resync with the server
    #[clap(long)]
    pub rescan_refresh_rate: Option<u64>,

    /// Rate at which to resync with the server
    #[clap(long)]
    pub recan_time_file: Option<PathBuf>,

    /// Should we watch the data-dir for changes and resync when we detect them
    #[clap(long)]
    pub watch_for_changes: bool,
}

impl hypertask_task_io_operations::ProvidesDataDir for CliArgs {
    fn get_task_state_dir(&self) -> &std::path::PathBuf {
        &self.data_dir
    }
}
