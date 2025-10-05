https://github.com/ruby/spec
More accurately, "Ruby Test Suite"

# Tested categories
- Language syntax under `spec/language/`
- Core library under `spec/core/`
- Standard library under `spec/library/`
- C extension api under `spec/optional/capi`
- Command line flags under `spec/command_line`
C extension API and command line flags don't really apply to us.

# MSpec runner
- A test runner specifically for running Ruby Spec
	- Tries to use a small subset of Ruby features, to avoid 