# Endpoint implementation of Technical Specifications for PCF Data Exchange (Version 2.2.0)

A yet incomplete beta-quality implementation of the HTTP REST API of the [Technical Specifications for PCF Data Exchange](https://wbcsd.github.io/tr/2023/data-exchange-protocol-20230221/)

## Status

⚠️⚠️⚠️⚠️⚠️
**This is **not** a "reference implementation" but a demonstrator used to generate OpenAPI spec files for documenting the Spec's REST API. A thorough review WRT specification compliance is still pending.**

**This means, you should not yet rely on this implementation for conducting conformance testing, yet.**
⚠️⚠️⚠️⚠️⚠️

For details on the backlog, please see [BACKLOG.md](BACKLOG.md).

# Endpoints

The following endpoints are available:

- Endpoints from Use Case 001 Specification Version 1
  - `/2/footprints` implementing the `ListFootprints` action
  - `/2/footprints/<footprint-id>` implementing the `GetFootprint` action
  - `/2/events` implementing the `Events` action
  - `/2/auth/token` implementing `Authenticate` action
- Additional endpoints are:
  - `/2/.well-known/openid-configuration`: OpenId provider configuration document
  - `/2/jwks`: the JSON Web Key Set used to encode and sign the authentication token
  - `/openapi.json`: OpenAPI description file which is automatically generated from the types defined in [`api_types.rs`](src/api_types.rs) and endpoints defined in [`main.rs`](src/main.rs)
  - Swagger UI: `/swagger-ui/` if you fancy a visualization

No further endpoints are supported by this implementation and all return `{"message":"Access Denied", "code":"AccessDenied"}`.

## Credentials

Currently, credentials are hardcoded to:
- client-id: `hello`
- client-secret: `pathfinder`

# Build instructions

## Build requirements

You need a working and up-to-date Rust toolchain installed. See [https://rustup.rs/](https://rustup.rs/) for details.

After having installed `rustup`, ensure you have the `stable toolchain` installed like this:

```sh
rustup update
rustup toolchain install stable
```

## Building

Once Rust is installed via rustup, just perform

```sh
cargo build
```

## Running locally

You first need to create a private key:

```sh
scripts/keygen.sh
```

Which will create the file `keypair.pem` for you.

Then, you can run the server like this:

```sh
PRIV_KEY=`cat keypair.pem` cargo run
```

To run it at a different port, e.g. 3333:

```sh
ROCKET_PORT=3333 PRIV_KEY=`cat keypair.pem` cargo run
```

## Running the server in a "Production" mode

```sh
## building
cargo build --release

## running
export ROCKET_SECRET_KEY=$(openssl rand -base64 32)
export PRIV_KEY="..."
cargo run --release
```

The resulting binary can be found in `target/release/bootstrap`
