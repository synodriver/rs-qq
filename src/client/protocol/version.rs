#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(Default)]
pub enum Protocol {
    #[derivative(Default)]
    IPad,
    AndroidPhone,
    AndroidWatch,
    MacOS,
    QiDian,
}

#[derive(Debug, Clone)]
pub struct Version {
    pub apk_sign: &'static [u8],
    pub apk_id: &'static str,
    pub sort_version_name: &'static str,
    pub sdk_version: &'static str,
    pub app_id: u32,
    pub sub_app_id: u32,
    pub build_time: u32,
    pub sso_version: u32,
    pub misc_bitmap: u32,
    pub sub_sig_map: u32,
    pub main_sig_map: u32,
    pub protocol: Protocol,
}

pub fn get_version(p: Protocol) -> &'static Version {
    match p {
        Protocol::IPad => IPAD,
        Protocol::AndroidPhone => ANDROID_PHONE,
        Protocol::AndroidWatch => ANDROID_WATCH,
        Protocol::MacOS => MACOS,
        Protocol::QiDian => QIDIAN,
    }
}

pub static ANDROID_PHONE: &'static Version = &Version {
    apk_id: "com.tencent.mobileqq",
    app_id: 537066738,
    sub_app_id: 537066738,
    sort_version_name: "8.5.0",
    build_time: 1607689988,
    apk_sign: &[
        0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6,
        0x8D,
    ],
    sdk_version: "6.0.0.2454",
    sso_version: 15,
    misc_bitmap: 184024956,
    sub_sig_map: 0x10400,
    main_sig_map: 34869472,
    protocol: Protocol::AndroidPhone,
};

pub static IPAD: &'static Version = &Version {
    apk_id: "com.tencent.minihd.qq",
    app_id: 537065739,
    sub_app_id: 537065739,
    sort_version_name: "5.8.9",
    build_time: 1595836208,
    apk_sign: &[
        170, 57, 120, 244, 31, 217, 111, 249, 145, 74, 102, 158, 24, 100, 116, 199,
    ],
    sdk_version: "6.0.0.2433",
    sso_version: 12,
    misc_bitmap: 150470524,
    sub_sig_map: 66560,
    main_sig_map: 1970400,
    protocol: Protocol::IPad,
};

pub static ANDROID_WATCH: &'static Version = &Version {
    apk_id: "com.tencent.qqlite",
    app_id: 537064446,
    sub_app_id: 537064446,
    sort_version_name: "2.0.5",
    build_time: 1559564731,
    apk_sign: &[
        0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6,
        0x8D,
    ],
    sdk_version: "6.0.0.236",
    sso_version: 5,
    misc_bitmap: 16252796,
    sub_sig_map: 0x10400,
    main_sig_map: 34869472,
    protocol: Protocol::AndroidWatch,
};

pub static MACOS: &'static Version = &Version {
    apk_id: "com.tencent.minihd.qq",
    app_id: 537064315,
    sub_app_id: 537064315,
    sort_version_name: "5.8.9",
    build_time: 1595836208,
    apk_sign: &[
        170, 57, 120, 244, 31, 217, 111, 249, 145, 74, 102, 158, 24, 100, 116, 199,
    ],
    sdk_version: "6.0.0.2433",
    sso_version: 12,
    misc_bitmap: 150470524,
    sub_sig_map: 66560,
    main_sig_map: 1970400,
    protocol: Protocol::MacOS,
};

pub static QIDIAN: &'static Version = &Version {
    apk_id: "com.tencent.qidian",
    app_id: 537061386,
    sub_app_id: 537036590,
    sort_version_name: "3.8.6",
    build_time: 1556628836,
    apk_sign: &[
        160, 30, 236, 171, 133, 233, 227, 186, 43, 15, 106, 21, 140, 133, 92, 41,
    ],
    sdk_version: "6.0.0.2365",
    sso_version: 5,
    misc_bitmap: 49807228,
    sub_sig_map: 66560,
    main_sig_map: 34869472,
    protocol: Protocol::QiDian,
};
