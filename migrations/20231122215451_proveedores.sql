-- Add migration script here
CREATE table proveedores (
	nit bigint primary key,
	ciudad varchar(40) not null,	
	direccion varchar(40) not null,
	nombre varchar(40) not null,
	telefono varchar(40) not null
);