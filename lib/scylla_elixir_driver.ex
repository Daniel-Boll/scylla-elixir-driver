defmodule ScyllaElixirDriver do
  use Rustler, otp_app: :scylla_elixir_driver, crate: "scylla_bindings"

  # When your NIF is loaded, it will override this function.
  def start_link(_uri), do: :erlang.nif_error(:nif_not_loaded)
  def execute(_session, _query), do: :erlang.nif_error(:nif_not_loaded)
end
