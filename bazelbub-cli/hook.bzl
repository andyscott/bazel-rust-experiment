hook_aspect = aspect(
    implementation = _hook_aspect_impl,
    attr_aspects = ['deps'],
)

def _hook_aspect_impl(target, ctx):
    if hasattr(ctx.rule.attr, 'srcs'):
        for src in ctx.rule.attr.srcs:
            for f in src.files:
                print(f.path)
    return []
