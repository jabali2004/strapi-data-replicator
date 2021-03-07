# Strapi Data Replicator

[![build](https://github.com/jabali2004/strapi-data-replicator/actions/workflows/build.yml/badge.svg)](https://github.com/jabali2004/strapi-data-replicator/actions/workflows/build.yml)

> Simple command line utility for replicating and migrating persistent tables or collections for Strapi applications.

The project is currently still in beta phase and more a conceptual proof than a finished product.

## About

The Strapi Data Replicator is a Tool provided as a binary or windows installer and is written in Rust for
syncing specific database tables or collections.

The main goal is to simplify development with Strapi and enable easy and automated deployment.

## Requirements

> Supported environments are Linux (amd64) and Windows.
The tool has not yet been tested under MacOS.

A statically linked binary is also available to ensure use within a container.

> Currently supported databases are MySQL, MariaDB and MongoDB

A strict config schema is needed for the automatic configuration creation when using sql databases. Cleanup your tables and remove unused or wrongly ordered columns before replicating.

This can be achieved with the help of KnexJS migrations.

> PostgreSQL is currently not supported.

## Features

Currently implemented features:

- Automatic creation of a project configuration.
- Replicating tables and converting them into a storable format which can be stored in the repository.
- Migrate tables or collections of persistent data for development or release.

## Upcoming features

Planned features:

- Add Prostgres support.
- Calculate delta of persistent data or see if the data are exactly the same. Then the data do not have to be to be stored again.
- Switching to sqlx and removing the mysqldump dependencies.
- Enable MySQL and MariaDB connection over Unix sockets.

## Installation

### Installation on Linux

Add "strapi-data-replicator" and "mysqldump" together to the project or install locally to /usr/local/bin.

### Installation on Windows

Either the files can be added to the project or "strapi-data-replicator.msi" can be installed locally.

## Commands

```` none
USAGE:
    strapi-data-replicator [FLAGS] [OPTIONS] <command> [path]

FLAGS:
    -h, --help       Prints help information
    -f, --force      Overwrite existing project configuration
    -e, --env        Use environment variables
    -V, --version    Prints version information

OPTIONS:
    -v, --replication-version <replication-version>     [default: ]

ARGS:
    <command>    Available commands: init, replicate, migrate, info
    <path>        [default: .]
````

## Build yourself

> Build release binary

```` sh
cargo build --release
````

> Build statically linked binary

```` sh
cargo build --target=x86_64-unknown-linux-musl --features vendored --release
````

## Config File

For the automatic creation of the project configuration, certain environment variables are required in the environment files.

There is also a mode using **--env** where only environment variables are used, which is suitable for deployments.

```` sh
strapi-data-replicator migrate --env
````

Environment variables used:

- DATABASE_TYPE= Example -> mysql | mongodb
- DATABASE_VERSION= Example -> 5.8
- DATABASE_HOST= Example -> 127.0.0.1
- DATABASE_PORT= Example -> 3306
- DATABASE_NAME= Example -> spm
- DATABASE_USERNAME= Example -> root
- DATABASE_PASSWORD= Example -> password
- DATABASE_SSL: Example -> false

Replication config file (**replicate.json**):

```` json
{
  "strapi_version": "3.2.2",
  "database": {
    "database_type": "mysql",
    "database_version": "5.8",
    "database_name": "spm",
    "host_information": {
      "address": "127.0.0.1",
      "port": "3306",
      "username": "root",
      "password": "password",
      "ssl" : true
    }
  },
  "replicated": [
    "users",
    "permissions"
  ]
}

````

## Datastore

The persistent data is stored under ./.replicated as JSON or SQL files.

## Contribute

1. Fork it (https://github.com/jabali2004/strapi-data-replicator/fork)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
