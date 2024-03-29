use std::convert::TryFrom;
use std::iter::FromIterator;

use colored::Colorize;
use mongodb::bson::doc;
use mongodb::bson::Bson;
use mongodb::bson::Document;
use mongodb::options::{ClientOptions, Credential, ServerAddress, Tls, TlsOptions};
use mongodb::sync::{Client, Database};
use semver::Version;
use serde_json::{Map, Value};

use crate::consts::SUPPORTED_MONGODB_MAJOR_VERSION;
use crate::modules::types::build_info::BuildInfo;
use crate::modules::types::config::Config;

/// List all tables for given database
pub fn list_tables(config: Config) -> Option<Vec<String>> {
    let database = get_connection(config);
    let collections = database.list_collection_names(None).expect("Error!");

    if collections.is_empty() {
        return Option::from(vec![]);
    }

    return Option::from(collections);
}

/// Return table as json string
pub fn get_table(table_name: String, config: Config) -> Option<String> {
    let database = get_connection(config);
    let collection = database.collection(&table_name);

    let documents = collection.find(None, None).unwrap();

    let mut documents_vec: Vec<Document> = vec![];

    for result in documents {
        let doc: Document = result.expect("Received network error during cursor operations.");
        documents_vec.push(doc);
    }

    let documents_bson = Bson::from_iter(documents_vec);

    let data_json = serde_json::to_string_pretty(&documents_bson).unwrap();

    if data_json.is_empty() {
        return None;
    }

    return Option::from(data_json);
}

// Import collection given as json string
pub fn import_table(json: String, table_name: String, config: Config) -> () {
    let database = get_connection(config);
    let collection = database.collection(&table_name);

    // delete all documents in given collection
    collection
        .delete_many(doc! {}, None)
        .expect("Could not delete documents of collection!");

    let json_vec: Vec<Map<String, Value>> =
        serde_json::from_str(&json).expect("Could not parse json file!");

    let mut documents: Vec<Document> = vec![];

    for j in json_vec {
        let document: Document = Document::try_from(j).expect("Could not convert map to document!");
        documents.push(document);
    }

    collection
        .insert_many(documents, None)
        .expect("Error while inserting multiple collections!");
}

/// Get mongodb connection configuration
fn get_opts(config: Config) -> ClientOptions {
    let mut options = ClientOptions::default();

    let port: u16 = config
        .database
        .host_information
        .port
        .parse()
        .expect("Could not parse port!");

    let server_address: ServerAddress = ServerAddress::Tcp {
        host: config.database.host_information.address,
        port: Option::from(port),
    };

    options.hosts = vec![server_address];

    let mut credentials = Credential::default();

    credentials.username = config.database.host_information.username.into();
    credentials.source = Option::from("admin".to_string());
    credentials.password = config.database.host_information.password.into();

    options.tls = Option::from(match config.database.host_information.ssl {
        true => Tls::Enabled(TlsOptions::default()),
        false => Tls::Disabled,
    });

    return options;
}

/// Create database connection with mongodb server and return ref
fn get_connection(config: Config) -> Database {
    let options = get_opts(config.clone());
    let client = Client::with_options(options).expect("Error while connecting to mongodb server!");

    let database = client.database(config.database.database_name.as_str());

    // Get mongodb server version and check compatibility
    let build_info_doc: Document = database
        .run_command(doc! {"buildInfo": 1}, None)
        .expect("Error while getting mongodb server version!");
    let build_info: BuildInfo = bson::from_bson(Bson::Document(build_info_doc))
        .expect("Error while converting BuildInfo document to bson!");

    // Warn user if mongodb server version is greater than or equal 4.x.x
    let server_version = Version::parse(build_info.version.as_str())
        .expect("Error while parsing mongodb server version!");

    if server_version.major > SUPPORTED_MONGODB_MAJOR_VERSION {
        println!("Your mongodb server version is: {}", build_info.version);
        println!("{}", "Currently only version 3.x.x is supported!".red());
    }

    return database;
}
