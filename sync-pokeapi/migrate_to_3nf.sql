-- Migration script to convert usage_stats from denormalized (JSONB) to 3NF
-- This script should be run AFTER creating the new tables with migrate()
-- WARNING: New design stores only single format/period, old data will be overwritten if same form_id exists

-- Step 1: Create backup of old data
CREATE TABLE IF NOT EXISTS usage_stats_backup AS SELECT * FROM usage_stats;

-- Step 2: Create temporary table with new structure (form_id as primary key)
CREATE TEMP TABLE usage_stats_new (
    form_id INTEGER PRIMARY KEY,
    format VARCHAR(50) NOT NULL,
    period VARCHAR(10) NOT NULL,
    raw_count INTEGER NOT NULL,
    usage DOUBLE PRECISION NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Step 3: Migrate main usage_stats data (if old schema exists)
DO $$
BEGIN
    -- Check if old columns exist
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'usage_stats'
        AND column_name = 'abilities'
    ) THEN
        -- Old schema exists, migrate data
        -- Note: If multiple records exist for the same form_id, only the first one will be kept
        INSERT INTO usage_stats_new (form_id, format, period, raw_count, usage, updated_at)
        SELECT DISTINCT ON (form_id) form_id, format, period, raw_count, usage, updated_at
        FROM usage_stats
        ORDER BY form_id, updated_at DESC
        ON CONFLICT (form_id) DO NOTHING;

        -- Migrate abilities data
        INSERT INTO usage_abilities (form_id, ability_name, percentage)
        SELECT DISTINCT ON (us.form_id, ability.key)
            us.form_id,
            ability.key,
            (ability.value)::text::double precision
        FROM usage_stats us,
        LATERAL jsonb_each(us.abilities) AS ability(key, value)
        ORDER BY us.form_id, ability.key, us.updated_at DESC
        ON CONFLICT (form_id, ability_name) DO NOTHING;

        -- Migrate items data
        INSERT INTO usage_items (form_id, item_name, percentage)
        SELECT DISTINCT ON (us.form_id, item.key)
            us.form_id,
            item.key,
            (item.value)::text::double precision
        FROM usage_stats us,
        LATERAL jsonb_each(us.items) AS item(key, value)
        ORDER BY us.form_id, item.key, us.updated_at DESC
        ON CONFLICT (form_id, item_name) DO NOTHING;

        -- Migrate moves data
        INSERT INTO usage_moves (form_id, move_name, percentage)
        SELECT DISTINCT ON (us.form_id, move.key)
            us.form_id,
            move.key,
            (move.value)::text::double precision
        FROM usage_stats us,
        LATERAL jsonb_each(us.moves) AS move(key, value)
        ORDER BY us.form_id, move.key, us.updated_at DESC
        ON CONFLICT (form_id, move_name) DO NOTHING;

        -- Migrate spreads data
        INSERT INTO usage_spreads (form_id, spread, percentage)
        SELECT DISTINCT ON (us.form_id, spread.key)
            us.form_id,
            spread.key,
            (spread.value)::text::double precision
        FROM usage_stats us,
        LATERAL jsonb_each(us.spreads) AS spread(key, value)
        ORDER BY us.form_id, spread.key, us.updated_at DESC
        ON CONFLICT (form_id, spread) DO NOTHING;

        -- Migrate tera_types data
        INSERT INTO usage_tera_types (form_id, tera_type, percentage)
        SELECT DISTINCT ON (us.form_id, tera_type.key)
            us.form_id,
            tera_type.key,
            (tera_type.value)::text::double precision
        FROM usage_stats us,
        LATERAL jsonb_each(us.tera_types) AS tera_type(key, value)
        ORDER BY us.form_id, tera_type.key, us.updated_at DESC
        ON CONFLICT (form_id, tera_type) DO NOTHING;

        -- Drop old table and rename new one
        DROP TABLE usage_stats;
        ALTER TABLE usage_stats_new RENAME TO usage_stats;

        -- Add foreign key constraint
        ALTER TABLE usage_stats
        ADD CONSTRAINT fk_usage_stats_form
        FOREIGN KEY (form_id) REFERENCES pokemon_forms(form_id);

        -- Recreate index
        CREATE INDEX IF NOT EXISTS idx_usage_stats_form ON usage_stats(form_id);

        RAISE NOTICE 'Migration completed successfully';
    ELSE
        -- New schema already in place
        DROP TABLE usage_stats_new;
        RAISE NOTICE 'Already using 3NF schema, no migration needed';
    END IF;
END $$;

-- Step 4: Verify migration
DO $$
DECLARE
    old_count INTEGER;
    new_count INTEGER;
    abilities_count INTEGER;
    items_count INTEGER;
    moves_count INTEGER;
    spreads_count INTEGER;
    tera_types_count INTEGER;
BEGIN
    -- Count records in backup (if exists)
    SELECT COUNT(*) INTO old_count FROM usage_stats_backup;
    SELECT COUNT(*) INTO new_count FROM usage_stats;
    SELECT COUNT(*) INTO abilities_count FROM usage_abilities;
    SELECT COUNT(*) INTO items_count FROM usage_items;
    SELECT COUNT(*) INTO moves_count FROM usage_moves;
    SELECT COUNT(*) INTO spreads_count FROM usage_spreads;
    SELECT COUNT(*) INTO tera_types_count FROM usage_tera_types;

    RAISE NOTICE 'Migration Summary:';
    RAISE NOTICE '  Old usage_stats records: %', old_count;
    RAISE NOTICE '  New usage_stats records: %', new_count;
    RAISE NOTICE '  Abilities records: %', abilities_count;
    RAISE NOTICE '  Items records: %', items_count;
    RAISE NOTICE '  Moves records: %', moves_count;
    RAISE NOTICE '  Spreads records: %', spreads_count;
    RAISE NOTICE '  Tera types records: %', tera_types_count;
END $$;
