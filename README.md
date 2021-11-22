### Stigmark

Stigmark is a kind of shared bookmark manager. The idea is to start collect data for Stigmee AI.

### Client

It comes as a Webbrowser extension (for now Chrome, Brave, Edge).

### Server

The server is written in Rust. It relies on [Rocket](https://rocket.rs) which in turn, needs to use a nightly version of Rust.

This is temporary. We'll move later to bare simple - if it means something in Rust - [Hyper](https://hyper.rs/) based framework and [Tokio](https://tokio.rs/).

### Bugs 

Yes

### TODO

A lot.

- make it fully RESTful
- Rocket -> Hyper/Tokio

### Notes

- Not RESTful yet
