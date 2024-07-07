use anyhow::Result;
use clap::Parser;
use futures::future::try_join_all;
use ic_agent::Agent;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::WalkDir;

mod storage;

const CHUNK_SIZE: usize = 2000; //2_000_000;

#[derive(Parser)]
struct Opts {
    path: PathBuf,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let agent = Agent::builder().with_url("http://localhost:4943").build()?;
    agent.fetch_root_key().await?;
    let service = storage::Service(
        candid::Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai")?,
        &agent,
    );
    let mut size = CHUNK_SIZE;
    let mut blob = Vec::with_capacity(CHUNK_SIZE);
    let mut item = Vec::new();
    let mut id = 0;
    let mut futures = Vec::new();
    for p in WalkDir::new(&opts.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
    {
        let metadata = fs::metadata(p.path())?;
        let timestamp = metadata
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let mut len = metadata.len() as usize;
        let mut f = fs::File::open(p.path())?;
        let mut data_type = storage::DataType::New;
        let key = format!("/{}", p.path().strip_prefix(&opts.path)?.display());
        println!("{} {} {}", key, p.path().display(), len);
        while size < len {
            let mut buf = vec![0; size];
            f.read_exact(&mut buf)?;
            blob.extend_from_slice(&buf);
            len -= size;
            item.push(storage::Item {
                key: key.clone(),
                data_type,
                len: size as u32,
                timestamp,
            });
            futures.push(upload_blob(&service, id, blob.clone(), item.clone()));
            blob.clear();
            item.clear();
            size = CHUNK_SIZE;
            data_type = storage::DataType::Append;
            id += 1;
        }
        size -= len;
        f.read_to_end(&mut blob)?;
        item.push(storage::Item {
            key,
            data_type,
            len: len as u32,
            timestamp,
        });
    }
    futures.push(upload_blob(&service, id, blob, item));
    try_join_all(futures).await?;
    service.commit().await?;
    Ok(())
}

async fn upload_blob(
    service: &storage::Service<'_>,
    id: u32,
    blob: Vec<u8>,
    item: Vec<storage::Item>,
) -> Result<()> {
    eprintln!("{:?}", item);
    service
        .upload(
            id,
            storage::UploadData {
                blob: blob.into(),
                item,
            },
        )
        .await?;
    Ok(())
}
