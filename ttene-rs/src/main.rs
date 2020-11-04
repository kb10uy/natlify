mod rpc;

use rpc::{initialize_context, Context};

use async_std::io::{prelude::*, stdin, stdout, BufReader, BufWriter};
use std::io::Cursor as SyncCursor;

use anyhow::Result;
use jsonrpc_core::{Compatibility, Error as JsonRpcError, MetaIoHandler, NoopMiddleware, Params};
use rand::{prelude::*, thread_rng};
use rodio::{Decoder, Source};
use serde_json::Value;

#[async_std::main]
async fn main() -> Result<()> {
    let mut stdin = BufReader::new(stdin());
    let mut stdout = BufWriter::new(stdout());
    let (context, _stream) = initialize_context().await?;

    let mut io_handler = MetaIoHandler::new(Compatibility::V2, NoopMiddleware);
    io_handler.add_method_with_meta("fire_ttene", fire_ttene);

    let mut line_buffer = String::with_capacity(1024);
    while stdin.read_line(&mut line_buffer).await? > 0 {
        let json_str = line_buffer.trim();
        if json_str.is_empty() {
            line_buffer.clear();
            continue;
        }

        let result = io_handler.handle_request(json_str, context.clone()).await;
        if let Some(response) = result {
            stdout.write_all(response.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }

        line_buffer.clear();
    }

    Ok(())
}

async fn fire_ttene(_params: Params, context: Context) -> Result<Value, JsonRpcError> {
    let target_voice = context
        .ttene_voices
        .choose(&mut thread_rng())
        .expect("Should have at least one voice")
        .clone();

    let reader = SyncCursor::new(target_voice);
    let decoder = Decoder::new(reader).map_err(|_| JsonRpcError::internal_error())?;
    context
        .stream_handle
        .play_raw(decoder.convert_samples())
        .map_err(|_| JsonRpcError::internal_error())?;

    Ok(Value::Null)
}
