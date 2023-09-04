CREATE TABLE IF NOT EXISTS celestial_body (
	body_id 									INTEGER NOT NULL PRIMARY KEY,
	body_name 	 							TEXT UNIQUE NOT NULL,
	radius 				 						REAL NOT NULL,
	aphelion			 						REAL NOT NULL,
  perihelion		 						REAL NOT NULL,
  orbital_period						REAL NOT NULL,
	region										INTEGER,
	subregion									INTEGER,
	created_at								INTEGER NOT NULL,
	updated_at								INTEGER,
	FOREIGN KEY (region) 			REFERENCES celestial_region(region_id),
	FOREIGN KEY (subregion) 	REFERENCES celestial_subregion(subregion_id)
);

CREATE TABLE IF NOT EXISTS celestial_region (
	region_id 								INTEGER NOT NULL PRIMARY KEY,
	region_name 							TEXT UNIQUE NOT NULL,
	region_description				TEXT,
	created_at								INTEGER NOT NULL,
	updated_at								INTEGER
);

CREATE TABLE IF NOT EXISTS celestial_subregion (
	subregion_id 							INTEGER NOT NULL PRIMARY KEY,
	subregion_name 						TEXT UNIQUE NOT NULL,
	subregion_description			TEXT,
	created_at								INTEGER NOT NULL,
	updated_at								INTEGER
);

CREATE TABLE IF NOT EXISTS orbital_parent (
	child 										INTEGER NOT NULL,
	parent 										INTEGER NOT NULL ,
	FOREIGN KEY (child) 			REFERENCES celestial_body(body_id),
	FOREIGN KEY (parent) 			REFERENCES celestial_body(body_id)
);
