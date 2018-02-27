def _bazelbub_aspect_impl(target, ctx):

  output = ctx.new_file("bazelbub.%s.json" % target.label.name)
  data = struct(
    runtime_classpath = [file.path for file in _get_runtime_jars(target)],
  )
  ctx.file_action(output, data.to_json())

  return struct(
      output_groups = {
          "bazelbub": depset([output]),
      }
  )

def _get_runtime_jars(target):
    if hasattr(target, "java"):
        return target.java.compilation_info.runtime_classpath
    if java_common.provider in target:
        return target[java_common.provider].transitive_runtime_jars
    return depset()

bazelbub_aspect = aspect(
    implementation = _bazelbub_aspect_impl
)
