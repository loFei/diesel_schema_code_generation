#!/bin/bash

diesel migration run

bash ../codegen.bash ./src/schema.rs ./src/db
