/*CREATE database gestao

use gestao


CREATE table produto (
id int auto_increment primary key,
nome varchar(200),
valorcusto decimal (10,2) not null,
valorvenda decimal (10,2),
estoqueatual int default 0,
pesogramas int,
status enum ('1' ,'2'),
dataproducao date not null,
datavalidade date not null
)

Create table cargo (
id int auto_increment primary key,
nome varchar (70) not null)


Create table usuario (
id int auto_increment primary key,
nome varchar (70) not null,
sobrenome varchar (150) not null,
cpf char (11) not null,
datanascimento date not null,
user varchar(70),
senha varchar(70),
fkidcargo int,
foreign key (fkidcargo) references cargo (id)
)

INSERT INTO cargo (nome)
values 
('Admin'),
('Funcionario'),
('Gerente');

insert into usuario (nome, sobrenome, cpf, datanascimento, user, senha, fkidcargo) 
values 
('Rafael', 'Matos Celestino', '000000000000', '2006-06-08', 'rmcelestino', '12345', 3)
*/

select cargo.nome as 'nome do cargo', 
usuario.* 
from usuario
left join cargo on cargo.id = usuario.fkidcargo