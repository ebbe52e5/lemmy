CREATE TYPE listing_type_enum_tmp AS ENUM (
    'All',
    'Local',
    'Subscribed',
    'ModeratorView',
    'Suggested'
);

UPDATE
    local_user
SET
    default_listing_type = 'Local'
WHERE
    default_listing_type = 'Following';

UPDATE
    local_site
SET
    default_post_listing_type = 'Local'
WHERE
    default_post_listing_type = 'Following';

ALTER TABLE local_user
    ALTER COLUMN default_listing_type DROP DEFAULT,
    ALTER COLUMN default_listing_type TYPE listing_type_enum_tmp
    USING (default_listing_type::text::listing_type_enum_tmp),
    ALTER COLUMN default_listing_type SET DEFAULT 'Local';

ALTER TABLE local_site
    ALTER COLUMN default_post_listing_type DROP DEFAULT,
    ALTER COLUMN default_post_listing_type TYPE listing_type_enum_tmp
    USING (default_post_listing_type::text::listing_type_enum_tmp),
    ALTER COLUMN default_post_listing_type SET DEFAULT 'Local';

DROP TYPE listing_type_enum;

ALTER TYPE listing_type_enum_tmp RENAME TO listing_type_enum;

