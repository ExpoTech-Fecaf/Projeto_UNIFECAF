# 🖥️ Especificação de Telas — Sistema de Gerenciamento de Estoque

Documento que descreve todas as telas do sistema, suas funcionalidades e as permissões de acesso por tipo de usuário.

---

## 🔐 Matriz de Permissões

| Funcionalidade | Admin | Gerente | Funcionário |
|---|:---:|:---:|:---:|
| Login | ✅ | ✅ | ✅ |
| Cadastrar funcionário | ✅ | ❌ | ❌ |
| Cadastrar produto | ❌ | ✅ | ❌ |
| Retirar produto do estoque | ❌ | ✅ | ✅ |
| Visualizar estoque | ✅ | ✅ | ✅ |
| Visualizar relatórios | ✅ | ✅ | ❌ |
| Alertas de consumo por dia | ✅ | ✅ | ❌ |
| Avisos de estoque baixo | ✅ | ✅ | ❌ |
| Promover usuário | ✅ | ❌ | ❌ |

---

## 📄 Telas

---

### 1. Tela de Health Check

**Rota:** `GET /`

**Acesso:** Público

**Descrição:**
Endpoint de verificação de saúde da API. Retorna status indicando que o servidor está online.

**Resposta:**
```json
{ "status": "ok", "message": "API is running" }
```

---

### 2. Tela de Login

**Rota:** `POST /login`

**Acesso:** Todos (público)

**Descrição:**
Tela inicial do sistema. O usuário informa suas credenciais para acessar o painel correspondente ao seu perfil.

**Campos:**
- Username
- Senha

**Comportamento:**
- Após login bem-sucedido, retorna o tipo de usuário (Admin, Gerente, Funcionário) para redirecionar ao painel correto
- Em caso de falha, exibe mensagem de erro ("Usuário ou senha incorretos")

**Payload:**
```json
{
  "username": "joao.silva",
  "password": "senha123"
}
```

---

### 3. Tela de Cadastro de Funcionário (Registro)

**Rota:** `POST /register`

**Acesso:** Somente Admin

**Descrição:**
Permite ao administrador cadastrar novos usuários no sistema (Funcionários, Gerentes ou outros Admins).

**Campos:**
| Campo | Tipo | Validação |
|---|---|---|
| Username | Texto | Único no sistema |
| Senha | Senha | Obrigatório |
| Primeiro nome | Texto | Obrigatório |
| Sobrenome | Texto | Obrigatório |
| Data de nascimento | Data (dd/mm/YYYY) | Não pode ser futura |
| CPF | Texto (11 dígitos) | Validação de dígitos verificadores |
| Cargo (role_id) | Select (1=Admin, 2=Funcionário, 3=Gerente) | Obrigatório |

**Comportamento:**
- Valida todos os campos antes de enviar
- Exibe mensagem de sucesso com o ID do usuário criado
- Exibe mensagem de erro específica por campo em caso de falha

**Payload:**
```json
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

---

### 4. Tela de Listagem de Usuários

**Rota:** `GET /users`

**Acesso:** Somente Admin

**Descrição:**
Lista todos os usuários cadastrados no sistema.

**Colunas da tabela:**
- ID
- Nome completo
- Username
- CPF
- Cargo
- Ações (Editar, Excluir, Promover)

---

### 5. Tela de Detalhes do Usuário

**Rota:** `GET /users/{id}`

**Acesso:** Somente Admin

**Descrição:**
Exibe os dados completos de um usuário específico.

**Informações exibidas:**
- ID, Username, Nome, Sobrenome, CPF, Data de nascimento, Cargo

---

### 6. Tela de Edição de Usuário

**Rota:** `PUT /users/update/{id}`

**Acesso:** Somente Admin

**Descrição:**
Permite ao administrador alterar os dados de um usuário existente.

**Campos:** Mesmos do cadastro, pré-preenchidos com os dados atuais.

**Validações:**
- Username único (excluindo o próprio usuário)
- CPF único (excluindo o próprio usuário)
- Role ID válido (1, 2 ou 3)
- CPF com dígitos verificadores válidos

**Fluxo:**
1. Carrega dados via `GET /users/{id}`
2. Exibe formulário pré-preenchido
3. Salva alterações via `PUT /users/update/{id}`

---

### 7. Tela de Exclusão de Usuário

**Rota:** `DELETE /users/delete/{id}`

**Acesso:** Somente Admin

**Descrição:**
Remove um usuário do sistema.

**Comportamento:**
- Exibe modal de confirmação antes de excluir
- Retorna status 204 (No Content) em caso de sucesso

---

### 8. Tela de Promoção de Usuário

**Rota:** `POST /users/promote`

**Acesso:** Somente Admin

**Descrição:**
Permite ao administrador alterar o cargo de um usuário.

**Campos:**
| Campo | Tipo | Validação |
|---|---|---|
| ID do usuário | Inteiro | Deve existir no sistema |
| Novo cargo (new_role_id) | Select (1=Admin, 2=Funcionário, 3=Gerente) | Obrigatório |

**Payload:**
```json
{
  "users_id": 5,
  "new_role_id": 3
}
```

**Comportamento:**
- Verifica se o usuário logado é Admin
- Retorna erro 403 se não tiver permissão

---

### 9. Tela de Listagem de Produtos

**Rota:** `GET /products`

**Acesso:** Admin, Gerente

**Descrição:**
Lista todos os produtos cadastrados com informações de estoque.

**Colunas da tabela:**
- ID
- Nome
- Preço de custo
- Preço de venda
- Estoque atual
- Peso (gramas)
- Status (Ativo/Inativo)
- Data de produção
- Data de validade
- Ações (Editar, Excluir) — somente Gerente

---

### 10. Tela de Detalhes do Produto

**Rota:** `GET /products/{id}`

**Acesso:** Admin, Gerente

**Descrição:**
Exibe os dados completos de um produto específico.

---

### 11. Tela de Cadastro de Produto

**Rota:** `POST /products/create`

**Acesso:** Somente Gerente

**Descrição:**
Permite ao gerente cadastrar novos produtos no sistema de estoque.

**Campos:**
| Campo | Tipo | Validação |
|---|---|---|
| Nome | Texto | Único (case-insensitive) |
| Preço de custo | Decimal | Obrigatório, > 0 |
| Preço de venda | Decimal | Obrigatório, > 0 |
| Estoque inicial | Inteiro | Obrigatório, ≥ 0 |
| Peso (gramas) | Inteiro | Obrigatório, > 0 |
| Status | Select (1=Ativo, 2=Inativo) | Padrão: 1 (Ativo) |
| Data de produção | Data (dd/mm/YYYY) | Obrigatório |
| Data de validade | Data (dd/mm/YYYY) | Obrigatório |

**Comportamento:**
- Data de entrada é preenchida automaticamente (data atual)
- Valida nome único antes de enviar
- Exibe mensagem de sucesso com o ID do produto criado

**Payload:**
```json
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

---

### 12. Tela de Edição de Produto

**Rota:** `PUT /products/update/{id}`

**Acesso:** Somente Gerente

**Descrição:**
Permite ao gerente alterar os dados de um produto existente.

**Fluxo:**
1. Carrega dados via `GET /products/{id}`
2. Exibe formulário pré-preenchido
3. Salva alterações via `PUT /products/update/{id}`

**Validações:**
- Nome único (excluindo o próprio produto)
- Status válido (1 ou 2)

---

### 13. Tela de Exclusão de Produto

**Rota:** `DELETE /products/delete/{id}`

**Acesso:** Somente Gerente

**Descrição:**
Remove um produto do sistema.

**Comportamento:**
- Exibe modal de confirmação antes de excluir
- Retorna status 204 (No Content) em caso de sucesso

---

### 14. Tela de Entrada de Estoque

**Rota:** `POST /products/stock/entry`

**Acesso:** Somente Gerente

**Descrição:**
Permite registrar a entrada de novos produtos/lotes no estoque.

**Campos:**
| Campo | Tipo | Validação |
|---|---|---|
| Nome do produto | Texto/Autocomplete | Deve existir no sistema |
| Quantidade | Inteiro | > 0 |
| ID do usuário | Inteiro | Usuário logado |
| Observações | Texto (opcional) | — |

**Comportamento:**
- Adiciona a quantidade ao lote mais recente do produto
- Registra a movimentação no histórico
- Exibe o novo total de estoque

**Payload:**
```json
{
  "product_name": "Arroz Integral 1kg",
  "quantity": 50,
  "user_id": 2,
  "notes": "Reposição de fornecedor"
}
```

---

### 15. Tela de Retirada de Estoque (Saída FIFO)

**Rota:** `POST /products/stock/exit`

**Acesso:** Gerente e Funcionário

**Descrição:**
Permite registrar a saída de produtos do estoque. A retirada segue a lógica FIFO (lotes mais antigos são consumidos primeiro).

**Campos:**
| Campo | Tipo | Validação |
|---|---|---|
| Nome do produto | Texto/Autocomplete | Deve existir no sistema |
| Quantidade | Inteiro | > 0, ≤ estoque disponível |
| ID do usuário | Inteiro | Usuário logado |
| Observações | Texto (opcional) | — |

**Comportamento:**
- Ao digitar o nome, exibe sugestões de produtos existentes
- Mostra o estoque disponível do produto selecionado
- Após confirmar, exibe:
  - Quantidade retirada
  - Estoque restante
- Se estoque insuficiente, exibe mensagem de erro com o disponível
- **Aviso de estoque baixo:** se após a retirada o estoque ficar ≤ `min_stock`, exibe alerta informativo (não bloqueia)
- **Alerta de consumo elevado:** se a quantidade retirada exceder o limite recomendado para o dia da semana (`min_stock × multiplicador`), exibe alerta informativo (não bloqueia)

**Payload:**
```json
{
  "product_name": "Pão de Hambúrguer",
  "quantity": 90,
  "user_id": 1,
  "notes": "Preparo para evento"
}
```

**Exemplo de resposta com avisos:**
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

### 16. Tela de Consulta de Estoque por Produto

**Rota:** `GET /products/stock/{name}`

**Acesso:** Admin, Gerente, Funcionário

**Descrição:**
Permite consultar o estoque de um produto específico, exibindo detalhes por lote.

**Campo de busca:**
- Nome do produto (passado na URL)

**Resultado exibido:**
- Nome do produto
- Estoque total
- Detalhes por lote:
  - ID do lote
  - Quantidade no lote
  - Data de entrada

---

### 17. Tela de Histórico de Movimentações

**Rota:** `GET /movements`

**Acesso:** Admin, Gerente

**Descrição:**
Exibe o histórico completo de entradas e saídas de estoque, ordenado por data (mais recentes primeiro).

**Colunas da tabela:**
- ID
- Produto (product_id)
- Lote (batch_id)
- Usuário (user_id)
- Tipo (entrada/saída)
- Quantidade
- Data/Hora
- Observações

---

### 18. Tela de Movimentações por Produto

**Rota:** `GET /movements/product/{product_id}`

**Acesso:** Admin, Gerente

**Descrição:**
Exibe o histórico de movimentações filtrado por um produto específico.

**Colunas:** Mesmas da tela 17, filtradas pelo product_id informado na URL.

---

### 19. Tela de Relatório de Estoque

**Rota:** `GET /reports/stock`

**Acesso:** Admin, Gerente

**Descrição:**
Relatório consolidado de todo o estoque, agrupado por produto.

**Informações exibidas por produto:**
- ID do produto
- Nome do produto
- Status (Ativo/Inativo)
- Estoque total
- Detalhes de cada lote (ID, quantidade, data de entrada)

---

### 20. Tela de Relatório de Estoque Crítico

**Rota:** `GET /reports/critical`

**Acesso:** Admin, Gerente

**Descrição:**
Lista produtos com estoque total ≤ 5 unidades, sinalizando necessidade de reposição.

**Informações exibidas:**
- Nome do produto
- Estoque total (destacado em vermelho)
- Lotes restantes com suas quantidades

---

### 21. Tela de Alertas de Consumo por Dia da Semana

**Rota:** `GET /reports/alerts`

**Acesso:** Admin, Gerente

**Descrição:**
Exibe alertas de estoque ajustados pelo nível de movimentação esperado para o dia da semana atual. O sistema multiplica o `min_stock` de cada produto por um fator baseado no dia:

| Dia | Nível | Multiplicador |
|---|---|---|
| Segunda | Baixo | 0.5x |
| Terça / Quarta | Médio | 1.0x |
| Quinta / Sexta | Alto | 1.3x |
| Sábado / Domingo | Muito Alto | 1.6x |

**Informações exibidas:**
- Dia da semana atual e nível de movimento
- Total de produtos analisados
- Quantidade de produtos em alerta
- Para cada produto:
  - Nome
  - Estoque atual
  - Estoque mínimo original (`min_stock`)
  - Estoque mínimo ajustado (min_stock × multiplicador)
  - Flag de alerta (estoque atual ≤ mínimo ajustado)
  - Mensagem descritiva

---

### 22. Tela de Avisos de Estoque Baixo

**Rota:** `GET /reports/low-stock`

**Acesso:** Admin, Gerente

**Descrição:**
Lista todos os produtos cujo estoque total está abaixo ou igual ao `min_stock` definido individualmente para cada produto. O aviso é apenas informativo — não impede nenhuma operação no sistema.

**Comportamento:**
- Percorre todos os produtos que possuem `min_stock > 0`
- Compara o estoque total (soma de todos os lotes) com o `min_stock`
- Retorna apenas os produtos em situação de alerta

**Informações exibidas por produto:**
- Nome do produto
- Estoque atual
- Estoque mínimo definido
- Mensagem de aviso

**Exemplo de resposta:**
```json
{
  "success": true,
  "total_avisos": 2,
  "data": [
    {
      "product_id": 3,
      "product_name": "Pão de Hambúrguer",
      "current_stock": 12,
      "min_stock": 15,
      "mensagem": "⚠ Aviso: estoque baixo. O produto \"Pão de Hambúrguer\" está próximo de acabar. Quantidade atual: 12 unidades."
    }
  ]
}
```

**Integração com saída de estoque:**
Além do endpoint dedicado, o aviso também é retornado automaticamente na resposta de `POST /products/stock/exit` quando a retirada faz o estoque cair abaixo do mínimo (ver Tela 15).

---

## 🚨 Alertas na Saída de Estoque

A tela de retirada (`POST /products/stock/exit`) pode retornar dois tipos de avisos informativos (não bloqueantes):

| Aviso | Condição | Campo na resposta |
|---|---|---|
| Estoque baixo | Estoque restante ≤ `min_stock` | `aviso_estoque_baixo` |
| Consumo elevado | Quantidade retirada > `min_stock × multiplicador do dia` | `aviso_consumo_elevado` |

---

## 🔀 Fluxo de Navegação por Perfil

### Admin
```
POST /login → Dashboard
  ├── POST /register              (Cadastrar Funcionário)
  ├── GET /users                  (Listar Usuários)
  │     ├── GET /users/{id}       (Detalhes)
  │     ├── PUT /users/update/{id}(Editar)
  │     ├── DELETE /users/delete/{id} (Excluir)
  │     └── POST /users/promote   (Promover)
  ├── GET /products/stock/{name}  (Consultar Estoque)
  ├── GET /reports/stock          (Relatório de Estoque)
  ├── GET /reports/critical       (Relatório Crítico)
  ├── GET /reports/alerts         (Alertas de Consumo)
  └── GET /reports/low-stock      (Avisos de Estoque Baixo)
```

### Gerente
```
POST /login → Dashboard
  ├── POST /products/create           (Cadastrar Produto)
  ├── GET /products                   (Listar Produtos)
  │     ├── GET /products/{id}        (Detalhes)
  │     ├── PUT /products/update/{id} (Editar)
  │     └── DELETE /products/delete/{id} (Excluir)
  ├── POST /products/stock/entry      (Entrada de Estoque)
  ├── POST /products/stock/exit       (Retirada de Estoque)
  ├── GET /products/stock/{name}      (Consultar Estoque)
  ├── GET /movements                  (Histórico de Movimentações)
  ├── GET /movements/product/{id}     (Movimentações por Produto)
  ├── GET /reports/stock              (Relatório de Estoque)
  ├── GET /reports/critical           (Relatório Crítico)
  ├── GET /reports/alerts             (Alertas de Consumo)
  └── GET /reports/low-stock          (Avisos de Estoque Baixo)
```

### Funcionário
```
POST /login → Dashboard
  ├── POST /products/stock/exit       (Retirada de Estoque)
  └── GET /products/stock/{name}      (Consultar Estoque)
```

---

## 🎨 Observações de UX

- Telas não autorizadas devem retornar erro 403 (Forbidden) ou redirecionar para o Dashboard do perfil
- Ações destrutivas (excluir) devem exigir confirmação via modal
- Campos com erro de validação devem ser destacados em vermelho com mensagem descritiva
- Produtos com estoque crítico devem ter indicador visual (badge/ícone) no dashboard
