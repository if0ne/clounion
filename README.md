# Clounion

Clounion is a distributed file storage system written in Rust. It keeps a version history for small files so clients can retrieve an earlier revision. It also supports large files by splitting them into blocks.

## Architecture

The main server keeps file metadata in Redis and tracks which data nodes hold each block. To upload or download a file, the client first asks the main server for block locations, then transfers the contents directly to or from the data nodes over gRPC. The CLI uses the `storage-client` library to handle both steps.

## Binaries

| Binary | Role |
| --- | --- |
| `main-server` | Registers data nodes, allocates storage, and looks up files and their versions. |
| `data-node` | Stores blocks on disk and serves read and write requests. |
| `clounion-cli` | Interactive client for listing, uploading, downloading, and deleting files. It can also add a revision to a small file or download a specific version. |
