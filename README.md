# 📦 Sistema de Gerenciamento de Estoque (UNIFECAF)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-v0.7-blue?style=for-the-badge)
![MySQL](https://img.shields.io/badge/mysql-%2300f.svg?style=for-the-badge&logo=mysql&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

Uma aplicação backend robusta desenvolvida em **Rust** utilizando o framework **Axum**. O sistema fornece uma API REST completa para o gerenciamento inteligente de produtos, controle de lotes com lógica **FIFO (First In, First Out)**, rastreamento de movimentações e geração de relatórios operacionais. O projeto conta também com uma interface frontend estática para administração e operação local.

---

## 📌 Índice

- [Status do Projeto](#-status-do-projeto)
- [Tecnologias Utilizadas](#-tecnologias-utilizadas)
- [Arquitetura e Estrutura de Pastas](#-arquitetura-e-estrutura-de-pastas)
- [Configuração do Ambiente](#%EF%B8%8F-configura%C3%A7%C3%A3o-do-ambiente)
- [Instalação e Execução](#-instala%C3%A7%C3%A3o-e-execu%C3%A7%C3%A3o)
- [Endpoints Principais](#-endpoints-principais)
- [Regras de Negócio e FIFO](#-regras-de-neg%C3%B3cio-e-fifo)
- [Contribuição](#-contribui%C3%A7%C3%A3o)
- [Licença e Contato](#-licen%C3%A7a-e-contato)

---

## 🚀 Status do Projeto

- **Versão Atual:** 1.0.0 (Acadêmico/MVP)
- **Status:** Funcional para uso local, apresentações e deploys em ambientes de desenvolvimento.
- **Testes:** Cobertura básica em implementação (contribuições são muito bem-vindas!).

---

## 🛠️ Tecnologias Utilizadas

- **Linguagem:** Rust (Garantia de performance, concorrência segura e gerenciamento de memória eficiente)
- **Framework Web:** [Axum](https://github.com/tokio-rs/axum) (Baseado em Tokio, ideal para serviços de alta performance)
- **Async Runtime:** [Tokio](https://tokio.rs/)
- **Persistência de Dados:** [SQLx](https://github.com/launchbadge/sqlx) (Queries SQL em tempo de compilação com MySQL)
- **Banco de Dados:** MySQL 8.x
- **Frontend:** HTML5, CSS3 e JavaScript (Single Page Application estática contida no diretório `frontend/`)

---

## 📂 Arquitetura e Estrutura de Pastas

O projeto adota uma arquitetura em camadas limpa, facilitando a manutenção e a escalabilidade do código Rust:

```text
Projeto_UNIFECAF/
├── docs/                  # Documentação técnica e fluxogramas do sistema
├── frontend/              # Interface web (HTML, CSS, JS estáticos)
├── src/
│   ├── database/          # Scripts SQL de migração e estrutura do banco
│   ├── handlers/          # Camada de controle (HTTP Requests/Responses)
│   ├── models/            # Entidades do domínio e Structs de validação
│   ├── repository/        # Camada de persistência e comunicação com SQLx
│   ├── services/          # Regras de negócio (Cálculos, validações e lógica FIFO)
│   └── main.rs            # Ponto de entrada da aplicação e configuração das rotas
├── .env.example           # Modelo de variáveis de ambiente
├── Cargo.toml             # Gerenciador de dependências do Rust
└── Dockerfile             # Configuração para containerização da API
```

## ⚙️ Configuração do Ambiente

Antes de iniciar, você precisará configurar as variáveis de ambiente. Copie o arquivo de exemplo:

```bash
cp .env.example .env
```

Abra o arquivo .env gerado e insira a string de conexão correspondente ao seu banco de dados MySQL:

```
DATABASE_URL=mysql://seu_usuario:sua_senha@localhost:3306/nome_do_banco
PORT=3001
```
## 💻 Instalação e Execução
Pré-requisitos

    Rustup / Cargo (Versão estável mais recente)

    MySQL Server 8.x instalado e rodando

### 1. Preparando o Banco de Dados

Crie o banco de dados no seu MySQL e execute o script estrutural para subir as tabelas:

```bash
mysql -u seu_usuario -p nome_do_banco < src/database/db_estoque.sql
```
### 2. Executando a Aplicação
Modo Desenvolvimento (Live Reload/Compilação rápida):

```bash
cargo run
```
A API estará disponível em http://localhost:3001.
Modo Produção / Release (Otimizado):
```bash
cargo build --release
./target/release/gerenciamento_de_estoque
```

### Executando com Docker 🐳

Se preferir rodar o backend isolado em um container:

```bash
# Construir a imagem
docker build -t gerenciamento_estoque .

# Executar o container passando o arquivo de ambiente
docker run -p 3001:3001 --env-file .env gerenciamento_estoque
```
## 🛣️ Endpoints Principais (Resumo)
### Autenticação & Usuários

POST /register — Cadastra um novo operador/administrador.

POST /login — Autentica o usuário e gera a sessão/token.

### Produtos & Estoque

GET /products — Lista todos os produtos cadastrados com paginação/filtros.

POST /products/create — Registra um novo produto no catálogo.

POST /products/stock/entry — Registra a entrada de mercadoria (Gera um novo lote).

POST /products/stock/exit — Registra a saída de mercadoria (Baixa automática usando FIFO).

### Relatórios

GET /reports/alerts — Exibe produtos com estoque crítico ou lotes próximos do vencimento.

## 🧪 Exemplo de Uso Prático (cURL)

### Criar Produto:

```Bash
curl -X POST http://localhost:3001/products/create \
    -H "Content-Type: application/json" \
    -d '{
      "name": "Arroz Integral 1kg",
      "cost_price": 5.50,
      "sale_price": 8.99,
      "current_stock": 0,
      "weight_grams": 1000,
      "status": 1,
      "production_date": "2025-01-01",
      "expiration_date": "2026-01-01"
    }'
```
## 🧠 Regras de Negócio e FIFO

Para evitar desperdícios e obsolescência de produtos perecíveis, este sistema utiliza estritamente o método FIFO (First In, First Out) na movimentação de saídas:

Ao realizar um POST /products/stock/exit, o serviçolocaliza os lotes ativos do produto.

A baixa do estoque é realizada primeiramente no lotemais antigo que ainda possua quantidades disponíveis.

Caso a quantidade de saída seja maior que o lotemais antigo, o sistema consome o saldo restante doslotes subsequentes de forma encadeada e automática.

Para detalhes profundos sobre diagramas de blocos,arquitetura SQLx e modelos de dados, consulte oarquivo docs/DOCUMENTACAO_TECNICA.md.

✉️ Contato

Desenvolvido como projeto acadêmico pela Equipe da Turma 3ADS.NA.

Caso encontre bugs ou queira sugerir melhorias, sinta-se à vontade para abrir uma Issue.

## 🧪 Testes

O projeto contém testes unitários localizados na pasta `tests/` e pode ser executado com o comando padrão do Cargo:

```bash
cargo test
```

Notas importantes:
- Testes unitários que não dependem de banco de dados rodam sem configuração adicional.
- Testes que interagem com o banco (integração) exigem uma instância MySQL e a variável `DATABASE_URL` corretamente configurada no arquivo `.env` ou no ambiente.
- Para rodar um único arquivo de teste use:

```bash
cargo test --test nome_do_teste
```

Exemplo para rodar apenas os testes de validadores:

```bash
cargo test --test validators_test
```

Se quiser que eu adicione testes de integração (requere MySQL), posso criar scripts de inicialização e fixtures para facilitar a execução em ambiente local ou via Docker.