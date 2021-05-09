use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    options::{ClientOptions, FindOptions},
    Client,
};

trait Finder {}

struct Database {}

impl Finder for Database {}

fn version() {}

fn usage() {}

fn parse_args() {}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    Ok(())
}
