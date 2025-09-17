oci_hook_conf = configuration_data()
oci_hook_conf.set('oci-hooks-bindir', oci_hooks_bindir)
configure_file(
    input: 'oci-dev-binder-hook.json.in',
    output: 'oci-dev-binder-hook.json',
    configuration: oci_hook_conf,
    install: true,
    install_dir: oci_hooks_datadir
)
