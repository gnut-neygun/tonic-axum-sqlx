# ObjectAPI

A sample project that provides a gRPC + REST service for managing an object store, supporting operations such as
create, read, update, delete, list and subscribe. This project uses:

- [Axum](https://github.com/tokio-rs/axum) web framework for REST request handling
- [Tonic](https://github.com/hyperium/tonic) as a gRPC server
- [SQLx + Postgres](https://github.com/launchbadge/sqlx) for the data layer, providing compile-time checked SQL queries
- [OpenAPI + Swaggger UI](https://swagger.io/tools/swagger-ui/) for API documentation. The OpenAPI spec is
  auto-generated from the protobuf definitions.
-
- [Buf CLI](https://buf.build/docs/tutorials/getting-started-with-buf-cli) for Protobuf API management
- [Python-Betterproto](https://github.com/danielgtaylor/python-betterproto) for human-readable python client
  generated from protobufs.
- [Connect-Web](https://connect.build/) for generation of Typescript web client from protobufs.
- [Protoc-Gen-OpenAPI](https://github.com/google/gnostic/tree/main/cmd/protoc-gen-openapi) for generating OpenAPI
  Schema from proto files.

## Building

### Install Rust toolchain

Head to https://www.rust-lang.org/learn/get-started and follow the instructions to install Rustup, the Rust
installer and version management tool, then follow the on-screen instruction. Specifically, run:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

We use the latest stable version of Rust. Rustup will install the Rust Compiler (rustc) and the Rust package manager (
cargo).

### Install Protobuf Compiler

We use [Protoc](https://grpc.io/docs/protoc-installation/) with additional plugins to generate Python, Typescript
and OpenAPI schemas from protobuf. To install Protoc, run:

`sudo apt update && sudo apt upgrade -y
sudo apt install -y protobuf-compiler libprotobuf-dev`

Then make sure protobuf is in your path by running: `protoc --version`

### Building the project

With Rust toolchain and Protoc compiler installed, now you are ready build rust code. Just run:

`SQLX_OFFLINE=true cargo build`

And cargo would download the depdencies and get the project built. The environment variable SQLX_OFFLINE tells
SQLx not to compile check SQL queries for now, because we haven't had a database setup.

## Database Setup

### Docker toolchain

First, install the docker engine: https://docs.docker.com/engine/install

You may choose to use Docker Desktop setup instead. But it's really not needed and recommended on Linux.

Then install docker compose: https://docs.docker.com/compose/install/linux/#install-using-the-repository

### Running the database

After docker toolchain is installed, run:

`docker compose up -d`

Would give you a database instance. For the database credentials, refer to the *docker-compose.yml* file

### Running migrations

Install the SQLx CLI with:

`cargo install sqlx-cli`

Then run:

`sqlx migrate run`

To do the database migration. It will look at the database creditials in the *.env* file.

## Running the project

First check if the database is ready by just running:

`cargo build`

This time without SQLX_OFFLINE.

Then run:

`cargo run`

This would run the main function in *src/main.rs* and start the server. Head to http://localhost:3000/ to see the
SwaggerUI Docs.

## Code generation

If you change the protobuf definitions located under proto, you have to update the generated code. By default, when
one executes cargo build, build.rs would be automatically invoked by cargo and in there we generate the rust code
using [Prost](https://docs.docker.com/compose/install/linux/#install-using-the-repository). But to regenerate the
OpenAPI schema, TS and Python code, we need to install additional protoc plugins.

### Install Homebrew (Optional)

For several of the installation steps below, we recommend using homebrew to install the required tools because it is
easiest. If you don't want to install brew, you may head to their main page and follow the instructions there instead.
To
install [brew](https://brew.sh/index_de), run:

`/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`

### Install Buf CLI

First, install Buf CLI for easy management of Proto code generation. Run:

`brew install bufbuild/buf/buf`

### Install the Go compiler

The protoc-gen-openapi plugin is written in Go so we need it to install the Go Compiler, run:

`brew install go`

### Install OpenAPI Code Generator

Run:

`go install github.com/google/gnostic/cmd/protoc-gen-openapi`

And make sure that protoc-gen-openapi is on your $PATH. The go tool will by default install the binary under *~
/go/bin/protoc-gen-openapi* folder

### Install Python Code Generator

You would need to have your Python toolchain installed. We recommend installing Anaconda and creating a new virtual
Anaconda environment. After that, run:

`pip install betterproto`

Make sure betterproto is on your $PATH (by activating the Python environment, if you haven't already)

### Generate Code

Now you are ready to generate all the code. The plugin for Typescript is not needed to install locally because it's
already in provided in Buf repository. Run:

`buf generate`

And check the output in the *generated* folder

## Task runner

We use [cargo-make](https://github.com/sagiegurari/cargo-make) to manage different project tasks from a centralized
command line interface. To install it, run:

`cargo install --force cargo-make`

We provide different tasks:

- `cargo make doc` : Generate documentation from code and open it in a browser
- `cargo make build` : Re-generate code with buf, rebuild the web grpc interface and build the rust code
- `cargo make generate`: Re-generate code using Buf CLI

Refer to *Makefile.toml* to see all the available tasks