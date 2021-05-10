use futures::stream::StreamExt;
use mongodb::{bson::Document, error::Error, options::ClientOptions, Client, Database};
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;
use termion::{color, style};

async fn list_databases(client: &Client) -> Result<Vec<String>, Error> {
    let databases: Result<Vec<String>, Error> = client.list_database_names(None, None).await;
    return databases;
}

async fn list_collections(database: &Database) -> Result<Vec<String>, Error> {
    let collections: Result<Vec<String>, Error> = database.list_collection_names(None).await;
    return collections;
}

fn print_alias_line(alias: String, item: String) {
    println!(
        "{}{}{}{}:{}\t{}",
        color::Fg(color::Red),
        style::Bold,
        alias,
        style::Reset,
        color::Fg(color::Reset),
        item
    );
}

fn letter_print(items: &Vec<String>) {
    let mut index: u32 = 10;
    for item in items {
        print_alias_line(
            std::char::from_digit(index, 16).unwrap().to_string(),
            item.to_string(),
        );
        index += 1;
    }
}

fn enumerate_print(items: &Vec<String>) {
    let mut index: u32 = 0;
    for item in items {
        print_alias_line(index.to_string(), item.to_string());
        index += 1;
    }
}

fn normal_print(items: &Vec<String>) {
    for item in items {
        println!("{}", item);
    }
}

fn print(items: &Vec<String>, enumerate: bool, numeric: bool) {
    if enumerate {
        if numeric {
            enumerate_print(items);
        } else {
            letter_print(items);
        }
    } else {
        normal_print(items);
    }
    print!("\n");
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

async fn show_databases(client: &Client) -> Vec<String> {
    let databases = list_databases(&client).await.unwrap();
    print(&databases, true, true);
    return databases;
}

async fn show_collections(
    client: &Client,
    databases: &Vec<String>,
    database_alias: String,
) -> (Database, Vec<String>) {
    let index: usize = database_alias.parse().unwrap();
    let database = client.database(&databases[index]);
    let collections = list_collections(&database).await.unwrap();
    print(&collections, true, false);
    return (database, collections);
}

async fn show_documents(
    database: &Database,
    collections: &Vec<String>,
    collection_alias: String,
) -> mongodb::error::Result<()> {
    let collection_alias_chars: Vec<char> = collection_alias.chars().collect();
    let collection_alias_char: char = collection_alias_chars[0];
    let index: usize = collection_alias_char as usize - 97;

    let collection = database.collection::<Document>(&collections[index]);
    let mut cursor = collection.find(None, None).await?;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                println!("{}", document);
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

fn get_alias() -> String {
    let mut alias = String::new();
    print!("Enter an alias: ");
    let _flushed = stdout().flush();
    stdin().read_line(&mut alias).unwrap();
    // Remove the trailing newline
    alias.pop();
    return alias;
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

    let databases: Vec<String> = show_databases(&client).await;
    let database_alias = get_alias();

    let (database, collections) = show_collections(&client, &databases, database_alias).await;
    let collection_alias = get_alias();

    let _res = show_documents(&database, &collections, collection_alias).await;

    Ok(())
}
