# AuthV
🔍 Authenticity files validator

## About
AuthV scans and generate a hash table of all the files contained in the folder.
The goal is to prevent files manipulations by thirdparty while sharing content.

## Build debug
```BASH
cd authv
cargo build
```

## Build release
```BASH
cd authv
cargo build --release
```

## Run
```BASH
./target/debug/authv <directory>
```

## Todo
- Check the files with the provided config file
- Generate a signature file
