CREATE table clientes (
	cedula bigint primary key,
	direccion varchar(40) not null,	
	email varchar(40) not null,
	nombre varchar(40) not null,
	telefono varchar(40) not null
);

CREATE table usuarios (
	cedula bigint primary key,
	email varchar(40) not null,	
	nombre varchar(40) not null,
	password varchar(40) not null,
	usuario varchar(40) not null
);

CREATE table proveedores (
	nit bigint primary key,
	ciudad varchar(40) not null,	
	direccion varchar(40) not null,
	nombre varchar(40) not null,
	telefono varchar(40) not null
);

CREATE table productos (
	codigo bigint primary key,
	nit_proveedor bigint not null,
	iva_compra decimal not null,	
	nombre_producto varchar(40) not null,
	precio_compra decimal not null,
	precio_venta decimal not null,
	foreign key(nit_proveedor) references proveedores(nit)
);

CREATE table ventas (
	codigo bigint primary key,
	cedula_cliente bigint not null,
	cedula_usuario bigint not null,
	iva_venta decimal not null,	
	total_venta decimal not null,
	valor_venta decimal not null,
	foreign key(cedula_cliente) references clientes(cedula),
	foreign key(cedula_usuario) references usuarios(cedula)
);

CREATE table detalle_ventas (
	codigo bigint primary key,
	codigo_producto bigint not null,
	codigo_venta bigint not null,
	cantidad_producto int not null,	
	valor_total decimal not null,
	valor_venta decimal not null,
	valor_iva decimal not null,
	foreign key(codigo_producto) references productos(codigo),
	foreign key(codigo_venta) references ventas(codigo)
);