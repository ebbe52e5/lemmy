-- Local-only "follow a person". Rows are never deleted on unfollow: `active`
-- marks the current state so past follow relationships are kept.
CREATE TABLE person_follow (
    id serial PRIMARY KEY,
    person_id int NOT NULL REFERENCES person (id) ON UPDATE CASCADE ON DELETE CASCADE,
    target_id int NOT NULL REFERENCES person (id) ON UPDATE CASCADE ON DELETE CASCADE,
    active boolean NOT NULL DEFAULT TRUE,
    published_at timestamptz NOT NULL DEFAULT now(),
    followed_at timestamptz NOT NULL DEFAULT now(),
    unfollowed_at timestamptz,
    UNIQUE (person_id, target_id),
    CHECK (person_id <> target_id)
);

CREATE INDEX idx_person_follow_target_active ON person_follow (target_id)
WHERE
    active;

ALTER TABLE person
    ADD COLUMN follower_count int NOT NULL DEFAULT 0;

