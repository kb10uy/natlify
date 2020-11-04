mod cli;
mod extensions;
mod commands {
    pub mod get;
    pub mod list;
}

use cli::{Arguments, SubCommand};

use anyhow::Result;
use clap::Clap;

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let arguments = Arguments::parse();

    match &arguments.sub_command {
        SubCommand::Get(_) => commands::get::get_sounds(&arguments).await,
        SubCommand::List(_) => commands::list::list_buttons(&arguments).await,
    }
}
