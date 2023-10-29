CREATE table clientes (
	cedula bigint(20) primary key,
	direccion varchar(40) not null,	
	email varchar(40) not null,
	nombre varchar(40) not null,
	telefono varchar(40) not null
);

CREATE table usuarios (
	cedula bigint(20) primary key,
	email varchar(40) not null,	
	nombre varchar(40) not null,
	password varchar(40) not null,
	usuario varchar(40) not null
);

CREATE table proveedores (
	nit bigint(20) primary key,
	ciudad varchar(40) not null,	
	direccion varchar(40) not null,
	nombre varchar(40) not null,
	telefono varchar(40) not null
);

CREATE table productos (
	codigo bigint(20) primary key,
	nit_proveedor bigint(20) not null,
	iva_compra double not null,	
	nombre_producto varchar(40) not null,
	precio_compra double not null,
	precio_venta double not null,
	foreign key(nit_proveedor) references proveedores(nit)
);

CREATE table ventas (
	codigo bigint(20) primary key,
	cedula_cliente bigint(20) not null,
	cedula_usuario bigint(20) not null,
	iva_venta double not null,	
	total_venta double not null,
	valor_venta double not null,
	foreign key(cedula_cliente) references clientes(cedula),
	foreign key(cedula_usuario) references usuarios(cedula)
);

CREATE table detalle_ventas (
	codigo bigint(20) primary key,
	codigo_producto bigint(20) not null,
	codigo_venta bigint(20) not null,
	cantidad_producto int(11) not null,	
	valor_total double not null,
	valor_venta double not null,
	valor_iva double not null,
	foreign key(codigo_producto) references productos(codigo),
	foreign key(codigo_venta) references ventas(codigo)
);


