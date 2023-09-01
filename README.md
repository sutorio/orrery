# Orrery <small>(Rust)</small>

## TODO (keep adding to this)

- [ ] API Routes + handlers
  - [ ] Database pool creation/access
  - [ ] Correct HTTP response for db constraint errors
  - [ ] Correct HTTP response for borked/otherwise db
  - [ ] Merged API routes
  - [ ] Celestial bodies
    - [x] table creation
    - [ ] de/serialisable structs
    - [ ] RESTful routes
    - [ ] handlers with inlined db queries
  - [x] Celestial regions
    - [x] table creation
    - [x] de/serialisable structs
    - [x] RESTful routes
    - [x] handlers with inlined db queries
      - [ ] allow put/patch update handlers to return updated data
  - [ ] Celestial subregions
    - [x] table creation
    - [ ] de/serialisable structs
    - [ ] RESTful routes
    - [ ] handlers with inlined db queries

## Overview

My daughter Joanie's interest in planets led to the creation of a concise,
visually appealing solar system information app.

### Data

Solar system objects of diverse types orbit the sun, some orbit other objects
(requiring parent/child links). Most objects I focus on are region-associated,
with subregions. Clouds/fields contain abundant small objects, e.g., the
Asteroid Belt, the Kuiper Belt, and planetary ring systems. Some objects, like
comets, lack region ties. The sun is a distinct case, a visual axle for the
orrery's spin.

While scientific accuracy is essential for the data, visual limitations must
also be considered. Directly depicting the solar system at true scale is
unfeasible due to immense distances. Even a log scale becomes visually cluttered
with numerous objects. To address this, a formula like sqrt(AU•n)•x can be
employed, with adjustable values of n and x for aesthetic appeal. However,
planet radii present a challenge, as uniform scaling leads to disproportionate
positioning, especially for larger planets in close proximity.

Initially, I tried to hand-write this structure with the aim of creating a seed
file. This proved infeasible, and an immediate data model concern emerged:
integrating an input interface.

This mandates a separate admin part with authentication/authorization for the
app, serving as a testbed for strategies.

Expect data model inaccuracies; this is a proof of concept.

## Goals

- Produce a simple application using Axum/SQLX in preparation for the two more
  complex applications to come (_cf_ Docket & Dead Plastic).
- Assess how feasible is it to produce an appliction in Rust that compiles to a
  single binary, with the database & frontend embeddded in said binary.
- Map out development & deployment pipeline for said single-binary application.
- Properly investigate set up and usage of observability - tracing etc.
- Investigate testing strategies for Rust web apps.
- Compare model of Rust web application Vs. Elixir web application.
- Get thinking straight regarding assesmtn of power efficiency/usage Vs.
  Elixir/JS Frameworks/etc (backend & frontend).

## Development

### Prerequisites

- `asdf` installed and on your $PATH
- `rustup` installed and on your $PATH

[asdf](https://asdf-vm.com/) is a universal version manager, and is used within
the project to manage non-Rust language-level dependencies (SQLite &c.).
Dependencies are specified in the [.tool-versions](./.tool-versions) file at the
root of the project.

> _Why is Deno listed in the .tool-versions file?_ Deno is used purely for
> development, and may be removed after review. The built-in formatter works for
> JS/TS/HTML/Markdown/CSS files, and works well; this is the primary usecase. It
> also includes a tool called `deno_emit` which will run a basic compile on
> TS/JS code to allow it to be deployed to a browser environment.

Although asdf uses [rustup](https://rustup.rs/) under the hood to manage Rust
versions, it is more relaiable to use rustup directly. The
[rust-toolchain](./rust-toolchain.toml) file at the root of the project
specifies the channel used + the components required on install.

### Installation/setup

To install language dependencies, CLI tools, Cargo tools, and to create the
database if not already created, run:

```sh
make dev_prerequisites
```
