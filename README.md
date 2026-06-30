# Rust API

**API REST minimalista em Rust com Axum.**

Estrutura base para construção de APIs com arquitetura em camadas (controllers → services), tratamento de erros padronizado e suporte a Docker.

## Features

- Roteamento com [Axum](https://github.com/tokio-rs/axum)
- Runtime assíncrono com Tokio
- Respostas JSON com Serde
- Tratamento de erros centralizado via `AppError`
- Estado compartilhado via `AppState`
- Pronto para Docker (multi-stage build)

## Quick Start

```bash
cargo run
```

API disponível em `http://localhost:3000`.

## Com Docker

```bash
docker compose up --build
```

## Requirements

- Rust 1.82+
- Cargo
- Docker (opcional)

## Tech Stack

| Camada | Tecnologia |
|--------|------------|
| Framework | Axum 0.7 |
| Runtime | Tokio 1.x |
| Serialização | Serde 1.x |
| Container | Docker + Compose |

## Estrutura

```
src/
├── main.rs
├── config/
│   ├── mod.rs
│   └── routes.rs          # definição de rotas
└── app/
    ├── mod.rs
    ├── state.rs            # AppState compartilhado
    ├── errors.rs           # AppError → resposta HTTP
    ├── controllers/
    │   └── index/          # handlers HTTP
    └── services/
        └── index/          # lógica de negócio
```

## Endpoints

| Método | Rota | Descrição |
|--------|------|-----------|
| GET | `/` | Retorna mensagem de status |

**Resposta:**

```json
{
  "status": "success",
  "message": "Hello from Rust API!"
}
```

## Build de produção

```bash
cargo build --release
./target/release/RustAPI
```