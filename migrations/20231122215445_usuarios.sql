-- Add migration script here
CREATE table usuarios (
	cedula bigint primary key,
	email varchar(40) not null,	
	nombre varchar(40) not null,
	password varchar(40) not null,
	usuario varchar(40) not null
);
