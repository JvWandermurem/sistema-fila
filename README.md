# Sistema Distribuído de Ingestão Massiva de Dados

[![Rust](https://img.shields.io/badge/rust-1.76+-orange.svg)](https://www.rust-lang.org/)  
[![RabbitMQ](https://img.shields.io/badge/rabbitmq-3.13+-red.svg)](https://www.rabbitmq.com/)  
[![PostgreSQL](https://img.shields.io/badge/postgresql-15+-blue.svg)](https://www.postgresql.org/)  
[![Docker](https://img.shields.io/badge/docker-orchestrated-blue.svg)](https://www.docker.com/)

---

## Visão Geral

Este projeto implementa um sistema distribuído de alta performance para ingestão massiva de dados, desenvolvido para um trabalho da faculdade.

A solução utiliza Rust para eficiência e segurança, adotando um modelo assíncrono baseado em filas.

### Objetivos principais

- Suportar alta taxa de requisições simultâneas(testei com 1M) 
- Garantir resiliência em picos de carga  
- Evitar sobrecarga direta no banco de dados  

---

## Arquitetura do Sistema

O sistema segue o padrão de processamento assíncrono baseado em filas, permitindo maior escalabilidade e tolerância a falhas.

### Fluxo de Dados

1. Gerador de carga (k6) - `load_test.js`
  
2. API de Ingestão (Rust + Actix-web) `src/*`  
   - Recebe requisições HTTP  
   - Enfileira mensagens no RabbitMQ  
 
3. Fila (RabbitMQ)  
   Atua como buffer para absorver picos de tráfego  

4. Worker (Rust)`src/worker.rs`
   - Consome mensagens da fila   

5. Banco de Dados (PostgreSQL)  
   Armazena os dados de forma consistente  

---

## Teste de Carga

O sistema foi validado utilizando k6, em ambiente isolado via Docker.

### Configuração do Teste

| Parâmetro | Valor |
|----------|------|
| Volume total | 1.000.000 mensagens |
| Usuários simultâneos | 200 |
| Ambiente | Docker Compose |

---

### Resultados

- Throughput médio: ~18.450 req/s  
- Tempo total: 54,2 segundos  
- Taxa de sucesso: 100%  

---

### Latência

| Métrica | Tempo |
|--------|------|
| Média | 10.07 ms |
| Mediana | 4.45 ms |
| P90 | 11.74 ms |
| P95 | 18.05 ms |

---

## Stack Tecnológica

- Linguagem: Rust  
- Framework Web: Actix-web  
- Mensageria: RabbitMQ  
- Banco de Dados: PostgreSQL  
- Driver/ORM: SQLx  
- Containerização: Docker + Docker Compose  
- Testes de carga: k6  

---

## Testes

O projeto inclui testes automatizados para validação de funcionalidades críticas.

### Executar testes

```bash
cargo test
```

## Como executar o projeto

### pré-requisitos
- Docker
- Docker Compose
- k6 

### subindo o ambiente

``` bash
docker-compose up --build -d
```

**API:** http://localhost:8080

**RabbitMQ:** http://localhost:15672
- usuário: admin
- senha: password123

### Teste de carga

```bash
k6 run load_test.js
```