-- DROP TYPE public."userstatus";

CREATE TYPE public."userstatus" AS ENUM (
	'Available',
	'Disabled');

-- DROP TYPE public."usertype";

CREATE TYPE public."usertype" AS ENUM (
	'Admin',
	'Normal',
	'SuperAdmin');

-- DROP TABLE public."role";

CREATE TABLE public."role" (
	id bigserial NOT NULL,
	"name" varchar(255) NOT NULL,
	description varchar(255) NULL,
	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamptz NULL,
	deleted_at timestamptz NULL,
	CONSTRAINT role_pkey PRIMARY KEY (id)
);

-- DROP TABLE public."user";

CREATE TABLE public."user" (
	id bigserial NOT NULL,
	email varchar(255) NOT NULL,
	salt varchar(255) NOT NULL,
	pwd varchar(255) NULL DEFAULT NULL::character varying,
	"name" varchar(16) NULL DEFAULT NULL::character varying,
	mobile varchar(16) NULL DEFAULT NULL::character varying,
	laston timestamptz NULL,
	avatar varchar(255) NULL DEFAULT NULL::character varying,
	created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamptz NULL,
	deleted_at timestamptz NULL,
	"type" public."usertype" NOT NULL DEFAULT 'Normal'::usertype,
	status public."userstatus" NOT NULL DEFAULT 'Available'::userstatus,
	CONSTRAINT user_pkey PRIMARY KEY (id)
);


-- DROP TABLE public.user_role_map;

CREATE TABLE public.user_role_map (
	id bigserial NOT NULL,
	user_id int8 NOT NULL,
	role_id int8 NOT NULL,
	CONSTRAINT user_role_map_pkey PRIMARY KEY (id),
	CONSTRAINT fk_user_role_map_user_id FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE,
	CONSTRAINT fk_user_role_map_role_id FOREIGN KEY (role_id) REFERENCES public."role"(id) ON DELETE CASCADE
);


-- DROP TABLE public."permission";

CREATE TABLE public."permission" (
	id bigserial NOT NULL,
	code int8 NOT NULL,
	"level" int4 NOT NULL,
	"name" varchar(255) NOT NULL,
	description varchar(255) NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	deleted_at timestamp NULL,
	CONSTRAINT permission_pkey PRIMARY KEY (id)
);


-- DROP TABLE public.permission_role_map;

CREATE TABLE public.permission_role_map (
	id bigserial NOT NULL,
	permission_code int8 NOT NULL,
	role_id int8 NOT NULL,
	CONSTRAINT permission_role_map_pkey PRIMARY KEY (id),
    CONSTRAINT fk_permission_role_map_role_id FOREIGN KEY (role_id) REFERENCES public."role"(id) ON DELETE CASCADE
);
