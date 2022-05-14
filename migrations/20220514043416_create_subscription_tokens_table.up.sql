CREATE TABLE subscription_tokens
(
    token         TEXT NOT NULL PRIMARY KEY CHECK ( length(token) >= 25 ),
    subscriber_id INT  NOT NULL REFERENCES subscriptions (id) UNIQUE
);

