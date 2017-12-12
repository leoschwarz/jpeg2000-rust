# Rust bindings to OpenJPEG
This crate provides access to OpenJPEG's JPEG2000 decoder.

## Warning
Please be advised that using C code means this crate is likely vulnerable to various memory exploits, e.g. see [http://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2016-8332](CVE-2016-8332) for an actual example from the past.

As soon as someone writes an efficient JPEG2000 decoder in pure Rust you should probably switch over to that.

## License
Most of the code in this repository is provided under the GPL license (check out the relevant's files headers for more details).
However note that this crate links statically to OpenJPEG which has its own (permissive) license, which you can find at `openjp2-sys/libopenjpeg/LICENSE` (you might have to check out the git submodule first).
