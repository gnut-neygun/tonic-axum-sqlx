CREATE TABLE IF NOT EXISTS Object
(
    id         SERIAL PRIMARY KEY,
    name       Text,
    properties JSON
);
