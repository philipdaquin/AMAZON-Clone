-- This file should undo anything in `up.sql`
ALTER TABLE prices RENAME COLUMN cost TO price;

DROP TABLE prices;
DROP TABLE prices_products;