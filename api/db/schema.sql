CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users
(
    id uuid DEFAULT uuid_generate_v1() NOT NULL CONSTRAINT users_pkey PRIMARY KEY,
    email text NOT NULL,
    password text NOT NULL,
    username text NOT NULL,
    is_active BOOLEAN DEFAULT FALSE,
    is_staff BOOLEAN DEFAULT FALSE,
    is_superuser BOOLEAN DEFAULT FALSE,
    icon text NOT NULL,
    date_joined timestamp with time zone default CURRENT_TIMESTAMP
);
