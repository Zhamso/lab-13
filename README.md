**Лабораторная работа №12**

> Implement web-server with Axum (https://github.com/tokio-rs/axum)
>
> GET /v1/todos/63443a02-2137-48e8-8db5-79fa54e8bfdf

Для запуска ЛБ №12 - из корня репозитория выполните следующие команды

```bash
$ cd backend/
$ cargo run
```

Запустится приложение прослушивающее адрес `127.0.0.1:3000`

```bash
> Finished dev [unoptimized + debuginfo] target(s) in 0.49s
>      Running `target/debug/axum_tokio_app`
> 2022-10-29T05:34:15.097558Z  INFO axum_tokio_app: listening on 127.0.0.1:3000
```

Теперь остается отправить GET-запрос по адресу `http://127.0.0.1:3000/v1/todos/{any_message}`, и вы получите в ответ json-файл в котором, в поле `id` лежит Ваше сообщение.

*Пример*

```bash
$ curl -X GET http://127.0.0.1:3000/v1/todos/sadasd
> {"assigned":"user@example.com","id":"sadasd","message":"Just do it!","priority":"A"}
```

#13
Nginx as proxy to Todos-server (Rust, Axum) -- local setup (with out docker)

#14
13 lab inside containers with Docker Compose
