-- Add migration script here
CREATE table productos (
	codigo bigint primary key,
	nit_proveedor bigint not null,
	iva_compra double precision not null,	
	nombre_producto varchar(40) not null,
	precio_compra double precision not null,
	precio_venta double precision not null,
	foreign key(nit_proveedor) references proveedores(nit)
);