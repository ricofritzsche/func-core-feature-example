-- migration.sql
CREATE TABLE IF NOT EXISTS asset_status (
                                            asset_id TEXT PRIMARY KEY,
                                            last_status TEXT NOT NULL,
                                            updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

