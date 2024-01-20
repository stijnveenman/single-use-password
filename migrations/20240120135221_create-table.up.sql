-- Add up migration script here


 CREATE TABLE "public"."passwords" (
    "id" uuid NOT NULL,
    "key" varchar NOT NULL,
    "password" varchar NOT NULL,
    PRIMARY KEY ("id")
);

