use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    options::{ClientOptions, FindOptions},
    Client,
};
use std::process;
use std::env;
use structopt::StructOpt;


trait Finder {}

struct Database {}

impl Finder for Database {}

#[derive(Debug, StructOpt)]
#[structopt(name = "mongoc-rs", about = "Quickly view your mongodb")]
struct Opt {
    /// The port mongodb client will use
    #[structopt(long = "port", short = "p", default_value = "27017")]
    port: u8,

    /// The location mongodb is at
    #[structopt(long = "location", short = "l", default_value = "localhost")]
    location: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    client_options.app_name = Some("mongoc-rs".to_string());
    let client = Client::with_options(client_options)?;

    Opt::from_args();
    Ok(())
}
