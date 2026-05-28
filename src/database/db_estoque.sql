-- 1. Roles (perfis de acesso)
CREATE TABLE roles (
    id   SMALLINT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Inserir os 3 tipos padrão
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
    min_stock       INT NOT NULL DEFAULT 0  -- para alerta de estoque baixo
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
    id           INT PRIMARY KEY AUTO_INCREMENT,
    product_id   INT NOT NULL,
    batch_id     INT,
    user_id      INT NOT NULL,
    movement_type ENUM('entrada', 'saida') NOT NULL,
    quantity     INT NOT NULL,
    created_at   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    notes        VARCHAR(255),
    FOREIGN KEY (product_id) REFERENCES products(id),
    FOREIGN KEY (batch_id)   REFERENCES batches(id),
    FOREIGN KEY (user_id)    REFERENCES users(id)
);

select * from roles;
select * from users;
select * from products;
select * from batches;
select * from movements;