defmodule ScyllaElixirDriverTest do
  use ExUnit.Case
  doctest ScyllaElixirDriver

  test "greets the world" do
    assert ScyllaElixirDriver.hello() == :world
  end
end
