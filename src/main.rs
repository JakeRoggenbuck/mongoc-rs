use futures::executor::block_on;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    error::Error,
    options::{ClientOptions, FindOptions},
    Client, Collection, Database,
};
use structopt::StructOpt;

async fn list_databases(client: &Client) -> Result<Vec<String>, Error> {
    let databases: Result<Vec<String>, Error> = client.list_database_names(None, None).await;
    return databases;
}

async fn list_collections(database: &Database) -> Result<Vec<String>, Error> {
    let collections: Result<Vec<String>, Error> = database.list_collection_names(None).await;
    return collections;
}

fn enumerate_print(items: Vec<String>) {
    let mut index: u32 = 10;
    for item in items {
        println!("{}:\t{}", std::char::from_digit(index, 16).unwrap(), item);
        index += 1;
    }
}

fn letter_print(items: Vec<String>) {
    let mut index: u32 = 0;
    for item in items {
        println!("{}:\t{}", index, item);
        index += 1;
    }
}

fn normal_print(items: Vec<String>) {
    for item in items {
        println!("{}", item);
    }
}

fn print(items: Vec<String>, enumerate: bool, numeric: bool) {
    if enumerate {
        if numeric {
            enumerate_print(items);
        } else {
            letter_print(items);
        }
    } else {
        normal_print(items);
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "mongoc-rs", about = "Quickly view your mongodb")]
struct Opt {
    /// The port mongodb client will use
    #[structopt(long = "port", short = "p", default_value = "27017")]
    port: u32,

    /// The location mongodb is at
    #[structopt(long = "location", short = "l", default_value = "localhost")]
    location: String,

    #[structopt(long = "verbose", short = "v")]
    verbose: bool,
}

fn make_url(port: u32, location: String) -> String {
    return "mongodb://".to_owned() + &location + ":" + &port.to_string();
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let args = Opt::from_args();

    let client_url = &make_url(args.port, args.location);
    if args.verbose {
        println!("Using url {}", client_url);
    }
    let mut client_options = ClientOptions::parse(client_url).await?;

    client_options.app_name = Some("mongoc-rs".to_string());

    let client = Client::with_options(client_options)?;
    match list_databases(&client).await {
        Ok(databases) => {
            println!("\nDatabases: ");
            print(databases, true, true);
        }
        Err(e) => println!("{}", e),
    }

    let db = client.database("mydb");
    match list_collections(&db).await {
        Ok(collections) => {
            println!("\nCollections: ");
            print(collections, true, false);
        }
        Err(e) => println!("{}", e),
    }

    Ok(())
}
