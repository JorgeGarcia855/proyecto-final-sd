-- Add migration script here
CREATE table ventas (
	codigo bigint primary key,
	cedula_cliente bigint not null,
	cedula_usuario bigint not null,
	iva_venta double precision not null,	
	total_venta double precision not null,
	valor_venta double precision not null,
	foreign key(cedula_cliente) references clientes(cedula),
	foreign key(cedula_usuario) references usuarios(cedula)
);

