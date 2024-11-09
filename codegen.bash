#!/bin/bash

BLACK="\e[30m"
RED='\e[31m'
GREEN='\e[32m'
YELLOW='\e[33m'
BLUE='\e[34m'
PURPLE="\e[35m"
CYAN="\e[36m"
WHITE="\e[37m"
RESET='\e[0m'

# check args
if [ "$#" -ne 2 ]; then
  echo -e "${RED}Usage: $0 <path_to_schema.rs> <output_directory>${RESET}"
  exit 1
fi

schema_file_path=$1
output_dir=$2

# check schema file path
if [ ! -f "$schema_file_path" ] || [ ! -r "$schema_file_path" ]; then
  echo -e "${RED}Error: File $schema_file_path does not exist or is not readable.${RESET}"
fi

schema_file=$(<"$schema_file_path")

# test_db -> TestDb
to_camel_case() {
  local input=$1
  local result=$(echo "$input" | sed -e 's/_\([a-z]\)/\U\1/g' -e 's/^\([a-z]\)/\U\1/' -e 's/_//g')
  echo "$result"
}

# TestDb -> test_db
to_snake_case() {
  local input=$1
  local result=$(echo "$input" | sed -e 's/\([A-Z]\)/_\L\1/g' -e 's/^_//')
  echo "$result"
}

# create table model file
generate_model_code() {
  local table_name=$1
  local fields=$2
  local table_snake_name=$(to_snake_case "$table_name")
  local output_file="$output_dir/${table_snake_name}/${table_snake_name}_model.rs"
  local model_name="Db$(to_camel_case "$table_name")Model"

  local field_cache=()
  for field in "${current_fields[@]}"; do
    IFS="," read -r field_name field_type <<< "$field"

    opt_field_type=$(echo "$field_type" | sed -n 's/.*Nullable<\(.*\)>.*/\1/p')
    is_opt_field_type=0
    if [ ! -z "$opt_field_type" ]; then
      field_type="$opt_field_type"
      is_opt_field_type=1
    fi

    case "$field_type" in
      Integer) rust_type="i32";;
      BigInt) rust_type="i64";;
      Float) rust_type="f64";;
      Text) rust_type="String";;
      Binary) rust_type="Vec<u8>";;
      Bool) rust_type="bool";;
      Date) rust_type="chrono::NaiveDate";;
      Time) rust_type="chrono::NaiveTime";;
      Timestamp) rust_type="chrono::NaiveDateTime";;
      # add other types
      *) echo -e "${RED}==>Error: Field type $field_type not process!";;
    esac

    case "$field_type" in
      Integer) rust_ref_type="i32";;
      BigInt) rust_ref_type="i64";;
      Float) rust_ref_type="f64";;
      Text) rust_ref_type="&'a str";;
      Binary) rust_ref_type="&'a Vec<u8>";;
      Bool) rust_ref_type="bool";;
      Date) rust_ref_type="&'a chrono::NaiveDate";;
      Time) rust_ref_type="&'a chrono::NaiveTime";;
      Timestamp) rust_ref_type="&'a chrono::NaiveDateTime";;
      # add other types
      *) echo -e "${RED}==>Error: Field ref type $field_type not process!";;
    esac

    field_cache+=("$field_name,$is_opt_field_type,$rust_type,Option<$rust_type>,$rust_ref_type,Option<$rust_ref_type>")
  done


  local rust_code="//! ${model_name}\n\n"
  rust_code+="use diesel::prelude::*;\n\n"

  # table model
  rust_code+="#[derive(Queryable, Selectable)]\n"
  rust_code+="#[diesel(table_name = crate::schema::$table_name)]\n"
  rust_code+="#[diesel(check_for_backend(diesel::sqlite::Sqlite))]\n"
  rust_code+="pub struct ${model_name} {\n"
  for field in "${field_cache[@]}"; do
    IFS="," read -r field_name is_opt_field_type rust_type opt_rust_type rust_ref_type opt_rust_ref_type <<< "$field"
    if [ $is_opt_field_type -eq 1 ]; then
      rust_code+="\tpub $field_name: $opt_rust_type,\n"
    else
      rust_code+="\tpub $field_name: $rust_type, \n"
    fi
  done
  rust_code+="}\n\n"

  # new table model
  rust_code+="#[derive(Insertable)]\n"
  rust_code+="#[diesel(table_name = crate::schema::$table_name)]\n"
  rust_code+="pub struct New$model_name<'a> {\n"
  for field in "${field_cache[@]}"; do
    IFS="," read -r field_name is_opt_field_type rust_type opt_rust_type rust_ref_type opt_rust_ref_type <<< "$field"
    if [ "$field_name" == "id" ]; then
      continue
    fi
    if [ $is_opt_field_type -eq 1 ]; then
      rust_code+="\tpub $field_name: $opt_rust_ref_type,\n"
    else
      rust_code+="\tpub $field_name: $rust_ref_type,\n"
    fi
  done
  rust_code+="}\n\n"

  # update model
  rust_code+="#[derive(AsChangeset)]\n"
  rust_code+="#[diesel(table_name = crate::schema::$table_name)]\n"
  rust_code+="pub struct Update$model_name<'a> {\n"
  for field in "${field_cache[@]}"; do
    IFS="," read -r field_name is_opt_field_type rust_type opt_rust_type rust_ref_type opt_rust_ref_type <<< "$field"
    if [ "$field_name" == "id" ]; then
      continue
    fi
    rust_code+="\tpub $field_name: $opt_rust_ref_type,\n"
  done
  rust_code+="}\n\n"

  mkdir -p "$(dirname "$output_file")"
  echo -e "$rust_code" > "$output_file"
  echo -e "${GREEN}==> code generate model: $output_file${RESET}"
}

# create table service file
generate_service_code() {
  local table_name=$1
  local table_snake_name=$(to_snake_case "$table_name")
  local model_name="Db$(to_camel_case "$table_name")Model"
  local output_file="$output_dir/${table_snake_name}/${table_snake_name}_service.rs"
  
  if [ -f "$output_file" ]; then
    echo -e "${YELLOW}==> service file $output_file is exists, skip${RESET}"
    return
  fi

  local service_name="Db$(to_camel_case "${table_name}")Service"

  local rust_code="//! ${service_name}\n\n"
  rust_code+="use super::{ ${model_name}, New${model_name}, Update${model_name} };\n"
  rust_code+="use crate::schema::{ self, ${table_name}::dsl::* };\n"
  rust_code+="use diesel::prelude::*;\n\n"
  rust_code+="pub struct ${service_name};\n\n"

  rust_code+="impl ${service_name} {\n"
  rust_code+="\tpub fn list_$table_name(conn: &mut SqliteConnection) -> Vec<$model_name> {\n"
  rust_code+="\t\tmatch $table_name.load::<$model_name>(conn) {\n"
  rust_code+="\t\t\tOk(list) => return list,\n"
  rust_code+="\t\t\tErr(e) => {\n"
  rust_code+="\t\t\t\tprintln!(\"Error loading $table_name: {}\", e);\n"
  rust_code+="\t\t}\n"
  rust_code+="\t\t}\n"
  rust_code+="\n"
  rust_code+="\t\t\tVec::new()\n"
  rust_code+="\t}\n"
  rust_code+="\n"
  rust_code+="\tpub fn add_$table_name(conn: &mut SqliteConnection, new_model: &New$model_name) -> $model_name {\n"
  rust_code+="\t\tdiesel::insert_into(schema::$table_name::table)\n"
  rust_code+="\t\t\t.values(new_model)\n"
  rust_code+="\t\t\t.returning($model_name::as_returning())\n"
  rust_code+="\t\t\t.get_result(conn)\n"
  rust_code+="\t\t\t.expect(\"Error saving new $table_name\")\n"
  rust_code+="\t}\n"
  rust_code+="\n"
  rust_code+="\tpub fn find_${table_name}_by_id(conn: &mut SqliteConnection, model_id: i32) -> Option<$model_name> {\n"
  rust_code+="\t\t$table_name\n"
  rust_code+="\t\t.find(model_id)\n"
  rust_code+="\t\t.first(conn)\n"
  rust_code+="\t\t.optional()\n"
  rust_code+="\t\t.expect(\"Error loading $table_name\")\n"
  rust_code+="\t}\n"
  rust_code+="\n"
  rust_code+="\tpub fn update_${table_name}_by_id(\n"
  rust_code+="\t\tconn: &mut SqliteConnection,\n"
  rust_code+="\t\tmodel_id: i32,\n"
  rust_code+="\t\tupdate_model: &Update$model_name,\n"
  rust_code+="\t) {\n"
  rust_code+="\t\tdiesel::update($table_name.find(model_id))\n"
  rust_code+="\t\t\t.set(update_model)\n"
  rust_code+="\t\t\t.execute(conn)\n"
  rust_code+="\t\t\t.expect(\"Error updating $table_name\");\n"
  rust_code+="\t}\n"
  rust_code+="\n"
  rust_code+="\tpub fn delete_${table_name}_by_id(conn: &mut SqliteConnection, model_id: i32) {\n"
  rust_code+="\t\tdiesel::delete($table_name.find(model_id))\n"
  rust_code+="\t\t\t.execute(conn)\n"
  rust_code+="\t\t\t.expect(\"Error deleting $table_name\");\n"
  rust_code+="\t}\n"

  rust_code+="}\n"

  mkdir -p "$(dirname "$output_file")"
  echo -e "$rust_code" > "$output_file"
  echo -e "${GREEN}==> code generate service: $output_file${RESET}"
}

# create table directory mod file
generate_mod_code() {
  local table_name=$1
  local table_snake_name=$(to_snake_case "$table_name")
  local output_file="$output_dir/${table_snake_name}/mod.rs"
  local model_file_name="${table_snake_name}_model"
  local service_file_name="${table_snake_name}_service"

  local rust_code="//! ${table_name}\n\n"
  rust_code+="mod $model_file_name;\n"
  rust_code+="mod $service_file_name;\n\n"
  rust_code+="pub use $model_file_name::*;\n"
  rust_code+="pub use $service_file_name::*;\n"

  mkdir -p "$(dirname "$output_file")"
  echo -e "$rust_code" > "$output_file"
  echo -e "${GREEN}==> code generate model mod: $output_file${RESET}"
}

# create db directory mod file
generate_db_mod_code() {
  local output_file="$output_dir/mod.rs"

  local rust_code="//! modelbase\n\n"

  for table in "${tables[@]}"; do
    rust_code+="mod ${table};\n"
  done
  
  rust_code+="\n"

  for table in "${tables[@]}"; do
    rust_code+="pub use ${table}::*;\n"
  done

  echo -e "$rust_code" > "$output_file"
  echo -e "${GREEN}==> code generate database mod: $output_file${RESET}"
}

tables=()
current_table_name=""
current_fields=()
reading_fields=false

while IFS= read -r line; do
  tmp_table_name=$(echo "$line" | sed -n 's/^\s*\(\w\+\)\s*(.*$/\1/p')
  if [[ ! -z "$tmp_table_name" ]]; then
    current_table_name=$tmp_table_name
    tables+=("$current_table_name")
    current_fields=()
    reading_fields=true
  elif [[ "$reading_fields" = true && $(echo "$line" | tr -d " ") =~ ^\} ]]; then
    echo "==> code generate: $current_table_name"
    reading_fields=false
    generate_model_code "$current_table_name" "$current_fields" "$output_dir"
    generate_service_code "$current_table_name" "$output_dir"
    generate_mod_code "$current_table_name" "$output_dir"
  elif [[ "$reading_fields" = true ]]; then
    field=$(echo "$line" | sed 's/^\s*//; s/\s*->\s*/,/; s/,$//')
    current_fields+=("$field")
  fi
done <<< "$schema_file"

generate_db_mod_code

echo -e "${GREEN}==> code generate complete!${RESET}"
