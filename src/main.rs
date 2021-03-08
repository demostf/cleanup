use chrono::{Duration, Utc};
use demostf_client::{ApiClient, ListOrder, ListParams};
use main_error::MainError;
use std::fs::remove_file;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let key = dotenv::var("DEMOSTF_KEY").expect("DEMOSTF_KEY not set");
    let root = PathBuf::from(dotenv::var("DEMOS_ROOT").expect("DEMOS_ROOT not set"));

    let client = ApiClient::new();

    let demos = client
        .list(
            ListParams::default()
                .with_order(ListOrder::Ascending)
                .with_backend("freezer"),
            1,
        )
        .await?;

    let cutoff_time = Utc::now() - Duration::days(3 * 365);

    for demo in demos {
        if demo.time > cutoff_time {
            break;
        }

        let path = root.join(&demo.path.trim_start_matches('/'));

        if !path.exists() {
            eprintln!("Demo not found: {}", path.to_str().unwrap());
            break;
        }

        client
            .set_url(demo.id, "deleted", "", "", demo.hash, &key)
            .await?;
        remove_file(&path)?;
        println!("{} {}", demo.id, demo.name);
    }

    Ok(())
}
