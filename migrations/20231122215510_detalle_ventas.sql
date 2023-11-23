-- Add migration script here
CREATE table detalle_ventas (
	codigo bigint primary key,
	codigo_producto bigint not null,
	codigo_venta bigint not null,
	cantidad_producto int not null,	
	valor_total double precision not null,
	valor_venta double precision not null,
	valor_iva double precision not null,
	foreign key(codigo_producto) references productos(codigo),
	foreign key(codigo_venta) references ventas(codigo)
);