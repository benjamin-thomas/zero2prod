CREATE TABLE subscriptions
(
    id    INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    NAME VARCHAR(255) NOT NULL,
    subscribed_at TIMESTAMPTZ NOT NULL
)