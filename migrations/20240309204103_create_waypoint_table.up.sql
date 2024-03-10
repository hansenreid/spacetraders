CREATE TABLE IF NOT EXISTS waypoints (
    location TEXT PRIMARY KEY NOT NULL,
    waypoint_type TEXT NOT NULL,
    traits TEXT NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL
);
