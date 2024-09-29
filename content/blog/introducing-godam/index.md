+++
title = "Introducing godam"
date = 2024-09-29
description = "Thoughts and reflections after releasing the first version of godam."

[taxonomies]
tags = ["rust", "cli", "godot", "addons", "pipelines"]

[extra]
footnote_backlinks = true
quick_navigation_buttons = true
+++

> 🙏🏼 Big thanks to [imjp94](https://github.com/imjp94) for creating [gd-plug](https://github.com/imjp94/gd-plug) and the team at [ChickenSoft Games](https://chickensoft.games/) for their tool [GodotEnv](https://github.com/chickensoft-games/GodotEnv). Both have been great inspirations for **godam**.

# Introduction

My sister and I rely on having a playable game build updated and available at all times. This includes installing addons in a non-user environment and deploying to [itch.io](https://itch.io/).

Tools like **gd-plug** and **GodotEnv** have made addon management easier, but we've hit a snag. They don’t yet support installing addons like the Godot editor does it — from zip archives. This is how many Godot assets are packaged, like the popular [LimboAI Extension](https://github.com/limbonaut/limboai).

Unfortunately, there’s no official Godot command-line tool yet to automate this process for CI/CD pipelines.

This is where [godam](https://github.com/nilsiker/godam) comes in — a command-line tool designed to install Godot addons just like the editor does, directly from zip archives pulled from the Godot AssetLib.

![the godam logo, an orange godot robot silhouette head, captioned "godam"](/img/godam-big.png)

At the bottom of this post, you'll find references on how to [get started with godam](#getting-started-with-godam). But first, let's look at what makes it tick!

# Creating command-line tools using clap

**godam** is implemented using the [clap](https://docs.rs/clap/latest/clap/) crate. It truly makes the user interface a breeze. For instance, this is my main:

```rs
use clap::Parser;
use godam::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = godam::run(&cli.command).await {
        eprintln!("godam: {e}");
    }
```

Before looking at a user command, let's look at the `Cli` struct and what the `godam::run` function does.

```rs
// mod statements omitted

use clap::Parser;
use commands::*;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
#[command(propagate_version = true)]
/// godam
///
/// A minimal addon manager for Godot.
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub async fn run(command: &Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Init => init::exec()?,
        Command::Search { name } => search::exec(name).await?,
        Command::Install { name } => install::exec(name).await?,
        Command::Uninstall { name } => uninstall::exec(name)?,
        Command::List => list::exec()?,
        Command::Clean => clean::exec()?,
    };

    Ok(())
}
```

It's just a simple match, each arm a command that **godam** supports. The `exec` functions contains the logic that is executed when running a command, taking in parameters that are parsed from the user input.

To see the actual *magic*, we have to look at the `Command` struct.

```rs
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    #[command()]
    /// Initializes your Godot project to use godam as your addon manager
    Init,
    /// Searches the Godot Asset Library API for assets by name.
    #[command(alias = "s")]
    Search {
        #[arg(index = 1)]
        name: String,
    },
    /// Installs the specified addon to your Godot project, adding it to the godam configuration.

    #[command(alias = "i")]
    Install {
        /// The name of the asset you want to install
        #[arg(index = 1)]
        name: Option<Vec<String>>,
    },
    /// Uninstalls the specified addon from your Godot project, removing it from the godam configuration.
    #[command(alias = "u")]
    Uninstall {
        /// The name of the asset you want to uninstall
        #[arg(index = 1)]
        name: Option<String>,
    },
    /// Lists all assets being managed by Godam
    #[command(alias = "ls", alias = "l")]
    ///
    List,
    /// Cleans the godam cache folder
    #[command(alias = "c")]
    Clean,
}
```

By deriving from `Subcommand`, these roughly 30 humble lines specifies the entire interface:

* Subcommands are created by adding enum variants:

  > `Command::Init` allows us to run `godam init`

* Subcommands can be configured using the attribute macro `#[command(...)]`:

  > For example, adding an alias allows us to run `godam install` as `godam i`.

* Arguments to subcommands are added using enum variant fields:

  > `godam search limboai` will assign `"limboai"` to the `name` field

* Help texts can even be provided by adding doc comments to the enum variants:

  > `godam search -h` will display not only the text specified in the doc comment, but also outline the arguments and usage.
  > ```bash
  > $ godam search -h
  > Searches the Godot Asset Library API for assets by name
  >
  > Usage: godam.exe search <NAME>
  >
  > Arguments:
  >  <NAME>
  >
  > Options:
  >  -h, --help     Print help
  >  -V, --version  Print version
  >
  >```

I cannot praise `clap` enough - it is unfathomably ergonomic and takes all the headache out of argument parsing! I'm using a fraction of its capabilities - `godam` is a fairly simple application.

For completeness sake, let's quickly showcase the `godam::commands::search::exec` function:

```rs
use thiserror::Error;

use crate::{
    godot::{
        asset_library::{get_assets_by_name, AssetLibraryError, AssetSearchResult},
        project::{get_version, GodotProjectError},
    },
    info,
};

#[derive(Error, Debug)]
pub enum SearchError {
    #[error(transparent)]
    Godot(#[from] GodotProjectError),
    #[error(transparent)]
    Request(#[from] AssetLibraryError),
}

pub async fn exec(asset_name: &str) -> Result<(), SearchError> {
    let version = get_version()?;
    let assets = get_assets_by_name(asset_name, &version).await?;

    for AssetSearchResult { title, asset_id } in &assets {
        info!("{asset_id}: {title}");
    }

    Ok(())
}
```

`exec` is nothing but a wrapper, calling functions in various service modules contained in `godam`, 

Firstly, it gets the Godot project version using the `godot::project::get_version` function, parsing it from the `project.godot` file in the current working directory.

Secondly, it calls the `godot::asset_library::get_assets_by_name` function, which makes an API request to the Godot Asset Library, finding assets filtering on the provided `asset_name`. 

Finally, it prints out each found asset on its own line, using an `info!` macro which is just a styled `println!` I made using the `console` crate.

> 🙋🏼 And so the story goes for all other commands. The `install` command might need some love further down the line, but let's forget about that for now...

# Getting started with godam

You can find **godam** in its [public repository over at GitHub](https://github.com/nilsiker/godam). Here you can read more about it and how to download it. Most importantly, you can [leave feedback and suggestions](https://github.com/nilsiker/godam/issues) to help improve it!

For general usage, check out the [README](https://github.com/nilsiker/godam/blob/main/README.md).

If you're looking for pipeline use cases, I'll include an example itch.io deploy workflow below. The docker image used is based on the [barichello/godot-ci](https://hub.docker.com/r/barichello/godot-ci) image.

```yaml
name: "build-and-deploy-to-itch"
on: 
  push:
    branches:
      - develop
    paths-ignore:
      - "*.md"

env:
  GODOT_VERSION: 4.3
  ITCH_USER: your_user
  EXPORT_NAME: our_project_name

jobs:
  export-web:
    name: Web Export
    runs-on: ubuntu-latest
    container:
      image: docker://nilsiker/godam:v0.1.1-godot4.3
    steps:
      - name: Checkout
        uses: actions/checkout@v1
      - name: Setup directories
        run: |
          mkdir -v -p build/web ~/.local/share/godot/export_templates
          mv  /root/.local/share/godot/export_templates/${GODOT_VERSION}.stable ~/.local/share/godot/export_templates/${GODOT_VERSION}.stable
      - name: Install addons (godam)
        run: |
          cd project
          godam install
      - name: Web Build
        run: |
          cd project
          godot --headless -v --export-release "Web" ../build/web/index.html
      - name: Upload Artifact
        uses: actions/upload-artifact@v4
        with:
          name: web
          path: ./build/web/
      - name: Deploy to itch.io
        run: |
          butler push ./build/web/ ITCH_USER/${EXPORT_NAME}:web
        env:
          BUTLER_API_KEY: ${{ secrets.ITCHIO_API_KEY }}
```

# What's next?

Currently **godam** only supports installing addons following the zip folder structure `/addons/name_of_addon/**/*` and `/name_of_addon/**/*`. Any other zip structure will throw an error.

Moving forward, it would be nice to handle any Godot AssetLib asset, and extract them tidily into the Godot project structure.

For now, I'm focusing on stabilizing what's already in place. It's early days for **godam**, and I suspect the API will change more often than what is comfortable. For this to turn out well, I need to use the tools more myself.

If I could make a wish, **godam** would see some user adoption from within the Godot community. Everyone is welcome to pitch in and help improve it.

To make **godam** the best tool it can be, I need more eyes on it - so let's make something cool! ☀️

*Thanks for reading,<br/>Nilsiker*