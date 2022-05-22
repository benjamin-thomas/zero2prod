CREATE TYPE QUEUE_STATUS AS ENUM ('Queued', 'Running', 'Failed');

CREATE TABLE queue
    ( id BIGINT NOT NULL GENERATED ALWAYS AS IDENTITY
    , status QUEUE_STATUS NOT NULL DEFAULT 'Queued'
    , message JSONB NOT NULL
    , failed_attempts INT NOT NULL DEFAULT 0
    , scheduled_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'UTC')
    , priority SMALLINT NOT NULL DEFAULT 0
    , created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'UTC')
    , updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'UTC')
    );

COMMENT ON COLUMN queue.priority IS 'The highest number represents the highest priority';
