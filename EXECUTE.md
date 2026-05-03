# 🚀 Gerenciamento de Estoque - Guia de Execução

## Status: ✅ Conectado ao Railway

Seu projeto já está **100% pronto** para rodar com o Railway!

## Pré-requisitos

- ✅ Rust 1.70+ instalado
- ✅ Acesso ao Railway (database MySQL)
- ✅ .env configurado com DATABASE_URL do Railway

## 🔧 Configuração do Railway

### ⚠️ IMPORTANTE (Primeira Execução)

Se é a **primeira vez** executando com este banco, você precisa criar/atualizar as tabelas. Execute o SQL em `src/database/db_estoque.sql` no seu Railway:

**Passo 1:** Acesse seu [Dashboard do Railway](https://railway.app)

**Passo 2:** Selecione seu Database MySQL

**Passo 3:** Clique em "Query" (ferrinha de SQL)

**Passo 4:** Cole e execute este SQL completo:

```sql
-- Adicionar coluna data_entrada à tabela produto (se não existir)
ALTER TABLE produto ADD COLUMN data_entrada DATE NOT NULL DEFAULT CURDATE();

-- Ou crie as tabelas do zero:
CREATE TABLE cargo (
    id int auto_increment primary key,
    nome varchar (70) not null
);

CREATE TABLE usuario (
    id int auto_increment primary key,
    nome varchar (70) not null,
    sobrenome varchar (150) not null,
    cpf char (11) not null,
    datanascimento date not null,
    user varchar(70),
    senha varchar(70),
    fkidcargo int,
    foreign key (fkidcargo) references cargo (id)
);

CREATE TABLE produto (
    id int auto_increment primary key,
    nome varchar(200),
    valorcusto decimal (10,2) not null,
    valorvenda decimal (10,2),
    estoqueatual int default 0,
    pesogramas int,
    status enum ('1' ,'2'),
    dataproducao date not null,
    datavalidade date not null,
    data_entrada date not null
);

INSERT INTO cargo (nome) VALUES ('Admin'), ('Funcionario'), ('Gerente');
INSERT INTO usuario (nome, sobrenome, cpf, datanascimento, user, senha, fkidcargo) 
VALUES ('Rafael', 'Matos Celestino', '00000000000', '2006-06-08', 'rmcelestino', '12345', 3);
```

### ✅ .env - Já Configurado

Seu `.env` já está correto com a URL do Railway:

```
DATABASE_URL=mysql://root:pGAqxILZvJrRxDDJcaUZpiQBgtxKBnyA@roundhouse.proxy.rlwy.net:53493/railway
RUST_LOG=info
```

**Nenhuma alteração necessária!**

## Executando a Aplicação

### Modo Desenvolvimento (Recomendado)
```bash
cargo run
```

**Output esperado:**
```
🚀 Iniciando aplicação de Gerenciamento de Estoque
📡 Conectando ao banco de dados...
✅ Conexão com banco de dados realizada com sucesso!
🌐 Servidor iniciando em http://0.0.0.0:3000
```

A aplicação estará rodando em: **`http://localhost:3001`**

### Executar Testes
```bash
cargo test
```

**Resultado esperado:**
```
test result: ok. 6 passed; 0 failed
```

### Build para Produção
```bash
cargo build --release
```

Executável criado em: `target/release/gerenciamento_de_estoque.exe`

---

## 📡 Endpoints da API

| Método | Endpoint | Descrição |
|--------|----------|-----------|
| GET | `/` | Health Check |
| POST | `/login` | Autenticar usuário |
| GET | `/produtos` | Listar produtos |
| POST | `/produtos/criar` | Criar novo produto |
| POST | `/estoque/entrada` | Registrar entrada de estoque |
| POST | `/estoque/saida` | Registrar saída de estoque |

---

## 🧪 Exemplos de Requisições HTTP

### 1. Health Check (Verificar se API está viva)
```bash
curl http://localhost:3001/
```

**Resposta esperada:**
```json
{
  "status": "ok",
  "message": "API is running"
}
```

### 2. Login (Com o usuário padrão)
```bash
curl -X POST http://localhost:3001/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "rmcelestino",
    "password": "12345"
  }'
```

**Resposta esperada:**
```json
{
  "success": true,
  "message": "Success",
  "user_type": "Gerente"
}
```

### 3. Listar Produtos
```bash
curl http://localhost:3001/produtos
```

### 4. Criar Produto
```bash
curl -X POST http://localhost:3001/produtos/criar \
  -H "Content-Type: application/json"
```

### 5. Entrada de Estoque
```bash
curl -X POST http://localhost:3001/estoque/entrada \
  -H "Content-Type: application/json"
```

### 6. Saída de Estoque
```bash
curl -X POST http://localhost:3001/estoque/saida \
  -H "Content-Type: application/json"
```

---

## 🔐 Credenciais Padrão

Os seguintes usuários estão cadastrados por padrão no Railway:

| Username | Senha | Tipo |
|----------|-------|------|
| rmcelestino | 12345 | Gerente |

---

## 🛠️ Estrutura de Pastas

```
src/
├── config/              # Configuração do banco
├── database/            # Scripts SQL
├── handlers/            # Controladores HTTP
├── models/              # Entidades de domínio
├── repository/          # Acesso a dados
├── routes/              # Definição de rotas
├── services/            # Lógica de negócio
├── lib.rs               # Declaração de módulos
└── main.rs              # Ponto de entrada

tests/
├── auth_handler.rs      # Testes de autenticação
├── auth_service.rs      # Testes de serviço
└── user_test.rs         # Testes de usuário
```

---

## 🐛 Troubleshooting

### ❌ "Erro ao conectar no banco de dados"

**Solução:**
1. Verifique se o Railway está ativo
2. Confirme o DATABASE_URL no `.env`
3. Teste a conexão com `mysql` client

### ❌ "Connection timeout"

**Solução:**
1. Verifique a conexão com a internet
2. Teste a URL do banco independentemente
3. Aguarde alguns segundos (Railway pode estar lento)

### ❌ "Table doesn't exist"

**Solução:**
1. Execute o SQL conforme instruído acima
2. Verifique se as tabelas foram realmente criadas no Railway

---

## 📞 Suporte

Se tiver dúvidas, verifique:
- Arquivo SQL: `src/database/db_estoque.sql`
- Configuração de conexão: `.env`
- Logs da aplicação: saída no console

🎉 **Sua aplicação está 100% pronta para usar!**
