# **Completed**

## 12

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

## 13
В `/target/todos.conf` находится правило, которое говорит nginx прослушивать все запросы по порту `80` и перенаправлять их на порт `3000` (3000 прослушивается этим приложением)  
```conf
server {
    listen        80;
    server_name _;
    location / {
        proxy_pass         http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header   Upgrade $http_upgrade;
        proxy_set_header   Connection keep-alive;
        proxy_set_header   Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header   X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto $scheme;
    }
}
```

## 14
Собрать контейнер
```bash
docker-compose build
```

Запустить контейнер
```bash
docker-compose up
```

Проверка
```bash
curl -X GET http://0.0.0.0/v1/todos/sadasd
```