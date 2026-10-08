-- zhifou.io Lemmy fork: record who created a local community. Existing and
-- remote communities stay NULL, since their creator isn't known.
ALTER TABLE community
    ADD COLUMN creator_id int REFERENCES person ON UPDATE CASCADE ON DELETE SET NULL;

CREATE INDEX idx_community_creator_id ON community (creator_id);

