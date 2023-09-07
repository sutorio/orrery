# Orrery <small>(Rust)</small>

## Overview

My daughter Joanie's interest in planets led to the creation of a concise,
visually appealing solar system information app.

## Development

This is based heavily on Jeremy Chone's `awesome-app`<sup> [1][1]</sup> work.
His initial video, _Rust Axum_<sup> [2][2]</sup> is one of the best
introductions to building an application using Axum that I've watched/read, and
he expanded on it in _Rust Axum Production Coding_<sup> [3][3]</sup>. I like the
structure he uses, it makes sense.

[1]: https://github.com/awesomeapp-dev
[2]: https://youtu.be/XZtlD_m59sM?si=NotUnKtun75eZNDt
[3]: https://youtu.be/3cA_mk4vdWY?si=c9dmgrHT0AHvo2bp

### Constraints

1. The application should be packaged as a single binary, trivially deployable.
2. The application binary should be as small as possible.
3. The application is expected to require persistence, and that should come in
   the form of an embedded database.
4. Ideally, the persistance should be in the form of a relational Db (so
   SQLite).
5. The application is not expected to have _many_ users. However
6. The application is expected to provide interactive _administrative_ functions
   personal to users. So:
7. The application should have users, secure authentication, and authorisation.
8. The application is expected to be a web application. However:
9. The application code should be modularised in such a way that the HTTP layer
   can be replaced with something else (_eg_ a desktop app framework, or a CLI).
   This implies:
10. The core application code should, as far as is feasible, avoid intermixing
    concerns (for example, using Tower idioms relating to Result types outside
    of actual Axum handler code).
11. The application modularisation should, as far as is sensible, ensure unit
    testing is as easy to carry out as possible.
12. The application should implement automated CI for building, testing and
    deploying at a production standard.
13. The application should act as a testbed for building out reusable CI jobs.
14. The application should implement logging/tracing.

### Prerequisites

- Rust, ideally installed directly via [rustup](ADD_LINK)
- The taskrunner/Make alternative [Just](https://just.systems). This has several
  nice features, has been refined over a few years to a tight set of features,
  and is simple. So the tradeoff for using it rather than Make seems reasonable.

### Setup

To install the Cargo tools & instantiate the database, run

```sh
just setup_dev
```

### Running the application

Then run the application in watch mode:

```sh
just watch_dev
```
