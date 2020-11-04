use crate::{
    cli::{Arguments, SubCommand},
    extensions::SurfErrorExt,
};

use anyhow::{format_err, Context, Result};
use async_std::{
    fs::{create_dir_all, File},
    io::{prelude::*, BufWriter},
    path::Path,
};
use url::Url;

pub async fn get_sounds(args: &Arguments) -> Result<()> {
    let subargs = if let SubCommand::Get(get) = &args.sub_command {
        get
    } else {
        unreachable!("Invalid subcommand state")
    };

    let base_url = Url::parse(&format!("https://{}", args.domain))
        .and_then(|s| s.join(&subargs.sounds_base_path))
        .context("Failed to construct URL")?;

    for target in &subargs.targets {
        let filename = format!("{}.{}", target, subargs.extension);
        let target_url = base_url
            .join(&filename)
            .context("Failed to construct URL")?;

        let sound_bytes = surf::get(target_url)
            .await
            .anyhow()
            .context("Failed to fetch sound file")?
            .body_bytes()
            .await
            .anyhow()
            .context("Failed to fetch sound file")?;

        let local_path = Path::new(&filename);

        let local_dir = local_path.parent().ok_or(format_err!("Invalid error"))?;
        if !local_dir.exists().await {
            create_dir_all(local_dir)
                .await
                .context("Failed to create directory")?;
        }

        let mut file = File::create(local_path)
            .await
            .and_then(|f| Ok(BufWriter::new(f)))?;
        file.write_all(&sound_bytes).await?;
        file.flush().await?;
    }

    Ok(())
}
