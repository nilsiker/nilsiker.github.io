+++
title = "Using Rust in Minimal Container Images"
date = 2024-03-23
description = "Containerizing a Rust application in a minimal container image."

[taxonomies]
tags = ["rust", "podman", "containers", "containerization"]

[extra]
footnote_backlinks = true
quick_navigation_buttons = true
+++

> ⚠️ This post wouldn't exist without Björn Molins [rust-minimal-docker](https://github.com/bjornmolin/rust-minimal-docker) repository.

# Introduction

In my professional work, I've developed a minimal Rust runner job to perform miscellaneous clean-up jobs based on messages from an event bus, all hosted in a cloud environment. 

Thanks to the performance Rust provides and the virtually weightless `scratch` image, cloud compute and storage costs are so low they're negligible.

Below I'll outline a quick and concise walkthrough of just that. Let's get started!

> 🙋🏼 This is a bit of an outlier - I won't be shoehorning any gamedev into this one. This is a quick post about a minimal Rust containerization example!

# Scope

For this small project I'll be using [Podman](https://podman.io/) to setup a containerized web application running on a `scratch` container image. It is a great open-source alternative to Docker.

For the application I will be using the web framework [Rocket](https://rocket.rs). It will include an index route, and an API route containing a singular endpoint for us to try out.

# The Rocket application

Let's quickly outline how we code and configure the Rocket web application. This project uses Rocket version 0.5.

## Code

Rocket provides a concise API to get started with web applications. The entire code is contained in `main.rs`.

```rust
use rocket::*;

// THIS IS OUR APP ENTRYPOINT
#[launch] 
fn rocket() -> _ {  
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![hello])
}

#[get("/")]
pub fn index() -> String {
    format!(
        "This is a dummy landing page for our Rocket web server.\n\nTry /api/hello?name=YOUR_NAME\n"
    )
}

#[get("/hello?<name>&<occupation>")]
pub fn hello(name: &str, occupation: Option<&str>) -> String {
    match occupation {
        Some(occupation) => format!("Hello {name}, you are a {occupation}.\n"),
        None => format!("Hello {name}! You can also provide your occupation as a query parameter.\n"),
    }
}
```

It's a small application that returns a string message when accessing the base URL. It also provides a `hello` endpoint in its API route, that responds based on the name and optional occupation provided as query parameters.

> 🙋🏼 I have yet to experiment seriously with Rocket, but it's remarkable how little code you actually need to get started. It reminds me of Express.js -- except I get to write Rust, making me a happy developer!

## Configuration

In Rocket, we can create a file called `Rocket.toml` to configure our application. In our case we want to bind the application to the `0.0.0.0`, so that we can curl our way into the container. The TOML looks like this:

```toml
[global]
address = "0.0.0.0"
```

Of course, there is more configuration available to us. The official guide has a good section about [Configuration](https://rocket.rs/guide/master/configuration/).

That's all regarding the application. Let's look at creating the container image.

# Containerizing the application

Running the application locally with `cargo run` tells me the code is sound, but we want to run this in a container, ready to be deployed to any suitable platform or host.

Let's quickly set up a configuration for all this.

## Setting up our Containerfile

First off, I create a `Containerfile` in my project root. The Containerfile specifies the following:

```Dockerfile
FROM clux/muslrust:stable as builder
ENV TARGET="x86_64-unknown-linux-musl"

WORKDIR /staging

COPY src ./src
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --target $TARGET --release

FROM scratch
ENV TARGET="x86_64-unknown-linux-musl"
ENV BINARY_NAME="rust-minimal-podman"

COPY Rocket.toml /Rocket.toml
COPY --from=builder /staging/target/$TARGET/release/$BINARY_NAME /app

ENTRYPOINT ["/app"]
```

No bells and whistles, in short we:

* Use the `clux/muslrust` image as our *builder*
* Copy over necessary project files
* Build the project in release mode, targetting `x86_64-unknown-linux-musl`
* Use the `scratch` image as our final image, our *runner*!
* Copy the compiled executable and the Rocket configuration file to the *runner*
* Specify our app as the image entrypoint

That should be enough to get us running Rust in a barebones image. Next, we leverage Podman to build and run our container image!

## Building the image

If you are familiar with Docker, you'll feel right at home. To build the image, we simply run:

```bash 
podman build -t rust-minimal-podman .
```

The `clux/muslrust` image includes the necessary toolchain and libraries to compile for our target image `scratch`.

## Running the image

Running the image is a breeze also. We run the image in a container using:

```bash
podman run -d -p 8000:8000 -t rust-minimal-podman
```

We use the detach flag `-d` to run the container in the background. 

Using the publish flag `-p` we export the port 8080 inside the container to port 8000 outside the container.

This will allow us to curl the application running in the container, using the port 8000.

## Testing some curls

We're *probably* all set now. To be sure, we try out our routes and API and see what we get.

```bash
$ curl "0.0.0.0/8000"
> This is a dummy landing page for our Rocket web server.
> 
> Try /api/hello?name=YOUR_NAME


$ curl "0.0.0.0:8000/api/hello?name=Nilsiker"
> Hello Nilsiker! You can also provide your occupation as a query parameter.


$ curl "0.0.0.0:8000/api/hello?name=Nilsiker&occupation=Rust%20propagandist"
> Hello Nilsiker, you are a Rust propagandist.
```

It works flawlessly!

Once content with the responses, we can kill the container with `podman kill <ID>`.

# Result

With the sanity check above performed, we can confirm that the `scratch` image is running our application without problems. Mission accomplished!

For the grand reveal, we check our image size with `podman images` (some headings are omitted from the output):

```bash
$ podman images localhost
REPOSITORY                     TAG         SIZE
localhost/rust-minimal-podman  latest      8.98 MB
```

Admittedly, the application is a very minimal example of a Rocket web application, but a basic web server application containing everything it needs to run as a standalone binary clocking in under 10MB? That is impressive!

> 🙋🏼 If you want to go all crazy about it, you can add the option `strip = true` to your Cargo.toml release profile. At the time of writing this, this brought my image size down to a miniscule 4.35 MB!

# Conclusion

Basing your container images on `scratch` can be useful for hyperoptimizing disk space used - an image size under 10MB is quite the feat in my book!

In the case of Rust, where we can statically link our application into a standalone binary, we can successfully and also quite effortlessly provide a minimal environment for our application to run using `scratch`.

For smaller use cases, we might also enjoy the complete control over dependencies used. Since we are fully aware of the binaries and libraries used on the image, you could also make a case for a more conscious and clear picture of what security concerns our app and environment is susceptible to.

I see a great use case for `scratch`-based images for small runner jobs or simple microservices where the use case isn't very complex - a great argument to use the least bloated image available.
 
Of course, the image does not come without its limitations. Imagine the observability issue; what if the container fails to run, and we want insights into what might be causing the problem. Since our standalone binary is possibly the only binary present in the container, how would we go about SSHing into the container?

Stripping out everything leaves us with not only full control but also full responsibility for what we put in our image.

At some point, it is more reasonable to use an Alpine Linux image. They're not much bigger, and comes with a lot of handy tools that make your containerization endeavours easier. 

But if all you want is a tiny data footprint, you can't beat doing it `FROM scratch`.

*Thanks for reading,<br/>Nilsiker*