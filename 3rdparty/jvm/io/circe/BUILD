load("@io_bazel_rules_scala//scala:scala_import.bzl", "scala_import")
scala_import(
    name = "circe_core",
    jars = [
        "//external:jar/io/circe/circe_core_2_11"
    ],
    runtime_deps = [
        "//3rdparty/jvm/org/scala_lang:scala_library",
        "//3rdparty/jvm/org/typelevel:cats_core",
        ":circe_numbers_2_11"
    ],
    visibility = [
        "//visibility:public"
    ]
)



scala_import(
    name = "circe_jawn",
    jars = [
        "//external:jar/io/circe/circe_jawn_2_11"
    ],
    runtime_deps = [
        "//3rdparty/jvm/org/scala_lang:scala_library",
        "//3rdparty/jvm/org/spire_math:jawn_parser_2_11",
        ":circe_core"
    ],
    visibility = [
        "//visibility:public"
    ]
)



java_library(
    name = "circe_numbers_2_11",
    exports = [
        "//external:jar/io/circe/circe_numbers_2_11"
    ],
    visibility = [
        "//visibility:public"
    ]
)


