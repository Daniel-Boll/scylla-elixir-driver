<h1 align="center">ScyllaDB Elixir Driver</h1>

---

## TODO: docs

### Hello World

```bash
iex -S mix
```

```elixir
"127.0.0.1:9042"
  |> ScyllaElixirDriver.start_link
  |> ScyllaElixirDriver.execute("SELECT table_name FROM system_schema.scylla_tables limit 1")
```
