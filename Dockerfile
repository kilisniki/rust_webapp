# Этап сборки
FROM rust:1.81.0 AS builder

WORKDIR /usr/src/app

# Создаем фиктивный main.rs для кеширования зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Копируем файлы Cargo.toml и Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Кэшируем зависимости
RUN cargo build --release
RUN rm -rf target/release/deps/*

# Копируем исходный код
COPY src ./src

# Сборка приложения
RUN cargo build --release

# Этап выполнения
FROM debian:bookworm-slim

# Устанавливаем необходимые библиотеки
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Копируем скомпилированный бинарный файл из этапа сборки
COPY --from=builder /usr/src/app/target/release/rest_counter /usr/local/bin/rest_counter

# Устанавливаем переменную окружения для URL базы данных
ENV DATABASE_URL=postgres://postgres:password@db:5432/postgres

# Открываем порт
EXPOSE 8080

# Запускаем приложение
CMD ["rest_counter"]