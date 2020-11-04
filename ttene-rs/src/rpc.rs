use async_std::{
    fs::{read_dir, File},
    io::{prelude::*, BufReader},
    prelude::*,
    sync::Arc,
};

use anyhow::Result;
use jsonrpc_core::Metadata;
use rodio::{OutputStream, OutputStreamHandle};

#[derive(Clone)]
pub struct Context {
    pub ttene_voices: Arc<Box<[Box<[u8]>]>>,
    pub stream_handle: OutputStreamHandle,
}

impl Metadata for Context {}

pub async fn initialize_context() -> Result<(Context, OutputStream)> {
    let (stream, stream_handle) = OutputStream::try_default()?;

    let mut dir = read_dir("./sounds").await?;
    let mut ttene_voices = vec![];
    while let Some(dir_entry) = dir.next().await {
        let file = File::open(dir_entry?.path()).await?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer).await?;

        ttene_voices.push(buffer.into_boxed_slice());
    }
    let ttene_voices = Arc::new(ttene_voices.into_boxed_slice());

    let context = Context {
        ttene_voices,
        stream_handle,
    };
    Ok((context, stream))
}
