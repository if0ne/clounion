use std::io::Write;
use storage_client::client::StorageClient;
use storage_client::config::Config;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

const PREFIX: &str = "co-cli > ";

#[tokio::main]
async fn main() {
    let client = StorageClient::new(Config::from_file("Client.toml").await.unwrap());
    let uuid = Uuid::new_v4();
    let mut input = String::new();

    loop {
        print!("{PREFIX}");
        let _ = std::io::stdout().flush();
        input.clear();
        let _ = std::io::stdin().read_line(&mut input);

        match input.trim() {
            "" => {}
            _ => {
                let args = input
                    .split(' ')
                    .map(|arg| arg.trim())
                    .collect::<Vec<&str>>();

                match args[0] {
                    "ls" => {
                        let prefix = if args.len() < 2 { "" } else { args[1] };

                        println!("Getting list of files...");
                        let res = client.get_files(prefix).await;

                        match res {
                            Ok(files) => {
                                println!("List: ");
                                for file in files {
                                    println!("Filename: {} Type: {:?}", file.0, file.1);
                                }
                            }
                            Err(err) => {
                                println!("Error: {:?}", err)
                            }
                        }
                    }
                    "ul" => {
                        if args.len() < 4 {
                            println!("Enter type of object, key and filename. Example: ul -s/-l <remote filename> <filename>");
                            continue;
                        }

                        let file = match tokio::fs::File::open(args[3]).await {
                            Ok(file) => file,
                            Err(err) => {
                                println!("{}", err);
                                continue;
                            }
                        };

                        println!("Uploading...");

                        match args[1] {
                            "-s" => {
                                let res = client.create_small_file(uuid, args[2], file).await;

                                if let Err(err) = res {
                                    println!("Error: {:?}", err)
                                } else {
                                    println!("Successful created")
                                }
                            }
                            "-l" => {
                                let res = client.create_large_file(uuid, args[2], file).await;

                                if let Err(err) = res {
                                    println!("Error: {:?}", err)
                                } else {
                                    println!("Successful created")
                                }
                            }
                            _ => {
                                println!("Unknown object type");
                            }
                        }
                    }
                    "dl" => {
                        if args.len() < 4 {
                            println!("Enter type of object, key and filename. Example: dl -s/-l <remote filename> <filename>");
                            continue;
                        }

                        let mut file = match tokio::fs::File::create(args[3]).await {
                            Ok(file) => file,
                            Err(err) => {
                                println!("{}", err);
                                continue;
                            }
                        };

                        println!("Downloading...");

                        match args[1] {
                            "-s" => {
                                let res = client.read_small_file_last_version(uuid, args[2]).await;

                                match res {
                                    Ok(bytes) => {
                                        let _ = file.write_all(&bytes).await;
                                        println!("Successful download");
                                    }
                                    Err(err) => {
                                        println!("Error: {:?}", err)
                                    }
                                }
                            }
                            "-l" => {
                                let res = client.read_large_file(uuid, args[2]).await;

                                match res {
                                    Ok(bytes) => {
                                        let _ = file.write_all(&bytes).await;
                                        println!("Successful download");
                                    }
                                    Err(err) => {
                                        println!("Error: {:?}", err)
                                    }
                                }
                            }
                            _ => {
                                println!("Unknown object type");
                            }
                        }
                    }
                    "ac" => {
                        if args.len() < 3 {
                            println!(
                                "Enter key and filename. Example: ac <remote filename> <filename>"
                            );
                            continue;
                        }

                        let mut file = match tokio::fs::File::open(args[2]).await {
                            Ok(file) => file,
                            Err(err) => {
                                println!("{}", err);
                                continue;
                            }
                        };

                        let mut buffer = vec![];
                        let _ = file.read_to_end(&mut buffer).await;

                        println!("Updating...");

                        let res = client
                            .add_new_commit_to_small_file(uuid, args[1], &buffer)
                            .await;

                        if let Err(err) = res {
                            println!("Error: {:?}", err)
                        } else {
                            println!("Successful updated")
                        }
                    }
                    "delete" => {
                        if args.len() < 2 {
                            println!("Enter filename");
                            continue;
                        }

                        println!("Deleting...");
                        let res = client.delete_file(uuid, args[1]).await;

                        if let Err(err) = res {
                            println!("Error: {:?}", err)
                        } else {
                            println!("Successful deleted")
                        }
                    }
                    "q" => {
                        break;
                    }
                    _ => {
                        println!("Unknown command");
                    }
                }
            }
        }
    }
}
