-- Add migration script here
CREATE table clientes (
	cedula bigint primary key,
	direccion varchar(40) not null,	
	email varchar(40) not null,
	nombre varchar(40) not null,
	telefono varchar(40) not null
);