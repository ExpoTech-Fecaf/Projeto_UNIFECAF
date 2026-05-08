# 📦 Sistema de Gerenciamento de Estoque

API REST para gerenciamento de estoque desenvolvida em **Rust** com o framework **Axum**, banco de dados **MySQL** e arquitetura em camadas.

Projeto acadêmico — **UNIFECAF**.

---

## 📐 Arquitetura

O projeto segue o padrão de camadas (Layered Architecture):

```
src/
├── config/          # Configuração (conexão com banco de dados)
├── database/        # Scripts SQL de criação do schema
├── handlers/        # Controladores HTTP (recebem requests e retornam responses)
├── models/          # Structs de domínio (Product, User, Role, Movement)
├── repository/      # Acesso a dados (queries SQL via SQLx)
├── routes/          # Definição das rotas da API
├── services/        # Regras de negócio (FIFO, autenticação, relatórios)
├── validators/      # Validações de entrada (CPF, datas, unicidade)
├── lib.rs           # Declaração dos módulos públicos
└── main.rs          # Ponto de entrada da aplicação
```

**Fluxo de uma requisição:**

```
Request → Router → Handler → Service → Repository → MySQL
```

---

## 🛠️ Tecnologias

| Tecnologia | Versão | Finalidade |
|---|---|---|
| Rust | Edition 2024 | Linguagem principal |
| Axum | 0.8 | Framework HTTP |
| SQLx | 0.7 | Driver MySQL assíncrono |
| Tokio | 1.x | Runtime assíncrono |
| Bcrypt | 0.14 | Hash de senhas |
| Chrono | 0.4 | Manipulação de datas |
| Serde | 1.0 | Serialização JSON |
| Docker | — | Containerização |

---

## 🚀 Como Executar

### Pré-requisitos

- Rust (rustup) instalado
- MySQL 8.x rodando
- Arquivo `.env` configurado

### Configuração

```bash
# Clone o repositório
cd Projeto_UNIFECAF

# Copie o arquivo de ambiente e preencha com seus dados
cp .env.example .env
```

Conteúdo do `.env`:
```env
DATABASE_URL=mysql://usuario:senha@localhost:3306/gestao
```

### Criação do Banco de Dados

Execute o script SQL em `src/database/db_estoque.sql` para criar as tabelas:

```sql
-- 1. Roles (perfis de acesso)
CREATE TABLE roles (
    id   SMALLINT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(50) NOT NULL UNIQUE
);

INSERT INTO roles(name) VALUES ('Admin'), ('Gerente'), ('Funcionario');

-- 2. Usuários
CREATE TABLE users (
    id            INT PRIMARY KEY AUTO_INCREMENT,
    username      VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    user_type     ENUM('Admin', 'Gerente', 'Funcionario') NOT NULL,
    first_name    VARCHAR(100) NOT NULL,
    last_name     VARCHAR(100) NOT NULL,
    birth_date    DATE NOT NULL,
    cpf           VARCHAR(14)  NOT NULL UNIQUE,
    role_id       SMALLINT NOT NULL,
    FOREIGN KEY (role_id) REFERENCES roles(id)
);

-- 3. Produtos
CREATE TABLE products (
    id              INT PRIMARY KEY AUTO_INCREMENT,
    name            VARCHAR(150) NOT NULL,
    cost_price      DOUBLE NOT NULL,
    sale_price      DOUBLE NOT NULL,
    current_stock   INT NOT NULL DEFAULT 0,
    weight_grams    INT NOT NULL,
    status          SMALLINT NOT NULL DEFAULT 1, -- 1=ativo, 2=inativo
    production_date DATE NOT NULL,
    expiration_date DATE NOT NULL,
    entry_date      DATE NOT NULL,
    min_stock       INT NOT NULL DEFAULT 0       -- alerta de estoque baixo
);

-- 4. Lotes (FIFO)
CREATE TABLE batches (
    id          INT PRIMARY KEY AUTO_INCREMENT,
    product_id  INT NOT NULL,
    quantity    INT NOT NULL,
    entry_date  DATE NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products(id)
);

-- 5. Movimentações (histórico)
CREATE TABLE movements (
    id            INT PRIMARY KEY AUTO_INCREMENT,
    product_id    INT NOT NULL,
    batch_id      INT,
    user_id       INT NOT NULL,
    movement_type ENUM('entrada', 'saida') NOT NULL,
    quantity      INT NOT NULL,
    created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    notes         VARCHAR(255),
    FOREIGN KEY (product_id) REFERENCES products(id),
    FOREIGN KEY (batch_id)   REFERENCES batches(id),
    FOREIGN KEY (user_id)    REFERENCES users(id)
);
```

| Tabela | Descrição |
|---|---|
| `roles` | Perfis de acesso (Admin, Gerente, Funcionário) |
| `users` | Usuários com autenticação, CPF e vínculo a cargo |
| `products` | Produtos com controle de estoque, datas e status |
| `batches` | Lotes individuais para ordenação FIFO por `entry_date` |
| `movements` | Histórico de entradas e saídas de estoque |

### Execução

```bash
# Modo desenvolvimento
cargo run

# Modo release
cargo build --release
./target/release/gerenciamento_de_estoque
```

O servidor inicia em `http://0.0.0.0:3001`.

### Docker

```bash
docker build -t estoque-api .
docker run -p 3001:3001 --env-file .env estoque-api
```

---

## 📡 Endpoints da API

### Health Check

| Método | Rota | Descrição |
|---|---|---|
| GET | `/` | Verifica se a API está online |

### Autenticação e Usuários

| Método | Rota | Descrição |
|---|---|---|
| POST | `/login` | Autenticação de usuário |
| POST | `/register` | Cadastro de novo usuário |
| GET | `/users` | Lista todos os usuários |
| GET | `/users/{id}` | Busca usuário por ID |
| PUT | `/users/update/{id}` | Atualiza dados do usuário |
| DELETE | `/users/delete/{id}` | Remove um usuário |
| POST | `/users/promote` | Promove usuário (requer Admin) |

### Produtos

| Método | Rota | Descrição |
|---|---|---|
| GET | `/products` | Lista todos os produtos |
| GET | `/products/{id}` | Busca produto por ID |
| POST | `/products/create` | Cadastra novo produto |
| PUT | `/products/update/{id}` | Atualiza produto |
| DELETE | `/products/delete/{id}` | Remove produto |

### Estoque (FIFO)

| Método | Rota | Descrição |
|---|---|---|
| POST | `/products/stock/entry` | Entrada de estoque |
| POST | `/products/stock/exit` | Saída de estoque (FIFO) |
| GET | `/products/stock/{name}` | Consulta estoque por nome |

### Movimentações

| Método | Rota | Descrição |
|---|---|---|
| GET | `/movements` | Lista todo o histórico |
| GET | `/movements/product/{id}` | Histórico por produto |

### Relatórios

| Método | Rota | Descrição |
|---|---|---|
| GET | `/reports/stock` | Relatório completo de estoque |
| GET | `/reports/critical` | Produtos com estoque crítico (≤ 5 unidades) |
| GET | `/reports/alerts` | Alertas de consumo ajustados por dia da semana |
| GET | `/reports/low-stock` | Avisos de produtos com estoque abaixo do mínimo |

---

## 📋 Exemplos de Requisição

### Registrar Usuário

```json
POST /register
{
  "username": "joao.silva",
  "password": "senha123",
  "first_name": "João",
  "last_name": "Silva",
  "birth_date": "15/03/1990",
  "cpf": "12345678909",
  "role_id": 2
}
```

### Criar Produto

```json
POST /products/create
{
  "name": "Arroz Integral 1kg",
  "cost_price": 5.50,
  "sale_price": 8.99,
  "current_stock": 100,
  "weight_grams": 1000,
  "status": 1,
  "production_date": "01/01/2025",
  "expiration_date": "01/01/2026"
}
```

### Saída de Estoque (FIFO)

```json
POST /products/stock/exit
{
  "product_name": "Arroz Integral 1kg",
  "quantity": 30,
  "user_id": 1,
  "notes": "Venda para cliente X"
}
```

Resposta com avisos (não bloqueantes):
```json
{
  "success": true,
  "message": "Saída de estoque registrada",
  "product_name": "Pão de Hambúrguer",
  "quantity_removed": 90,
  "remaining_stock": 12,
  "aviso_estoque_baixo": {
    "alerta": true,
    "mensagem": "⚠ Aviso: estoque baixo. O produto \"Pão de Hambúrguer\" está próximo de acabar. Quantidade atual: 12 unidades.",
    "current_stock": 12,
    "min_stock": 15
  },
  "aviso_consumo_elevado": {
    "alerta": true,
    "mensagem": "⚠ Alerta: quantidade elevada para o dia. Limite recomendado para Sabado (Muito Alto): 80 unidades. Solicitado: 90 unidades."
  }
}
```

---

## 🔄 Lógica FIFO (First In, First Out)

A retirada de estoque consome os lotes mais antigos primeiro:

1. Busca todos os lotes do produto ordenados por `entry_date ASC`
2. Valida se o estoque total é suficiente
3. Itera pelos lotes do mais antigo ao mais novo, consumindo cada um até atingir a quantidade solicitada
4. Persiste as alterações no banco de dados
5. Registra a movimentação no histórico

---

## 🔐 Sistema de Permissões

| Role ID | Tipo | Nível |
|---|---|---|
| 1 | Admin | 3 (acesso total) |
| 2 | Funcionário | 1 (acesso básico) |
| 3 | Gerente | 2 (acesso intermediário) |

A promoção de usuários requer nível de Admin.

---

## 🚨 Sistema de Alertas

O sistema possui três mecanismos de alerta complementares:

### Aviso de Estoque Baixo
Quando o estoque total de um produto atinge ou fica abaixo do `min_stock` definido, o sistema exibe um aviso informativo. Não bloqueia operações.

### Alerta de Consumo Elevado
Na saída de estoque, se a quantidade retirada exceder o limite recomendado para o dia da semana (calculado como `min_stock × multiplicador`), o sistema retorna um alerta. Não bloqueia a operação.

| Dia | Nível | Multiplicador |
|---|---|---|
| Segunda | Baixo | 0.5x |
| Terça / Quarta | Médio | 1.0x |
| Quinta / Sexta | Alto | 1.3x |
| Sábado / Domingo | Muito Alto | 1.6x |

### Relatório de Alertas por Dia
Endpoint dedicado (`GET /reports/alerts`) que analisa todos os produtos e indica quais estão com estoque abaixo do mínimo ajustado para o dia atual.

---

## ✅ Validações

### Usuário
- Username único no banco
- CPF com 11 dígitos, validação de dígitos verificadores
- Data de nascimento no formato `dd/mm/YYYY` e não pode ser futura
- Role ID deve ser 1, 2 ou 3

### Produto
- Nome único (case-insensitive)
- Status deve ser 1 (ativo) ou 2 (inativo)
- Datas no formato `dd/mm/YYYY`

---

## 📁 Estrutura de Módulos

| Módulo | Responsabilidade |
|---|---|
| `config::database` | Conexão com MySQL via pool de conexões |
| `models::product` | Struct Product (lote de produto) |
| `models::user` | Struct User + enum UserType |
| `models::role` | Struct Role (cargo) |
| `models::movement` | Struct Movement (histórico) |
| `models::consumo` | Enums DiaSemana + NivelMovimento (alertas) |
| `repository::product_repository` | CRUD de produtos + queries FIFO + relatórios |
| `repository::user_repository` | CRUD de usuários + promoção |
| `repository::role_repository` | CRUD de cargos |
| `repository::movement_repository` | Registro e consulta de movimentações |
| `services::stock_service` | Lógica FIFO + relatórios + alertas de consumo + aviso de estoque baixo |
| `services::product_service` | Delegação para o repositório de produtos |
| `services::auth_service` | Hash de senha, autenticação, permissões |
| `services::user_service` | Delegação para o repositório de usuários |
| `services::role_service` | Delegação para o repositório de cargos |
| `handlers::stock_handler` | Endpoints de estoque e relatórios |
| `handlers::product_handler` | Endpoints CRUD de produtos |
| `handlers::auth_handler` | Endpoints de login, registro e gestão de usuários |
| `validators::user_validator` | Validação de CPF, datas, username |
| `validators::product_validator` | Validação de nome, status, datas de produto |

---

## 📄 Licença

Projeto acadêmico — UNIFECAF.
