def cargo_bin(
        name,
        srcs = [],
        manifest_path = "Cargo.toml",
        bin_name = None,
        release = False,
):
    bin_name = name if bin_name == None else bin_name

    cmd = _strip_margin("""
      |#export CARGO_HOME={cargo_home}
      |export CARGO_TARGET_DIR={cargo_target_dir}
      |cargo build \\
      |  --manifest-path $(location {manifest_path}) \\
      |  --bin {bin_name}
      |find {cargo_target_dir} -name {bin_name} -exec bash -c 'cp "$$0" $(location {bin_name})' {{}} \\;      
      """
    ).format(
        cargo_home = "`pwd`",
        cargo_target_dir = "`pwd`/target",
        bin_name = bin_name,
        manifest_path = manifest_path,
    )

    native.genrule(
        name = name,
        srcs = srcs + native.glob(["src/**/*.rs"]) + [manifest_path],
        outs = [bin_name],
        cmd = cmd,
        executable = True,
        local = 1,
        tags = ["manual", "requires-network", "arc-ignore"],
    )

def _strip_margin(str, delim = "|"):
    """
    For every line in str:
        Strip a leading prefix consisting of spaces followed by delim from the line.
        This is extremely similar to Scala's .stripMargin
    """
    return "\n".join([
        _strip_margin_line(line, delim) for line in str.splitlines()
    ])

def _strip_margin_line(line, delim):
    trimmed = line.lstrip(" ")
    pos = trimmed.find(delim, end = 1)
    if pos == 0:
        return trimmed[1:]
    else:
        return line
