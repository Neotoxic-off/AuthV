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

## Example
```BASH
$> ./target/debug/authv ../test/
[2024-10-08T13:37:13Z INFO  authv] Directory set to: ../test/
[2024-10-08T13:37:13Z INFO  authv] Element in directory: 4
[2024-10-08T13:37:13Z WARN  authv] Hashing the files
[2024-10-08T13:37:13Z INFO  authv] Saved hash table at: table.avht

$> ./target/debug/authv ../test/
[2024-10-08T13:38:06Z INFO  authv] Directory set to: ../test/
[2024-10-08T13:38:06Z INFO  authv] Element in directory: 4
[2024-10-08T13:38:06Z WARN  authv] Hashing the files
[2024-10-08T13:38:06Z INFO  authv] ../test/2/sub 1/item_2_2 is intact
[2024-10-08T13:38:06Z INFO  authv] ../test/2/item_1 is intact
[2024-10-08T13:38:06Z INFO  authv] ../test/2/item_2 is intact
[2024-10-08T13:38:06Z INFO  authv] ../test/2/sub 1/item_1_1 is intact
```