cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]
cargo_test_options = []

if get_option('profile') == 'default'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

if get_option('disable_udev_required_tests')
    cargo_test_options += [ '--features', 'disable-udev-required-tests' ]
endif

custom_target(
  'oci-dev-binder-hook',
  build_by_default: true,
  input: oci_dev_binder_hook_sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: oci_hooks_bindir,
  command: [
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
  ],
  env: {
    'CARGO_HOME': meson.project_build_root() / 'cargo-home',
  }
)

test(
  'cargo test',
  cargo,
  args: [ 'test', cargo_options, cargo_test_options ],
  env: {
    'CARGO_HOME': meson.project_build_root() / 'cargo-home',
  },
  verbose: true,
)
