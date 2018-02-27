load("@io_bazel_rules_scala//scala:scala_import.bzl", "scala_import")
scala_import(
    name = "coursier",
    jars = [
        "//external:jar/io/get_coursier/coursier_2_11"
    ],
    runtime_deps = [
        "//3rdparty/jvm/org/scala_lang/modules:scala_xml_2_11",
        "//3rdparty/jvm/org/scala_lang:scala_library",
        "//3rdparty/jvm/org/scalaz:scalaz_core_2_11"
    ],
    visibility = [
        "//visibility:public"
    ]
)



scala_import(
    name = "coursier_cache",
    jars = [
        "//external:jar/io/get_coursier/coursier_cache_2_11"
    ],
    runtime_deps = [
        "//3rdparty/jvm/org/scala_lang:scala_library",
        "//3rdparty/jvm/org/scalaz:scalaz_concurrent_2_11",
        ":coursier"
    ],
    visibility = [
        "//visibility:public"
    ]
)


