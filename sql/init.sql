CREATE TABLE "cities"
(
    "id"              SERIAL  NOT NULL,
    "department_code" VARCHAR NOT NULL,
    "insee_code"      VARCHAR, -- if empty, put NULL
    "zip_code"        VARCHAR, -- if empty, put NULL
    "name"            VARCHAR NOT NULL,
    "lat"             FLOAT   NOT NULL,
    "lon"             FLOAT   NOT NULL,

    CONSTRAINT "cities_pkey" PRIMARY KEY ("id")
);
