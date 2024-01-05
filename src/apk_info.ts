export type ApkInfo = {
    file_name:string,
    package_name:string,
    version_code: string,
    version_name: string,
    min_sdk_version: string,
    target_sdk_version: string,
    compile_sdk_version: string,
    compile_sdk_version_code_name: string,
    permissions: string[],
    icon: string,
}