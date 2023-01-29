CREATE TYPE order_status_enum AS ENUM(
    'open',
    'completed',
    'blocked'
);

CREATE TYPE task_status_enum AS ENUM(
    'waiting',
    'completed',
    'blocked',
    'problem'
);

CREATE TABLE IF NOT EXISTS orders (
    id bigint GENERATED ALWAYS AS IDENTITY,
    title VARCHAR(50) NOT NULL,
    status order_status_enum NOT NULL DEFAULT 'open',
    PRIMARY KEY(id) 
);

CREATE TABLE IF NOT EXISTS tasks (
    id bigint GENERATED ALWAYS AS IDENTITY,
    order_id bigint NOT NULL,
    title VARCHAR(50) NOT NULL,
    status task_status_enum NOT NULL DEFAULT 'waiting',
    PRIMARY KEY(id),
    CONSTRAINT fk_order
        FOREIGN KEY(order_id) REFERENCES orders(id)
);