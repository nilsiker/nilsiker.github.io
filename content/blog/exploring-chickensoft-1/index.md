+++
title = "Exploring Chickensoft - GodotEnv and GodotGame"
date = 2024-12-27
description = "Getting started with Chickensoft C# architecture in Godot."
draft = true

[taxonomies]
tags = ["Godot", "recipe", "C#", ".NET", "Chickensoft", "saving", "serialization"]

[extra]
footnote_backlinks = true
quick_navigation_buttons = true
+++

> 🙋🏼 A huge thanks goes out to Joanna May and all Chickensoft collaborators. Check out their [site](https://chickensoft.games/) and their truly impressive open-source tools for most things C# in Godot!

> ⚠️ This post assumes you have a basic understanding of Nodes and Scenes in Godot and at least mid-level familiarity with C# syntax if you plan to code along!

# Introduction

If you're a Godot C# developer, you might have stumbled over the Chickensoft tools. The folks over at the organization has done a wonderful job showcasing the architecture and packages in their [GameDemo](https://github.com/chickensoft-games/GameDemo).

In short, the tools and guidelines provided by Chickensoft should provide you with a clean, extendible and testable architecture for your games.

**However, taking it all in at once is quite daunting.**

In this post, I hope to introduce the Chickensoft concepts, progressively adding them to a Chickensoft game template project as we run into good use cases.

> 🙋🏼 I'm not providing any golden paths or silver bullets here. I will be using the Chickensoft packages **as I understand them**, essentially recreating my personal journey learning these tools and patterns!

We'll start things off really simple - let's get cookin'! 🐔

# Scope

At the end of this post, we will have finished the following steps:

* Install a Godot C# version with the [GodotEnv CLI](https://github.com/chickensoft-games/GodotEnv)
* Create a project using the [GodotGame template](https://github.com/chickensoft-games/GodotGame)
* Prepare a scene structure for our game - starting with the `App` node

We'll make stops along the way and rationalize not only how we perform these steps, but *why* we do it.

We won't be explaining or describing every technical detail of the Chickensoft packages.

The goal is to build a solid game architecture that we can easily expand on using Chickensoft tools.

# Installing GodotEnv and Godot

We'll start off with getting GodotEnv on our machine. As of writing this post, GodotEnv requires `net8.0`.

> 🙋🏼 Specifically, I'll be using .NET 8.0.404. Consider this foreshadowing!

An argument for using GodotEnv is that it makes installing, uninstalling and switching between Godot versions a breeze. It also provides a way to manage addons.

Installing the GodotEnv tool is a sweet one-liner:

```bash
dotnet tool install --global Chickensoft.GodotEnv
```

With the tool installed, another one-liner helps us install Godot 4.3.0 (C#):

```bash
godotenv godot install 4.3.0 # adding -n would install the non-C# version.
```

That's it! We now have Godot 4.3.0 installed. You can confirm this by running:

```bash
godot --version
```

# Create a GodotGame template project

Next, we'll install the GodotGame template, allowing us to quickly scaffold Godot projects using some boilerplate Chickensoft configuration.

Install the template like so:

```bash
dotnet new install Chickensoft.GodotGame
```

Then generate a new project using the template:

```bash
dotnet new chickengame --name "Eggsploration" --param:author "Andreas Nilsson"
```

I'm calling my project **Eggsploration**, so that we can finally get that inevitable pun out of the way. I'll set myself as the author.

We now have a Chickensoft game template project set up! Easy does it...

## Inspecting the default template

Let's cd into the project and edit it in Godot:

```bash
cd Eggsploration # substitute with your project name!
godot -e
```

## Run the game

When running the game with F5, we are presented with a scene containing a test button:

![alt text](img/test_button_scene.png)

Taking a closer look, we see this is actually the `Game.tscn` scene, not `Main.tscn` - the latter being completely empty.

Reading the comments in `Main.cs`, we realize that the main scene is used to conditionally run tests. Let's skip the specifics for now, as we won't dive into the testing capabilities of Chickensoft at this point.

> 🙋🏼 I'm not fully sold on TDD. I'm sorry, Joanna...

At the end, the `Main` script changes the scene to `Game`, containing the test button. We're on track!

## The project file

Having a gander at the good ol' `.csproj` file, we have some out-of-the-box configuration.

```xml
<Project Sdk="Godot.NET.Sdk/4.3.0">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
    <EnableDynamicLoading>true</EnableDynamicLoading>
    <LangVersion>latest</LangVersion>
    <Nullable>enable</Nullable>
    <RootNamespace>Eggsploration</RootNamespace>
    <!-- Required for some nuget packages to work -->
    <!-- godotengine/godot/issues/42271#issuecomment-751423827 -->
    <CopyLocalLockFileAssemblies>true</CopyLocalLockFileAssemblies>
    <!-- To show generated files -->
    <!-- <EmitCompilerGeneratedFiles>true</EmitCompilerGeneratedFiles> -->
    <!--
      <CompilerGeneratedFilesOutputPath>.generated</CompilerGeneratedFilesOutputPath>
    -->
    <DebugType>portable</DebugType>
    <DebugSymbols>true</DebugSymbols>
    <Title>Eggsploration</Title>
    <Version>1.0.0</Version>
    <Description>Eggsploration</Description>
    <Copyright>© 2024 Andreas Nilsson</Copyright>
    <Authors>Andreas Nilsson</Authors>
    <!-- Don't include unit tests in release builds. -->
    <DefaultItemExcludes Condition="'$(Configuration)' == 'ExportRelease'">
      $(DefaultItemExcludes);test/**/*
    </DefaultItemExcludes>
  </PropertyGroup>
  <ItemGroup Condition=" '$(Configuration)' == 'Debug' or '$(Configuration)' == 'ExportDebug' ">
    <!-- Test dependencies go here! -->
    <!-- Dependencies added here will not be included in release builds. -->
    <PackageReference Include="Chickensoft.GoDotTest" Version="1.5.10" />
    <!-- Used to drive test scenes when testing visual code -->
    <PackageReference Include="Chickensoft.GodotTestDriver" Version="3.0.2" />
    <!-- Bring your own assertion library for tests! -->
    <!-- We're using Shouldly for this example, but you can use anything. -->
    <PackageReference Include="Shouldly" Version="4.2.1" />
    <!-- LightMock is a mocking library that works without reflection. -->
    <PackageReference Include="LightMock.Generator" Version="1.2.2" />
    <!-- LightMoq is a Chickensoft package which makes it more like Moq. -->
    <PackageReference Include="LightMoq" Version="0.1.0" />
  </ItemGroup>
  <ItemGroup>
    <!-- Production dependencies go here! -->
  </ItemGroup>
</Project>

```

We're set up for project-wide nullables (I wouldn't have it any other way).

We also have a few debug dependencies set up for us for testing purposes. The comments are more than sufficient to give us an idea on their use.

Before long, we will add some packages to accompany that Production dependencies comment.

## Miscellaneous configuration and tooling

The template provides a lot of out-of-the-box config and tooling - too much to go over here and now!

There's **GitHub actions** for **continuous testing** and **spell checking**, **test coverage scripts** and **editor configuration** to enforce **coding style**. The list goes on!

We'll put blinders on for now and focus on the Godot architecture side of things!

# Preparing our scene structure

Let's get back to the Godot side of things and start adding our own stuff!

The main node of my Godot projects is the `App`. The idea is to have  `App` handle all application concerns.

This will be simple stuff, such as "starting the game" or "closing the application". We're preparing for the game that does not yet exist.

> 🙋🏼 Prior to adopting the Chickensoft ways, I used to call the App node `Main`. This has now changed!

I'll create an App scene and script and place them in the `src/app` folder. Let's also organize our `Game` source files into their own folder.

![The project hierarchy, a source folder containing an app folder and a game folder, with scripts and scenes organized accordingly.](img/project_structure_1.png)

In the App, let's throw in an idea for a node hierarchy. This is a rough sketch of what I like to do:

![A scene hierarchy, the scene root being the a Node, containing a CanvasLayer which in turn contains a VBox. The VBox has 4 children; a label and 2 buttons.](./img/node_hierarchy1.png)

Next, I give them descriptive names and center the VBox in the CanvasLayer.

![Renamed nodes in scene hierarchy, a centered main menu in the editor viewport containing the title "Eggsploration", and the buttons New Game and Quit.](./img/node_hierarchy2.png)

Let's pop into `Main.cs` real quick and have `Main.RunScene` change the scene to our app:

```cs
  private void RunScene() 
    => GetTree().ChangeSceneToFile("res://src/app/App.tscn");
```

Now we end up at our main menu when running the game!

Of course, the buttons do nothing at this point. We should give our App some behaviour by editing `App.cs`.

Before we do that, let's introduce the first potentially daunting part of Chickensoft - state machines.

## State machines using LogicBlocks

I will start of by shouting from the rooftops ***that [LogicBlocks](https://github.com/chickensoft-games/LogicBlocks) is a really cool package!***

> 🙋🏼 They're all I dreamed of accomplishing with my FSM series. I could consider myself defeated - but frankly it's more so being ✨ blessed ✨.

With LogicBlocks, we can create complex logic for our game nodes by separating it into different states. Part of the value proposition is that we won't create a tangled mess while doing so.

I remember promising simple beginnings, so we **will** be keeping it simple! This means that the LogicBlock usage might seem overkill, but I'll try my best at explaining why we should bother at all!

### Installing LogicBlocks

The packages need to be added to our project. The LogicBlock docs recommends including the introspection and diagram generator for added convenience and to get automatically generated diagrams for your state machines (we'll take a look at that later).

To include the packages in the right spot in our project file, I'll use good old-fashioned copypasting:

```xml
 <ItemGroup>
    <!-- Production dependencies go here! -->
    <PackageReference Include="Chickensoft.LogicBlocks" Version="5.14.0" />
    <PackageReference Include="Chickensoft.Introspection.Generator" Version="2.1.0" />
    <PackageReference Include="Chickensoft.LogicBlocks.DiagramGenerator" Version="5.14.0" />
  </ItemGroup>
```

I'll check my project sanity by hitting F5 in the editor - my game still runs.

### Implementing App states

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
[LogicBlock(typeof(AppLogic), Diagram = true)]
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

A definition lightning round might go:

* `Logic`:  This is a reference to the LogicBlocks state machine. This is mainly used to send inputs into the machine.
* `Binding`: This is a reference to the state binding. It is primarily used to listen to outputs produced by the machine.
* `[Meta]`: Blinders on for this one! To quote the creators of Chickensoft:
    > You don't need to fully understand this package to make the most of it. In fact, you may never need to use it directly since you are more likely to encounter it as a dependency of one of the other Chickensoft tools.
* `[LogicBlock(typeof(AppLogic), Diagram = true)]`: This attribute extends the class, enabling LogicBlocks generators to generate serialization utilities for the our machine. Setting `Diagram = true` includes the class in the diagram generation.
* `AppLogic`: The LogicBlocks machine that will contain our game logic in various states, taking in input and producing output based on rules we decide.
  * `Input`: A class containing record structs that are used to send pieces of data as input into the machine.
  * `Output`: A class containing record structs that are used to produce outputs from the machine, later to be consumed and reacted to in the `App` node via the `Binding`.
  * `State`: An abstract state class for all our App LogicBlock states.
    * `InMainMenu`: The state for when the app is in the main menu.
    * `InGame`: The state for when the app is running the game.

> 🙋🏼 Later on when using AutoInject, we have access to a separate set of lifecycle methods that we should prefer!
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

Next, let's actually send some input into the machine. We'll connect the `Button.Pressed` signals to handlers through code. Let's leverage Godot unique names to get a hold of the references!

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
public abstract partial record State : StateLogic<State> {
  public partial record InMainMenu 
    : State,
      IGet<Input.NewGameClick>, 
      IGet<Input.QuitClick> {
    public Transition On(in Input.NewGameClick input) => 
      To<InGame>();

    public Transition On(in Input.QuitClick input) =>
      To<ClosingApplication>();
  }

  public partial record InGame : State { }
  
  public partial record ClosingApplication : State { }
}
```

The code should now be hopping over to the other states; `InGame` and `ClosingApplication`. However, we're not producing any outputs in the states. 

Let's fix that also - by adding the state constructor and specifying what happens `OnEnter`. Tiny boilerplate incoming!

```cs
public partial record InGame : State {
  public InGame() {
    this.OnEnter(() => Output(new Output.StartNewGame()));
  }
}

public partial record ClosingApplication : State {
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
    .Handle((in AppLogic.Output.StartNewGame output) => GD.Print("TODO start game"))
    .Handle((in AppLogic.Output.QuitApp output) => GetTree().Quit());
}
```
