-- zhifou.io Lemmy fork: record when a multi-community was followed. Existing
-- follows stay NULL, since that time isn't known.
ALTER TABLE multi_community_follow
    ADD COLUMN followed_at timestamptz;

ALTER TABLE multi_community_follow
    ALTER COLUMN followed_at SET DEFAULT now();

