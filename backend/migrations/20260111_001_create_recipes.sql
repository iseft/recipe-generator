CREATE TABLE recipes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    ingredients TEXT[] NOT NULL,
    instructions TEXT[] NOT NULL,
    prep_time_minutes INTEGER,
    cook_time_minutes INTEGER,
    servings INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
