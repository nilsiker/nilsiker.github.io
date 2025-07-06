+++
title = "First Look: SpacetimeDB in Godot"
date = 2025-03-16
description = "Trying out SpacetimeDB in a minimal Godot demo project"
draft = true

[taxonomies]
tags = ["Godot", "recipe", "extension", "rust", "spacetimedb", "spacetimedb-cli"]

[extra]
footnote_backlinks = true
quick_navigation_buttons = true
+++

# Introduction

The people over at Clockwork Labs [recently announced version 1.0 of SpacetimeDB](https://www.youtube.com/watch?v=kzDnA_EVhTU&), their in-memory database/server used to develop their game BitCraft.

<a href="https://spacetimedb.com" style="">
    <img src="image-1.png" />
    <img src="image.png" />
    <h2 style="text-align: center;">Multiplayer at the speed of light.</br></br></h2>
</a>

Should you be excited? I don't know yet...

It's allegedly *multiplayer at the speed of light* - that might scratch the Rust ego!

It's self-described as *the easiest way to build a multiplayer game* - that sounds **too good to be true**!

I'm not a multiplayer game developer by any stretch of the imagination. Could SpacetimeDB change that?

What would it look like using SpacetimeDB in Godot?

Let's find out...

# Scope

The prototype is simple:

* The prototype consists of a game client (Godot executable) and a server (SpacetimeDB)
* The player controls a ball in a confined area
* The client takes player input and sends it to the server
* The server acts on the player input and moves the ball
* The ball position is synced back to the client

## Prerequisites

I'll be using Godot v4.4. If you're in the ballpark of that there shouldn't be any glaring issues coding along.

We need to install the SpacetimeDB CLI. Check out the [install section](https://spacetimedb.com/install) over at the SpacetimeDB website for instructions.

Alternatively, you can install from source, [but Clockwork Labs currently does not recommend it](https://github.com/clockworklabs/SpacetimeDB/blob/26c1387512cd524af3a70bdeae8906c4661284b0/README.md#installing-from-source). I'll opt out of the free coffee break this time, use the install script on Windows:

```sh
iwr https://windows.spacetimedb.com -useb | iex
```

# Implementation

We'll start of by making an minimal game client without thinking about the server. I'll call it **Spaceball**.

Secondly, we'll implement SpacetimeDB modules to handle player input and ball positions. We'll try them out using the SpacetimeDB CLI.

Lastly, we'll write a Godot extension that allows us to interface with the SpacetimeDB server and have the ball position update based on the server.

Sounds easy - might not be. Let's get going!

## Folder structure

I'll prepare three folders in our project for the client, the extension and the server.

```sh
spaceball
├── client      # godot client
├── extension   # godot extension to interface with server
└── server      # spacetimedb
``` 

## Game Client

Let's keep this simple. 