load("@io_bazel_rules_scala//scala:scala_import.bzl", "scala_import")
scala_import(
    name = "cats_core",
    jars = [
        "//external:jar/org/typelevel/cats_core_2_11"
    ],
    runtime_deps = [
        "//3rdparty/jvm/org/scala_lang:scala_library",
        ":cats_kernel",
        ":cats_macros_2_11",
        ":machinist_2_11"
    ],
    visibility = [
        "//visibility:public"
    ]
)



scala_import(
    name = "cats_effect",
    jars = [
        "//external:jar/org/typelevel/cats_effect_2_11"
    ],
    runtime_deps = [
        "//3rdparty/jvm/org/scala_lang:scala_library",
        ":cats_core"
    ],
    visibility = [
        "//visibility:public"
    ]
)



scala_import(
    name = "cats_kernel",
    jars = [
        "//external:jar/org/typelevel/cats_kernel_2_11"
    ],
    runtime_deps = [
        "//3rdparty/jvm/org/scala_lang:scala_library"
    ],
    visibility = [
        "//visibility:public"
    ]
)



java_library(
    name = "cats_macros_2_11",
    exports = [
        "//external:jar/org/typelevel/cats_macros_2_11"
    ],
    visibility = [
        "//visibility:public"
    ]
)



java_library(
    name = "machinist_2_11",
    exports = [
        "//external:jar/org/typelevel/machinist_2_11"
    ],
    runtime_deps = [
        "//3rdparty/jvm/org/scala_lang:scala_reflect"
    ],
    visibility = [
        "//visibility:public"
    ]
)


