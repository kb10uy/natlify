use crate::{
    cli::{Arguments, SubCommand},
    extensions::SurfErrorExt,
};

use anyhow::{Context, Result};
use async_std::io::{prelude::*, stdout, BufWriter};
use regex::Regex;
use scraper::{Html, Selector};
use url::Url;

pub async fn list_buttons(args: &Arguments) -> Result<()> {
    let subargs = if let SubCommand::List(list) = &args.sub_command {
        list
    } else {
        unreachable!("Invalid subcommand state")
    };

    let url =
        Url::parse(&format!("https://{}/", args.domain)).context("Failed to construct URL")?;

    let query = if let Some(qs) = &subargs.query {
        let regex = Regex::new(qs).context("Failed to parse Regex")?;
        Some(regex)
    } else {
        None
    };

    let html_document = {
        let html = surf::get(url)
            .await
            .anyhow()
            .context("Failed to fetch HTML")?
            .body_string()
            .await
            .anyhow()
            .context("HTML was not UTF-8 document")?;
        Html::parse_document(&html)
    };
    let button_selector = Selector::parse("button.sounds").expect("Selector error");

    let mut stdout = BufWriter::new(stdout());
    for button in html_document.select(&button_selector) {
        let element = button.value();
        let data_file = match element.attr("data-file") {
            Some(path) => path,
            None => continue,
        };

        match &query {
            Some(re) => {
                if re.is_match(data_file) {
                    stdout.write_all(data_file.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                }
            }
            None => {
                stdout.write_all(data_file.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
            }
        }
    }

    stdout.flush().await?;
    Ok(())
}
