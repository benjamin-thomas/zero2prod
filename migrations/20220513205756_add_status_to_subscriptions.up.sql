ALTER TABLE subscriptions
    ADD COLUMN status VARCHAR(255);

BEGIN;

-- noinspection SqlWithoutWhere
UPDATE subscriptions
SET status = 'confirmed';

ALTER TABLE subscriptions
    ALTER COLUMN status SET NOT NULL;

COMMIT;