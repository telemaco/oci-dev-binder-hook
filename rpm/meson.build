spec = configuration_data()

project_version = meson.project_version()
version_array = project_version.split('.')

if 'dev' in meson.project_version()
    version = '.'.join(version_array[0], version_array[1], version_array[2])
    release = '1.' + version_array[3] + '%{?dist}'
    dev_version = '.' + version_array[3]
else
    version = meson.project_version()
    release = '%autorelease'
    dev_version = ''
endif

spec.set('VERSION', version)
spec.set('DEV_VERSION', dev_version)

configure_file(
    input: 'oci-dev-binder-hook.spec.in',
    output: 'oci-dev-binder-hook.spec',
    configuration: spec,
)
