# Duckception

An implementation of the Mafia/Werewolf party game to be played online. 

## Development
you will require the following dependencies
- Rust + Cargo
- Node + npm

```bash 
 # Starts the rust backend, uses cargo to fetch and compile directories
 cargo run
 # Start parcel in watch mode, bundling into the dist directory
 npm start
 # Builds parcel production version of the web assets & production version of server in docker container
 ./build_full
```

Development commands will be located in the top level ```package.json``` file and in root level scripts.

## Logging
Example of info logging, prints to stdout & stderr
using https://crates.io/crates/env_logger for logging
```bash 
RUST_LOG=trace cargo run 
```

## Heroku deployment
https://dashboard.heroku.com/apps/duckception
```bash
heroku container:login
heroku container:push web
heroku container:release web
```

## Learning and development takeaways

### Back end

- Learning Rust through developing a web server
  - borrow checker
  - memory management
  - meta-programming 

- Multithreaded programming 
  - Green Thread (Tokio runtime)
  - Mutexes
  - Atomic read/writes
  - Rust Futures + Async Await

- websocket / stream programming

- Deeper understanding of web protocols
  - HTTP headers
  - file caching 
  - File compressions
  - Cookies
  - TLS

- Session based authentication

### Front end

- "Modern" web app with Vanilla Javascript
- CSS animations
- A bit of SCSS
- Bundling Applications
  - tree shaking
  - javascript compilers / transpilers / minifiers