# ScyllaElixirDriver

**TODO: Add description**

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `scylla_elixir_driver` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:scylla_elixir_driver, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/scylla_elixir_driver>.

### Hello World

```bash
iex -S mix
```

```elixir
"127.0.0.1:9042"
  |> ScyllaElixirDriver.start_link
  |> ScyllaElixirDriver.execute("SELECT table_name FROM system_schema.scylla_tables limit 1")
```
