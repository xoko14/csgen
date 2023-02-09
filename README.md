# csgen

A collection of tools useful for ASP.NET/C#/SQL development.

This repository features both a Rust/C library and a executable consuming the library.

## Executable (csgen)

Run `csgen help` to see all avaliable commands.

## Library (libcsgen)

If used in rust, all the functions are avaliable inside `libcsgen::operations::<operation_type>`.

If used in any other programming language compatible with C libraries, the following functions are exposed:

- ```c
  char* gen_service(char *service_name)
  ```

- ```c
  char* gen_repository(char *repository_name, char *dbo_name)
  ```

- ```c
  char* gen_sql_insert_copy(char *table_csv, bool is_identity)
  ```

- ```c
  char* gen_sql_insert_empty(char *table_csv, bool is_identity)
  ```

- ```c
  char* gen_db_enum_code(char *enum_name, char *table_csv, bool skip_col, bool extra_docs)
  ```

## TODO

- [ ] Add rust documentation.
- [ ] Add header file with documentation.
