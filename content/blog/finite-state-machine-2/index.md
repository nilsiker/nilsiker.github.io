+++
title = "Godot Recipe: Finite State Machine #2"
date = 2023-10-20
description = "Solving circular dependencies and promoting loose coupling"

[taxonomies]
tags = ["Godot", "recipe", "C#", "dependency inversion", "finite state machine", "character controller", "namespaces", "abstraction", "lsp"]

[extra]
footnote_backlinks = true
quick_navigation_buttons = true
+++

> ⚠️ This post assumes you have a basic understanding of Nodes and Scenes in Godot and some familiarity with C# syntax if you plan to code along!

Picking up [where we left off](/blog/finite-state-machine-1) a class overview of our FSM looks like this:

{% mermaid() %}
classDiagram
direction LR
    class StateMachine {
        + ChangeState(State) void
    }
    class State {
        - _name : string
        + Enter() void
        + Tick(double) void
        + PhysicsTick(double) void
        + Exit() void
    }
    State --> StateMachine
    StateMachine --> WalkingState
    StateMachine --> IdleState
    StateMachine --> State
    StateMachine --> Mover
    <<abstract>> State
    WalkingState --|> State
    IdleState --|> State
    Mover --> RigidBody3D
    namespace Godot {
        class RigidBody3D
        class Node
    }
{% end %}

It might not look *too bad* at this point. However, if we were to add more systems, more states and (cue foreshadowing) a way for our states to handle external events, it will swiftly become a breeding ground for logical errors.

My pain points at the moment are:

1. `State` and `StateMachine` forms a circular dependency
2. `WalkingState` need to access `Mover` through the StateMachine
3. States need to access other states through the StateMachine
4. Our classes are depending too much on concrete classes, creating tight coupling
5. We're not using any organizational tools to categorize our classes

Let's make a plan to solve this!

# Scope

The goal for this post is to utilize the Dependency Inversion Principle (DIP) and Liskov's Substition Principle (LSP) to kill of our circular dependency and make our code more loosely coupled. In addition to this, we'll organize or classes into namespaces to give our codebase some well-needed structure.

> 🙋🏼 If you're like me and come from a software engineering background, seeing DIP and LSP is the equivalent of dusting of an old book from the shelf - likely a book on SOLID principles from university times.

We'll end up with something that looks like this:


{% mermaid() %}
graph
    subgraph FSM
        IState
        IHasSystems
        IHasStates
        State--implements-->IState
        State--has-->IHasSystems
        State--has-->IHasStates
        subgraph Control2[Control]
            State
            IdleState
            WalkingState
            StateMachine
            IdleState--inherits-->State
            WalkingState--inherits-->State
        end
        StateMachine--has-->IState
        StateMachine--implements-->IHasStates
        StateMachine--implements-->IHasSystems
    end
    subgraph Systems
        subgraph Control
        Mover
        end
        IMover
        Mover--implements-->IMover
    end
    WalkingState--has-->IMover
    Mover--has-->RigidBody3D
    StateMachine--inherits-->Node
    subgraph Godot
        Node
        RigidBody3D
    end
{% end %}

# Solving circular dependencies with DIP

DIP stipulates that a high-level class mustn't depend on a lower-level class. The goal is to create a more robust and maintainable solution. 

In our current setup, I think of the state as a `lower-level class` and the machine is a `higher-level class`. States should not necessarily need a machine to exist, but a machine is pretty much useless without states!

What does this look like in practice? We could introduce an abstraction for states like so:

{% mermaid() %}
classDiagram
    State--|>IState
    <<interface>> IState
{% end %}


## The final result

sssss

All the best,<br/>
Nilsiker