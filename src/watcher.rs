use async_std::{channel::{Receiver, bounded}, task};
use notify::{RecursiveMode, RecommendedWatcher, Event, Watcher, Config};
// notify::watcher.watch() wants std::PathBuf and I haven't figured out how to use
// async_std::PathBuf with it.
use std::path::PathBuf;

pub async fn watch(paths: Vec<PathBuf>) -> notify::Result<()> {
    let (mut watcher, rx) = generate_watcher()?;

    // Canonicalize paths before adding them to the watch list,
    // Honestly not sure if that's needed, but I like absolute paths.
    for path in paths.into_iter() {
        if let Ok(canonical_path) = path.canonicalize() {
            watcher.watch(&canonical_path, RecursiveMode::NonRecursive)?;
        }
    }

    while let Ok(res) = rx.recv().await {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn generate_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = bounded(1);

    // notify recommends using RecommendedWatcher to automatically pick the best
    // implementation for the platform, even though we're only building for
    // windows.
    let watcher = RecommendedWatcher::new(move |res| {
        task::block_on(async {
            tx.send(res).await.unwrap();
        })
    })?;
    Ok((watcher, rx))
}
