project('oci-dev-binder-hook', 'rust',
        version: '0.1.0',
        meson_version: '>= 0.62',
        license: 'GPL-2.0-or-later'
)

cargo = find_program('cargo')
udev = dependency('udev', required : true)

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
libexec = prefix / get_option('libexecdir')
datadir = prefix / get_option('datadir')
oci_hooks_datadir = datadir / 'containers/oci/hooks.d'
oci_hooks_bindir = libexec / 'oci/hooks.d'

oci_dev_binder_hook_sources = files (
    'Cargo.toml',
    'src/lib.rs',
    'src/main.rs',
    'src/cli.rs',
    'src/oci_spec.rs'
)

subdir('src')
subdir('data')

summary({
  'Hook binary': oci_hooks_bindir / 'oci-dev-binder-hook',
  'Hook configuration': oci_hooks_datadir / 'oci-dev-binder-hook.json',
}, section: 'Configuration summary', bool_yn: true)
