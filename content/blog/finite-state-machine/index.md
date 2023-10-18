+++
title = "Godot Recipe: Finite State Machine"
date = 2023-10-13
description = "Ever wanted to create complex player behaviour? Let's do it!"
draft = true

[taxonomies]
tags = ["Godot", "recipe", ".NET"]

[extra]
footnote_backlinks = true
quick_navigation_buttons = true
+++

{% mermaid() %}
stateDiagram-v2
direction LR
    state Grounded {
        Walking --> Idle: has zero velocity
        Idle --> Walking: has non-zero velocity
    }
    
    Grounded --> Jumping: jumped

    state Airborne {
        Jumping --> Falling: velocity.Y > 0
    }

    Falling --> Idle: isGrounded

{% end %}

> This post wouldn't have been possible without the great book [Game Programming Patterns, by Robert Nystrom](https://gameprogrammingpatterns.com/). If you're an up and coming game developer, you **should consider get it**!
