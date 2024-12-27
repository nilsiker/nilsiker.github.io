+++
title = "Exploring Chickensoft - Getting Started"
date = 2024-12-27
description = "Getting started with Chickensoft architecture in Godot C#."

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

**However, taking it all in at once can show quite daunting.**

In this post, I hope to introduce the Chickensoft concepts, progressively adding them to a Chickensoft game template project as we run into good use cases.

> 🙋🏼 I'm not providing any golden paths or silver bullets here. I will be using the Chickensoft packages **as I understand them**.

We'll start things off really simple - let's cook some chicken! 🐔

# Scope

At the end of this post, we will have finished the following steps:

* Install a Godot C# version with the [GodotEnv CLI](https://github.com/chickensoft-games/GodotEnv)
* Create a project using the [GodotGame template](https://github.com/chickensoft-games/GodotGame)
* Setup an App node and script, to handle the application lifecycle of our project.

We'll make stops along the way and rationalize not only how we perform these steps, but *why* we do it.

We'll start off with getting GodotEnv on our machine. As of writing this post, GodotEnv requires `net8.0`.

> 🙋🏼 Specifically, I'll be using .NET 8.0.404. Consider this foreshadowing!

An argument for using GodotEnv is that it makes installing, uninstalling and switching between Godot versions a breeze. It also provides a way to manage addons.

# Installing GodotEnv and Godot

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

Reading the comments in the `Main.cs`, we realize that the main scene is used to conditionally run tests. Let's skip the specifics for now, as we won't dive into the testing capabilities of Chickensoft at this point.

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

# Handling application stuff

Let's get back to the Godot side of things and start adding our own stuff!

The main node of my Godot projects is the `App`. The idea is to have  `App` handle all application concerns.

This will be simple stuff, such as "starting the game" or "closing the application". We're preparing for the game that does not yet exist.

> 🙋🏼 Prior to adopting the Chickensoft ways, I used to call the App node `Main` instead. This has now changed!

I'll create an App scene and script and place them in the `src/app` folder. Let's also organize our `Game` source files into their own folder.

![alt text](img/project_structure_1.png)

In the App, let's throw in an idea for a node hierarchy. This is a rough sketch of what I like to do:

![A scene hierarchy, the scene root being the a Node, containing a CanvasLayer which in turn contains a VBox. The VBox has 4 children; a label and 3 buttons.](./img/node_hierarchy1.png)

Next, I give them descriptive names and center the VBox in the CanvasLayer.

![Renamed nodes in scene hierarchy, a centered main menu in the editor viewport containing the title "Eggsploration", and the buttons New Game, Load Game and Quit.](./img/node_hierarchy2.png)

Let's pop into `Main.cs` real quick and have `Main.RunScene` change the scene to our app:

```cs
  private void RunScene() 
    => GetTree().ChangeSceneToFile("res://src/app/App.tscn");
```

Now we end up at our main menu when running the game!

Of course, the buttons do nothing at this point. We should give our App some behaviour by editing `App.cs`.

Before we do that, let's introduce the first potentially daunting parts of Chickensoft - state machines and dependency injection.

## Adding state machines (LogicBlocks)

I've [previously written posts on state machines](/blog/finite-state-machine-1), so I won't go into specifics here.

However, what I will do is shout from the rooftops that [LogicBlocks](https://github.com/chickensoft-games/LogicBlocks) are really cool!

> 🙋🏼 They're all I dreamed of accomplishing with my FSM series. I might consider myself defeated - but frankly it's more that I've been ✨ blessed ✨.

With LogicBlocks, we can easily create complex logic for our game nodes by separating it into different states. I remember promising simple beginnings, so we **will** be keeping it simple!

This means that the LogicBlock usage might seem overkill, but I'll try my best at explaining why we should bother at all!



## Adding dependency injection