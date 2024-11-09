#!/bin/bash

diesel migration run

bash ../codegen.sh ./src/schema.rs ./src/db
