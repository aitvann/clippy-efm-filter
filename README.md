# Clippy efm filter

`cargo clippy` works for the whole package,
the `efm-langserver` expects it to work with a single file
thus, a self-written `clippy-efm-filter` tool is used
to filter the output of clippy for a specific file.
Probably `grep` can be used for this, but I don't know how :)
