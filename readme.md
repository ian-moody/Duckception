# Duckception

An implementation of the Mafia/Werewolf party game to be played online. Development commands will be located in the top level ```package.json``` file and in root level scripts.

## Development

you will require the following dependencies
- Rust + Cargo
- Node + npm

```bash 
cargo run # Starts the rust backend, uses cargo to fetch and compile directories
npm start # Start parcel in watch mode, bundling into the dist directory
./build_full # Builds parcel production version of the web assets & production version of server in docker container
```

## Build

run ```build_full.sh``` located in the root directory for a full build

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