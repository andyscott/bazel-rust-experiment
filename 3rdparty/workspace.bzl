def declare_maven(hash):
    native.maven_jar(
        name = hash["name"],
        artifact = hash["artifact"],
        sha1 = hash["sha1"],
        repository = hash["repository"]
    )
    native.bind(
        name = hash["bind"],
        actual = hash["actual"]
    )

def maven_dependencies(callback = declare_maven):
    callback({"artifact": "io.circe:circe-core_2.11:0.9.1", "lang": "scala", "sha1": "7c409e1e781cc711248e0b84c3a31c7f42ce2c3d", "repository": "http://central.maven.org/maven2/", "name": "io_circe_circe_core_2_11", "actual": "@io_circe_circe_core_2_11//jar:file", "bind": "jar/io/circe/circe_core_2_11"})
    callback({"artifact": "io.circe:circe-jawn_2.11:0.9.1", "lang": "scala", "sha1": "12f5d5593039a39a8c61db633484afe6a0f38e3f", "repository": "http://central.maven.org/maven2/", "name": "io_circe_circe_jawn_2_11", "actual": "@io_circe_circe_jawn_2_11//jar:file", "bind": "jar/io/circe/circe_jawn_2_11"})
    callback({"artifact": "io.circe:circe-numbers_2.11:0.9.1", "lang": "java", "sha1": "cc5a78867a1f285970cb184cd75c354c9e82e8a6", "repository": "http://central.maven.org/maven2/", "name": "io_circe_circe_numbers_2_11", "actual": "@io_circe_circe_numbers_2_11//jar", "bind": "jar/io/circe/circe_numbers_2_11"})
    callback({"artifact": "io.get-coursier:coursier-cache_2.11:1.0.1", "lang": "scala", "sha1": "397ca56b22ef136e955c8177ea038594b15366ee", "repository": "http://central.maven.org/maven2/", "name": "io_get_coursier_coursier_cache_2_11", "actual": "@io_get_coursier_coursier_cache_2_11//jar:file", "bind": "jar/io/get_coursier/coursier_cache_2_11"})
    callback({"artifact": "io.get-coursier:coursier_2.11:1.0.1", "lang": "scala", "sha1": "697f4a8ca0e9fc61b3da8338e6de6cd295c3c300", "repository": "http://central.maven.org/maven2/", "name": "io_get_coursier_coursier_2_11", "actual": "@io_get_coursier_coursier_2_11//jar:file", "bind": "jar/io/get_coursier/coursier_2_11"})
    callback({"artifact": "org.scala-lang.modules:scala-xml_2.11:1.0.6", "lang": "java", "sha1": "4ebd108453e6455351c0ec50d32509ae1154fdb1", "repository": "http://central.maven.org/maven2/", "name": "org_scala_lang_modules_scala_xml_2_11", "actual": "@org_scala_lang_modules_scala_xml_2_11//jar", "bind": "jar/org/scala_lang/modules/scala_xml_2_11"})
    callback({"artifact": "org.scala-lang:scala-library:2.11.12", "lang": "java", "sha1": "bf5534e6fec3d665bd6419c952a929a8bdd4b591", "repository": "http://central.maven.org/maven2/", "name": "org_scala_lang_scala_library", "actual": "@org_scala_lang_scala_library//jar", "bind": "jar/org/scala_lang/scala_library"})
    callback({"artifact": "org.scala-lang:scala-reflect:2.11.8", "lang": "java", "sha1": "b74530deeba742ab4f3134de0c2da0edc49ca361", "repository": "http://central.maven.org/maven2/", "name": "org_scala_lang_scala_reflect", "actual": "@org_scala_lang_scala_reflect//jar", "bind": "jar/org/scala_lang/scala_reflect"})
    callback({"artifact": "org.scalaz:scalaz-concurrent_2.11:7.2.16", "lang": "java", "sha1": "8c8f59d900b7f66d87addd468a27a6f648256b3d", "repository": "http://central.maven.org/maven2/", "name": "org_scalaz_scalaz_concurrent_2_11", "actual": "@org_scalaz_scalaz_concurrent_2_11//jar", "bind": "jar/org/scalaz/scalaz_concurrent_2_11"})
    callback({"artifact": "org.scalaz:scalaz-core_2.11:7.2.16", "lang": "java", "sha1": "3bc3cf470728a47c7e2590b0da7b8bc21b995736", "repository": "http://central.maven.org/maven2/", "name": "org_scalaz_scalaz_core_2_11", "actual": "@org_scalaz_scalaz_core_2_11//jar", "bind": "jar/org/scalaz/scalaz_core_2_11"})
    callback({"artifact": "org.scalaz:scalaz-effect_2.11:7.2.16", "lang": "java", "sha1": "b90b1a04e488245013da09500dcbadf853515192", "repository": "http://central.maven.org/maven2/", "name": "org_scalaz_scalaz_effect_2_11", "actual": "@org_scalaz_scalaz_effect_2_11//jar", "bind": "jar/org/scalaz/scalaz_effect_2_11"})
    callback({"artifact": "org.spire-math:jawn-parser_2.11:0.11.0", "lang": "java", "sha1": "e17c156887c97440db2d8d3e513b0cfb7e5ae327", "repository": "http://central.maven.org/maven2/", "name": "org_spire_math_jawn_parser_2_11", "actual": "@org_spire_math_jawn_parser_2_11//jar", "bind": "jar/org/spire_math/jawn_parser_2_11"})
    callback({"artifact": "org.typelevel:cats-core_2.11:1.0.1", "lang": "scala", "sha1": "c796592c733267960bd620fba97053685eb1333c", "repository": "http://central.maven.org/maven2/", "name": "org_typelevel_cats_core_2_11", "actual": "@org_typelevel_cats_core_2_11//jar:file", "bind": "jar/org/typelevel/cats_core_2_11"})
    callback({"artifact": "org.typelevel:cats-effect_2.11:0.8", "lang": "scala", "sha1": "9ec374264cf015432a6422710bad64426df03403", "repository": "http://central.maven.org/maven2/", "name": "org_typelevel_cats_effect_2_11", "actual": "@org_typelevel_cats_effect_2_11//jar:file", "bind": "jar/org/typelevel/cats_effect_2_11"})
    callback({"artifact": "org.typelevel:cats-kernel_2.11:1.0.1", "lang": "scala", "sha1": "029d1039d964b894389c09d3dfa1e1eb6cdfccd7", "repository": "http://central.maven.org/maven2/", "name": "org_typelevel_cats_kernel_2_11", "actual": "@org_typelevel_cats_kernel_2_11//jar:file", "bind": "jar/org/typelevel/cats_kernel_2_11"})
    callback({"artifact": "org.typelevel:cats-macros_2.11:1.0.1", "lang": "java", "sha1": "f5bc0a113ed95a451f6c31ae597a798fbddb76b5", "repository": "http://central.maven.org/maven2/", "name": "org_typelevel_cats_macros_2_11", "actual": "@org_typelevel_cats_macros_2_11//jar", "bind": "jar/org/typelevel/cats_macros_2_11"})
    callback({"artifact": "org.typelevel:machinist_2.11:0.6.2", "lang": "java", "sha1": "029c6a46d66b6616f8795a70753e6753975f42fc", "repository": "http://central.maven.org/maven2/", "name": "org_typelevel_machinist_2_11", "actual": "@org_typelevel_machinist_2_11//jar", "bind": "jar/org/typelevel/machinist_2_11"})
