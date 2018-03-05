def _bazelbub_aspect_impl(target, ctx):

  output = ctx.new_file("bazelbub.%s.json" % target.label.name)

  runtime_classpath = _get_runtime_jars(target)
  data = struct(
    runtime_classpath = [file.path for file in runtime_classpath],
  )
  ctx.file_action(output, data.to_json())

  return struct(
      output_groups = {
          "bazelbub": depset([output], transitive = [runtime_classpath]),
      }
  )

def _get_runtime_jars(target):
    if hasattr(target, "java"):
        return target.java.transitive_runtime_deps
    if java_common.provider in target:
        return target[java_common.provider].transitive_runtime_jars
    return depset()

bazelbub_aspect = aspect(
    implementation = _bazelbub_aspect_impl
)
