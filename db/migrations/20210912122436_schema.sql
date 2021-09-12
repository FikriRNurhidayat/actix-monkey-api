-- add monkeys table
CREATE TABLE IF NOT EXISTS "monkeys" (
  "id" uuid PRIMARY KEY,
  "username" varchar(30) NOT NULL,
  "encrypted_password" text NOT NULL,
  "created_at" timestamp NOT NULL,
  "updated_at" timestamp NOT NULL
);

-- add monkey_locations table
CREATE TABLE IF NOT EXISTS "monkey_locations" (
  "monkey_id" uuid,
  "longitude" float,
  "latitude" float,
  "geolocation" geography(Point, 4326)
);

-- add monkeys one to one relationship with monkey_locations
ALTER TABLE "monkey_locations" ADD FOREIGN KEY ("monkey_id") REFERENCES "monkeys" ("id");
