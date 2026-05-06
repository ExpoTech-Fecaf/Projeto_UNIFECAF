# 📦 Gerenciamento de Estoque - Guia de Execução e Testes

## Pré-requisitos

- **Rust** (instalar via [rustup](https://rustup.rs/))
- **MySQL** rodando localmente ou em um servidor acessível
- **curl**, **Postman** ou **Insomnia** para testar as rotas

---

## 1. Configuração do Banco de Dados

Crie o banco e as tabelas no MySQL:

```sql
CREATE DATABASE gestao;
USE gestao;

CREATE TABLE products (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(200),
    cost_price DECIMAL(10,2) NOT NULL,
    sale_price DECIMAL(10,2),
    current_stock INT DEFAULT 0,
    weight_grams INT,
    status SMALLINT DEFAULT 1,
    production_date DATE NOT NULL,
    expiration_date DATE NOT NULL,
    entry_date DATE NOT NULL
);

CREATE TABLE roles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(70) NOT NULL
);

CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(70) NOT NULL,
    last_name VARCHAR(150) NOT NULL,
    cpf CHAR(11) NOT NULL,
    birth_date DATE NOT NULL,
    username VARCHAR(70),
    password_hash VARCHAR(255),
    role_id SMALLINT,
    FOREIGN KEY (role_id) REFERENCES roles(id)
);

INSERT INTO roles (name) VALUES ('Admin'), ('Funcionario'), ('Gerente');
```

---

## 2. Configuração do Arquivo .env

Crie um arquivo `.env` na raiz do projeto (`Projeto_UNIFECAF/.env`):

```env
DATABASE_URL=mysql://SEU_USUARIO:SUA_SENHA@localhost:3306/gestao
```

Exemplo:
```env
DATABASE_URL=mysql://root:123456@localhost:3306/gestao
```

---

## 3. Rodando o Projeto

```bash
cd Projeto_UNIFECAF
cargo run
```

Se tudo estiver certo, vai aparecer:
```
🚀 Iniciando aplicação de Gerenciamento de Estoque
📡 Conectando ao banco de dados...
✅ Conexão com banco de dados realizada com sucesso!
🌐 Servidor iniciando em http://0.0.0.0:3001
```

> **Dica:** Para ver os logs, rode com `RUST_LOG=info cargo run`

---

## 4. Testando as Rotas

A API roda em `http://localhost:3001`. Abaixo estão todos os endpoints disponíveis.

### Health Check

```bash
curl http://localhost:3001/
```

---

### 🛒 CRUD de Produtos

#### Criar produto
```bash
curl -X POST http://localhost:3001/products/create \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Arroz 5kg",
    "cost_price": 15.50,
    "sale_price": 22.90,
    "current_stock": 100,
    "weight_grams": 5000,
    "status": 1,
    "production_date": "01/03/2025",
    "expiration_date": "01/03/2026"
  }'
```

#### Listar todos os produtos
```bash
curl http://localhost:3001/products
```

#### Buscar produto por ID
```bash
curl http://localhost:3001/products/1
```

#### Atualizar produto
```bash
curl -X PUT http://localhost:3001/products/update/1 \
  -H "Content-Type: application/json" \
  -d '{
    "id": 1,
    "name": "Arroz 5kg Premium",
    "cost_price": 18.00,
    "sale_price": 25.90,
    "current_stock": 100,
    "weight_grams": 5000,
    "status": 1,
    "production_date": "2025-03-01",
    "expiration_date": "2026-03-01",
    "entry_date": "2025-06-01"
  }'
```

#### Deletar produto
```bash
curl -X DELETE http://localhost:3001/products/delete/1
```

---

### 📦 Movimentação de Estoque

#### Entrada de estoque
```bash
curl -X POST http://localhost:3001/products/stock/entry \
  -H "Content-Type: application/json" \
  -d '{
    "product_name": "Arroz 5kg",
    "quantity": 50
  }'
```

Resposta:
```json
{
  "success": true,
  "message": "Entrada de estoque registrada",
  "product_name": "Arroz 5kg",
  "quantity_added": 50,
  "total_stock": 150
}
```

#### Saída de estoque (FIFO)
```bash
curl -X POST http://localhost:3001/products/stock/exit \
  -H "Content-Type: application/json" \
  -d '{
    "product_name": "Arroz 5kg",
    "quantity": 30
  }'
```

Resposta:
```json
{
  "success": true,
  "message": "Saída de estoque registrada",
  "product_name": "Arroz 5kg",
  "quantity_removed": 30,
  "remaining_stock": 120
}
```

#### Consultar estoque de um produto
```bash
curl http://localhost:3001/products/stock/Arroz%205kg
```

Resposta:
```json
{
  "product_name": "Arroz 5kg",
  "total_stock": 120,
  "batches": [
    {
      "batch_id": 1,
      "current_stock": 120,
      "entry_date": "2025-06-01"
    }
  ]
}
```

---

### 👤 Usuários

#### Registrar usuário
```bash
curl -X POST http://localhost:3001/register \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "João",
    "last_name": "Silva",
    "cpf": "12345678901",
    "birth_date": "1990-05-15",
    "username": "joaosilva",
    "password": "senha123",
    "role_id": 2
  }'
```

#### Login
```bash
curl -X POST http://localhost:3001/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "joaosilva",
    "password": "senha123"
  }'
```

#### Listar usuários
```bash
curl http://localhost:3001/users
```

#### Buscar usuário por ID
```bash
curl http://localhost:3001/users/1
```

#### Atualizar usuário
```bash
curl -X PUT http://localhost:3001/users/update/1 \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "João",
    "last_name": "Santos",
    "cpf": "12345678901",
    "birth_date": "1990-05-15",
    "username": "joaosilva",
    "password": "novaSenha456",
    "role_id": 2
  }'
```

#### Deletar usuário
```bash
curl -X DELETE http://localhost:3001/users/delete/1
```

---

## 5. Estrutura do Projeto

```
src/
├── config/          → Configuração do banco de dados
├── handlers/        → Controllers (recebem as requisições HTTP)
├── models/          → Structs que representam as tabelas
├── repository/      → Acesso direto ao banco (queries SQL)
├── routes/          → Definição das rotas da API
├── services/        → Lógica de negócio (FIFO, validações)
├── validators/      → Validações de entrada
├── lib.rs           → Exportação dos módulos
└── main.rs          → Ponto de entrada da aplicação
```

---

## 6. Possíveis Erros

| Erro | Solução |
|------|---------|
| `Erro ao conectar no banco de dados` | Verifique se o MySQL está rodando e se o `.env` está correto |
| `Erro ao fazer bind do servidor` | A porta 3001 já está em uso. Mate o processo ou mude a porta |
| `ColumnDecode` / `mismatched types` | As colunas do banco não batem com o código. Recrie as tabelas com o SQL acima |

---

## 7. Roles (Cargos)

| ID | Cargo |
|----|-------|
| 1  | Admin |
| 2  | Funcionario |
| 3  | Gerente |
