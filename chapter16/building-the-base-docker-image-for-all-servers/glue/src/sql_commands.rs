

pub static WIPE_DB: &str = "
DO $$ 
DECLARE
    r RECORD;
BEGIN
    -- Disable referential integrity checks
    SET session_replication_role = replica;

    -- Loop through all table names and drop them
    FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
        EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
    END LOOP;

    -- Re-enable referential integrity checks
    SET session_replication_role = DEFAULT;
END $$;
";
