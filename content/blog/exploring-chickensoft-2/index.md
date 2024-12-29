+++
title = "Exploring Chickensoft - LogicBlocks"
date = 2024-12-29
description = "Getting started with Chickensoft C# architecture in Godot."
draft = true

[taxonomies]
tags = ["Godot", "C#", ".NET", "Chickensoft", "fsm", "state machines"]

[extra]
footnote_backlinks = true
quick_navigation_buttons = true
+++

[< part 1](/blog/exploring-chickensoft-1)

> ⚠️ This post assumes you have a basic understanding of Nodes and Scenes in Godot and at least mid-level familiarity with C# syntax if you plan to code along!

# Introduction

In the last part, we downloaded Godot and setup a project from the Chickensoft GodotGame template. Not terribly riveting, but a good start nonetheless!

In this part, we'll dive straight into implementing game logic that ends up being decoupled from the game engine.

To achieve this, we will start off by using LogicBlocks - a hierarchical state machine package for C# by Chickensoft.

# Scope

At the end of this post, we will have finished the following steps:

- Implement two major app states, InMainMenu and InGame.
- Add a blackout-state to introduce fade transitions between scenes!
- Try out refactoring steps when working with LogicBlocks

The goal is to introduce state machines to our project, aiming to build a solid game architecture that we can easily expand on using Chickensoft tools.

# Implementation

I will start of by shouting from the rooftops **_that [LogicBlocks](https://github.com/chickensoft-games/LogicBlocks) is a really cool package!_**

> 🙋🏼 They're all I dreamed of accomplishing with my FSM series. I could consider myself defeated - but frankly it's more so being ✨ blessed ✨.

With LogicBlocks, we can create complex logic for our game nodes by separating it into different states. Part of the value proposition is that we won't create a tangled mess while doing so.

I remember promising simple beginnings, so we **will** be keeping it simple! This means that the LogicBlock usage might seem overkill, but I'll try my best at explaining why we should bother at all!

## Installing LogicBlocks

The packages need to be added to our project. The LogicBlock docs recommends including the introspection and diagram generator for added convenience and to get automatically generated diagrams for your state machines (we'll take a look at that later).

To include the packages in the right spot in our project file, I'll use good old-fashioned copypasting:

```xml
 <ItemGroup>
    <!-- Production dependencies go here! -->
    <PackageReference Include="Chickensoft.LogicBlocks" Version="5.14.0" />
    <PackageReference Include="Chickensoft.Introspection.Generator" Version="2.0.0" />
    <PackageReference Include="Chickensoft.LogicBlocks.DiagramGenerator" Version="5.14.0" />
  </ItemGroup>
```

I'll check my project sanity by hitting F5 in the editor - my game still runs.

## Implementing App states

Earlier, I created `App.cs`. Now, let's put that on our App node and get coding!

```cs
using System;
using Godot;

public partial class App : Node { }
```

Our naked App script looks like this. I'll introduce the LogicBlocks boilerplate piece by piece and explain it as I go.

Let's add the boilerplate and have a look.

```cs
namespace Eggsploration;

using Chickensoft.Introspection;
using Chickensoft.LogicBlocks;
using Godot;

public partial class App : Node {
  #region State
  private AppLogic Logic { get; set; } = default!;
  private AppLogic.IBinding Binding { get; set; } = default!;
  #endregion


  public override void _Ready() {
    Logic = new AppLogic();
    Binding = Logic.Bind();
  }
}

[Meta]
[LogicBlock(typeof(State), Diagram = true)]
public partial class AppLogic : LogicBlock<AppLogic.State> {
  public override Transition GetInitialState() => To<State.InMainMenu>();

  public partial static class Input { }

  public partial static class Output { }

  public abstract partial record State : StateLogic<State> {
    public partial record InMainMenu : State { }
    public partial record InGame : State { }
    public partial record ClosingApplication : State { }
  }
}
```

A definition lightning round might be in order:

- `Logic`: This is a reference to the LogicBlocks state machine. This is mainly used to send inputs into the machine.
- `Binding`: This is a reference to the state binding. It is primarily used to listen to outputs produced by the machine.
- `[Meta]`: This adds some convenience for our LogicBlock, such as automatically preallocating and setting the states that are used in the machine. As for details; it's blinders on for this one! To quote the creators of Chickensoft:
  > You don't need to fully understand this package to make the most of it. In fact, you may never need to use it directly since you are more likely to encounter it as a dependency of one of the other Chickensoft tools.
- `[LogicBlock(typeof(AppLogic), Diagram = true)]`: This attribute extends the class, enabling LogicBlocks generators to generate serialization utilities for the our machine. Setting `Diagram = true` includes the class in the diagram generation.
- `AppLogic`: The LogicBlocks machine that will contain our game logic in various states, taking in input and producing output based on rules we decide.
  - `Input`: A class containing record structs that are used to send pieces of data as input into the machine.
  - `Output`: A class containing record structs that are used to produce outputs from the machine, later to be consumed and reacted to in the `App` node via the `Binding`.
  - `State`: An abstract state class for all our App LogicBlock states.
    - `InMainMenu`: The state for when the app is in the main menu.
    - `InGame`: The state for when the app is running the game.

> 🙋🏼 Later on in the blog series when using AutoInject, we have access to a separate set of lifecycle methods that we should prefer!
>
> For now, I will be using the standard Node lifecycle - by overriding `Ready`.

Let us add some inputs; `NewGameClick` and `QuitClick`.

```cs
public static partial class Input {
  public partial record struct NewGameClick;
  public partial record struct QuitClick;
}
```

The happenings we want to react to are: `StartNewGame` and `QuitApp`. Let's add them to the output!

```cs
public static partial class Output {
  public partial record struct StartNewGame;
  public partial record struct QuitApp;
}
```

## Sending and listening to inputs

Next, let's actually send some input into the machine. We'll connect the `Button.Pressed` signals to handlers through code. At this point, I've added unique names to my buttons to get a hold of the references!

```cs
public partial class App : Node {
  #region State
  private AppLogic Logic { get; set; } = default!;
  private AppLogic.IBinding Binding { get; set; } = default!;
  #endregion

  #region Nodes
  private Button _newGameButton = default!;
  private Button _quitButton = default!;
  #endregion

  public override void _Ready() {
    Logic = new AppLogic();
    Binding = Logic.Bind();

    _newGameButton = GetNode<Button>("%NewGameButton");
    _quitButton = GetNode<Button>("%QuitButton");

    _newGameButton.Pressed += OnNewGameButtonPressed;
    _quitButton.Pressed += OnQuitButtonPressed;
  }

  private void OnNewGameButtonPressed() =>
    Logic.Input(new AppLogic.Input.NewGameClick());

  private void OnQuitButtonPressed() =>
    Logic.Input(new AppLogic.Input.QuitClick());
}

// ... Rest of script omitted!
```

At this point our App script actually sends input into the AppLogic machine! However, we haven't told our states to listen to any inputs yet.

Let's fix that by adding the `IGet<T>` interface!

```cs
// Beginning of script omitted!
public abstract record State : StateLogic<State> {
  public record InMainMenu
    : State,
      IGet<Input.NewGameClick>,
      IGet<Input.QuitClick> {
    public Transition On(in Input.NewGameClick input) =>
      To<InGame>();

    public Transition On(in Input.QuitClick input) =>
      To<ClosingApplication>();
  }

  public record InGame : State { }

  public record ClosingApplication : State { }
}
```

The code should now be hopping over to the other states; `InGame` and `ClosingApplication`. However, we're not producing any outputs in the states.

## Producing and reacting to outputs

Let's fix that also - by adding the state constructor and specifying what happens `OnEnter`. Tiny boilerplate incoming!

```cs
public record InGame : State {
  public InGame() {
    this.OnEnter(() => Output(new Output.StartNewGame()));
  }
}

public record ClosingApplication : State {
  public ClosingApplication() {
    this.OnEnter(() => Output(new Output.QuitApp()));
  }
}
```

The final step now, is to actually **react** to the output from the machine via the `Binding`. We'll hop over to our `Ready` function and set up those reactions!

```cs
public override void _Ready() {
  Logic = new AppLogic();
  Binding = Logic.Bind();

  _newGameButton = GetNode<Button>("%NewGameButton");
  _quitButton = GetNode<Button>("%QuitButton");

  _newGameButton.Pressed += OnNewGameButtonPressed;
  _quitButton.Pressed += OnQuitButtonPressed;

  // NEW BOILERPLATE UNLOCKED
  Binding
    .Handle((in AppLogic.Output.StartNewGame output) =>
      GD.Print("TODO start game"))
    .Handle((in AppLogic.Output.QuitApp output) =>
      GetTree().Quit());
}
```

When running the game through the editor, we can now quit the game. This proves our state machine is working!

## More transitions and states

However, we can also get stuck in the `InGame` state, since the New Game button transitions us there. We have no way of getting out of there!

To showcase the ease-of-control that LogicBlocks gives us. The plan is:

- Allow for pressing Escape to return to the main menu.
-
