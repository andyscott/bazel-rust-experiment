workspace(name = "bazelbub")

git_repository(
    name = "io_bazel_rules_scala",
    remote = "git://github.com/bazelbuild/rules_scala",
    commit = "375b34af68915ab16b69edf66a472b1be96a1381"
)

load("@io_bazel_rules_scala//scala:scala.bzl", "scala_repositories")
scala_repositories()
register_toolchains("//tools:scala_toolchain")

load("//3rdparty:workspace.bzl", "maven_dependencies")
maven_dependencies()
