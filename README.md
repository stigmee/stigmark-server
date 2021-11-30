### Stigmark

Stigmark is a kind of shared bookmark manager. The idea is to start collect data for Stigmee AI.

### Client

It comes as a Webbrowser extension (for now Chrome, Brave, Edge).

### Server

The server is written in Rust. It relies on [Rocket](https://rocket.rs) which in turn, needs to use a nightly version of Rust.

This is temporary. We'll move later to bare simple - if it means something in Rust - [Hyper](https://hyper.rs/) based framework and [Tokio](https://tokio.rs/).

### How to compile :

1. Install Rust compiler with [rustup](https://rustup.rs/)
2. Install nightly compiler (required by Rocket library). In command line (works with bash/cmd/powershell) type the following instructions :

```bash
rustup default nightly
```

3. Fetch the source code :

```bash
git clone https://github.com/stigmee/stigmark-rocket-rs
```

4. Enter the directory and compile with [cargo](https://doc.rust-lang.org/cargo/) (the Rust package manager) :

```bash
cd stigmark-rocket-rs
cargo run
```

### How to use it ?

![step 1](/docs/img/snip-1.png)
![step 2](/docs/img/snip-2.png)
![step 3](/docs/img/snip-3.png)
![step 4](/docs/img/snip-4.png)
![step 5](/docs/img/snip-5.png)

### test

In test directory, you'll find rust tests lib+app

### Bugs 

Yes

### TODO

A lot.

- make it fully RESTful
- Rocket -> Hyper/Tokio

### Notes

- Not RESTful yet
