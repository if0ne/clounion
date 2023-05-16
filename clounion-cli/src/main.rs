use std::io::Write;
use storage_client::client::StorageClient;
use storage_client::config::Config;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const PREFIX: &str = "co-cli > ";

#[tokio::main]
async fn main() {
    let client = StorageClient::new(Config::from_file("Client.toml").await.unwrap());
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
                    "help" => {
                        println!("Список команд:");

                        println!("\tls <prefix> - список файлов пользователя; ARGS: <prefix> - префикс названия файла для поиска");
                        println!("\tul <-s/-l> <remote filename> <filename> - загрузка файла на сервер; ARGS: <-s> - маленький файл, <-l> - большой файл, <remote filename> - удаленное название файла, <filename> - название файла на локальной машине");
                        println!("\tdl <-s/-l> <remote filename> <filename> - загрузка файла на локальную машину; ARGS: <-s> - маленький файл, <-l> - большой файл, <remote filename> - удаленное название файла, <filename> - название файла на локальной машине");
                        println!("\tac <remote filename> <filename> - обновление файла маленького размера; ARGS: <remote filename> - удаленное название файла, <filename> - название файла на локальной машине");
                        println!("\tdelete <remote filename> - удаление файла; ARGS:<remote filename> - удаленное название файла");
                    }
                    "ls" => {
                        let prefix = if args.len() < 2 { "" } else { args[1] };

                        println!("Получаем список...");
                        let res = client.get_files(prefix).await;

                        match res {
                            Ok(files) => {
                                println!("Список файлов: ");
                                for file in files {
                                    println!("\tИмя файла: {} Тип: {:?}", file.0, file.1);
                                }
                            }
                            Err(err) => {
                                println!("Ошибка: {:?}", err)
                            }
                        }
                    }
                    "ul" => {
                        if args.len() < 4 {
                            println!("Введите тип файла, его ключ и имя файла. Пример: ul -s/-l <remote filename> <filename>");
                            continue;
                        }

                        let file = match tokio::fs::File::open(args[3]).await {
                            Ok(file) => file,
                            Err(err) => {
                                println!("{}", err);
                                continue;
                            }
                        };

                        println!("Загрузка...");

                        match args[1] {
                            "-s" => {
                                let res = client.create_small_file(args[2], file).await;

                                if let Err(err) = res {
                                    println!("Ошибка: {:?}", err)
                                } else {
                                    println!("Успешно загружено")
                                }
                            }
                            "-l" => {
                                let res = client.create_large_file(args[2], file).await;

                                if let Err(err) = res {
                                    println!("Ошибка: {:?}", err)
                                } else {
                                    println!("Успешно загружено")
                                }
                            }
                            _ => {
                                println!("Неизвестный тип файла");
                            }
                        }
                    }
                    "dl" => {
                        if args.len() < 4 {
                            println!("Введите тип файла, его ключ и имя файла. Пример: dl -s/-l <remote filename> <filename>");
                            continue;
                        }

                        let mut file = match tokio::fs::File::create(args[3]).await {
                            Ok(file) => file,
                            Err(err) => {
                                println!("{}", err);
                                continue;
                            }
                        };

                        println!("Скачивание...");

                        match args[1] {
                            "-s" => {
                                let res = client.read_small_file_last_version(args[2]).await;

                                match res {
                                    Ok(bytes) => {
                                        let _ = file.write_all(&bytes).await;
                                        println!("Успешно скачано");
                                    }
                                    Err(err) => {
                                        println!("Ошибка: {:?}", err)
                                    }
                                }
                            }
                            "-l" => {
                                let res = client.read_large_file(args[2]).await;

                                match res {
                                    Ok(bytes) => {
                                        let _ = file.write_all(&bytes).await;
                                        println!("Успешно скачано");
                                    }
                                    Err(err) => {
                                        println!("Ошибка: {:?}", err)
                                    }
                                }
                            }
                            _ => {
                                println!("Неизвестный объект");
                            }
                        }
                    }
                    "ac" => {
                        if args.len() < 3 {
                            println!(
                                "Введите ключ и новое имя файла. Пример: ac <remote filename> <filename>"
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

                        println!("Обновление...");

                        let res = client.add_new_commit_to_small_file(args[1], &buffer).await;

                        if let Err(err) = res {
                            println!("Ошибка: {:?}", err)
                        } else {
                            println!("Успешно обновлено")
                        }
                    }
                    "delete" => {
                        if args.len() < 2 {
                            println!("Введите имя файла");
                            continue;
                        }

                        println!("Удаление...");
                        let res = client.delete_file(args[1]).await;

                        if let Err(err) = res {
                            println!("Ошибка: {:?}", err)
                        } else {
                            println!("Успешно удалено")
                        }
                    }
                    "q" => {
                        break;
                    }
                    _ => {
                        println!("Неизвестная команда");
                    }
                }
            }
        }
    }
}
