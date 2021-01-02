# Duckception

An implementation of the Mafia/Werewolf party game to be played online. Development commands will be located in the top level ```package.json``` file and in root level scripts.

## Development

you will require the following dependencies
- Rust + Cargo
- Node + npm

```bash 
npm run server # Starts the rust backend, uses cargo to fetch and compile directories
npm run web # Start parcel in watch mode, bundling into the dist directory
npm run build_web # parcel builds production version of the web assets
npm run build_server # build production version of sever
npm run serve # Have parcel serve instead of the server ex. for testing css quickly
```

## Build

run ```build_full.sh``` located in the root directory for a full build


