load("@io_bazel_rules_scala//scala:scala_toolchain.bzl",
    upstream_scala_toolchain = "scala_toolchain")

load(":scala.bzl", "scalacopts_strict")

upstream_scala_toolchain(
    name = "scala_toolchain_impl",
    scalacopts = scalacopts_strict,
    visibility = ["//visibility:public"]
)

toolchain(
    name = "scala_toolchain",
    toolchain_type = "@io_bazel_rules_scala//scala:toolchain_type",
    toolchain = "scala_toolchain_impl",
    visibility = ["//visibility:public"]
)
