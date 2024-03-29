pub mod parser {
    use std::io::Read;
    use std::path::PathBuf;
    use xml::reader::XmlEvent;
    use xml::EventReader;

    use crate::app::android_xml::axml;
    use crate::app::apk_info::ApkParsedInfo;
    use crate::ErrRes;

    // pub async fn parse(path: &PathBuf) -> Option<ApkParsedInfo> {
    // pub fn parse(path: &PathBuf) -> Option<ApkParsedInfo> {
    pub fn parse(path: &PathBuf) -> Result<ApkParsedInfo,ErrRes> {
        let file = std::fs::File::open(path).map_err(|e|ErrRes{code:200,msg:e.to_string()})?;
        let mut file_content: Vec<u8> = Vec::new();
        let mut icon = String::new();
        // let mut archive = zip::ZipArchive::new(file).unwrap();
        let mut archive = zip::ZipArchive::new(file).map_err(|e| ErrRes{code:200,msg:e.to_string()})?;
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        for i in 0..archive.len() {
            let mut in_file = archive.by_index(i).unwrap();
            if in_file.name() == "res/drawable-xxxhdpi-v4/ic_launcher.png" {
                let mut f_content: Vec<u8> = Vec::new();
                in_file.read_to_end(&mut f_content).unwrap();
                icon = base64::encode(&f_content);
                break;
            }
        }

        for i in 0..archive.len() {
            let mut inner_file = archive.by_index(i).unwrap();
            if inner_file.name() == "AndroidManifest.xml" {
                inner_file.read_to_end(&mut file_content).unwrap();
                break;
            }
        }

        let xml = axml::extract_xml(file_content).map_err(|e|ErrRes{code:200,msg:e})?;
        parse_to_info(xml, icon,file_name)
    }

    // fn parse_to_info(content: String, icon: String, file_name:String) -> Option<ApkParsedInfo> {
    fn parse_to_info(content: String, icon: String, file_name:String) -> Result<ApkParsedInfo,ErrRes> {
        let mut apk_info = ApkParsedInfo::new();

        apk_info.icon = icon;
        apk_info.file_name = file_name;
        let reader = EventReader::from_str(content.as_str());
        for e in reader {
            match e {
                Ok(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace: _,
                }) => match name.local_name.as_str() {
                    "manifest" => {
                        for attribute in attributes {
                            let attr = attribute.name.to_string();

                            if attr.contains("versionCode") {
                                apk_info.version_code = attribute.value;
                            } else if attr.contains("versionName") {
                                apk_info.version_name = attribute.value;
                            } else if attr.contains("compileSdkVersionCodename") {
                                apk_info.compile_sdk_version_code_name = attribute.value;
                            } else if attr.contains("compileSdkVersion") {
                                apk_info.compile_sdk_version = attribute.value;
                            } else if attr.contains("package") {
                                apk_info.package_name = attribute.value;
                            }
                        }
                    }

                    "uses-sdk" => {
                        for attribute in attributes {
                            let attr = attribute.name.to_string();

                            if attr.contains("minSdkVersion") {
                                apk_info.min_sdk_version = attribute.value;
                            } else if attr.contains("targetSdkVersion") {
                                apk_info.target_sdk_version = attribute.value;
                            }
                        }
                    }

                    "uses-permission" => {
                        for attribute in attributes {
                            if attribute.name.to_string().contains("name") {
                                apk_info.permissions.push(attribute.value)
                            }
                        }
                    }

                    _ => {}
                },

                Err(e) => {
                    return Err(ErrRes{
                        code:200,
                        msg:e.to_string()
                    })
                    // return None;
                }

                _ => {}
            }
        }
        // Some(apk_info)
        Ok(apk_info)
    }
}
