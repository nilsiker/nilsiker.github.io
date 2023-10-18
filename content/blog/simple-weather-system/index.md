+++
title = "Godot Recipe: Simple Weather System"
date = 2023-10-13
description = "Conquering locomotion for a third person character controller in Godot, step by step."
draft = true

[taxonomies]
tags = ["Godot", "recipe", ".NET"]

[extra]
footnote_backlinks = true
quick_navigation_buttons = true
+++

Let's create a simple character controller using a floating collider!

But first of, let's determine whether we actually want a floating collider solution or not. In my case, the problems that needs to be solved are:

| Requirement                       | Supported by CharacterBody3D |
| --------------------------------- | ---------------------------- |
| Walk up slopes                    | Yes                          |
| Walk up steps                     | No                           |
| Walk down slopes (no air-time)    | No                           |
| Jumping                           | Yes                          |

The standard CharacterBody3D works wonders for basic movement scenarios. To allow Safetyfor more detailed terrain interactions, I'm goign to opt in to a custom solution using RigidBody3D.

{% mermaid() %}
classDiagram
direction LR
class IGrounder {
«get» + IsGrounded(): bool

    }
    <<interface>> IGrounder

    class Mover {
        + Move(velocity: Vector3) : void
    }
    IGrounder <-- Mover
    IGrounder <-- Animator

{% end %}
