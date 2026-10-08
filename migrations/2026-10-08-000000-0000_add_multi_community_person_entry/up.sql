-- zhifou.io Lemmy fork: local people as multi-community entries. Their posts
-- are shown in the multi-community's feed. Not federated.
CREATE TABLE multi_community_person_entry (
    multi_community_id int NOT NULL REFERENCES multi_community ON UPDATE CASCADE ON DELETE CASCADE,
    person_id int NOT NULL REFERENCES person ON UPDATE CASCADE ON DELETE CASCADE,
    published_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (multi_community_id, person_id)
);

CREATE INDEX idx_multi_community_person_entry_person_id ON multi_community_person_entry (person_id);

