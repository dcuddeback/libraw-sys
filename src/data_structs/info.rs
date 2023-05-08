use libc::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_gps_info_t {
    pub latitude: [f32; 3usize],
    pub longitude: [f32; 3usize],
    pub gpstimestamp: [f32; 3usize],
    pub altitude: f32,
    pub altref: c_char,
    pub latref: c_char,
    pub longref: c_char,
    pub gpsstatus: c_char,
    pub gpsparsed: c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_shootinginfo_t {
    pub DriveMode: c_short,
    pub FocusMode: c_short,
    pub MeteringMode: c_short,
    pub AFPoint: c_short,
    pub ExposureMode: c_short,
    pub ExposureProgram: c_short,
    pub ImageStabilization: c_short,
    pub BodySerial: [c_char; 64usize],
    pub InternalBodySerial: [c_char; 64usize],
}

#[repr(C)]
pub struct libraw_lensinfo_t {
    pub MinFocal: f32,
    pub MaxFocal: f32,
    pub MaxAp4MinFocal: f32,
    pub MaxAp4MaxFocal: f32,
    pub EXIF_MaxAp: f32,
    pub LensMake: [c_char; 128usize],
    pub Lens: [c_char; 128usize],
    pub LensSerial: [c_char; 128usize],
    pub InternalLensSerial: [c_char; 128usize],
    pub FocalLengthIn35mmFormat: c_ushort,
    pub nikon: libraw_nikonlens_t,
    pub dng: libraw_dnglens_t,
    pub makernotes: libraw_makernotes_lens_t,
}

#[repr(C)]
pub struct libraw_nikonlens_t {
    pub EffectiveMaxAp: f32,
    pub LensIDNumber: c_uchar,
    pub LensFStops: c_uchar,
    pub MCUVersion: c_uchar,
    pub LensType: c_uchar,
}

#[repr(C)]
pub struct libraw_dnglens_t {
    pub MinFocal: f32,
    pub MaxFocal: f32,
    pub MaxAp4MinFocal: f32,
    pub MaxAp4MaxFocal: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_makernotes_lens_t {
    pub LensID: c_ulonglong,
    pub Lens: [c_char; 128usize],
    pub LensFormat: c_ushort,
    pub LensMount: c_ushort,
    pub CamID: c_ulonglong,
    pub CameraFormat: c_ushort,
    pub CameraMount: c_ushort,
    pub body: [c_char; 64usize],
    pub FocalType: c_short,
    pub LensFeatures_pre: [c_char; 16usize],
    pub LensFeatures_suf: [c_char; 16usize],
    pub MinFocal: f32,
    pub MaxFocal: f32,
    pub MaxAp4MinFocal: f32,
    pub MaxAp4MaxFocal: f32,
    pub MinAp4MinFocal: f32,
    pub MinAp4MaxFocal: f32,
    pub MaxAp: f32,
    pub MinAp: f32,
    pub CurFocal: f32,
    pub CurAp: f32,
    pub MaxAp4CurFocal: f32,
    pub MinAp4CurFocal: f32,
    pub MinFocusDistance: f32,
    pub FocusRangeIndex: f32,
    pub LensFStops: f32,
    pub TeleconverterID: c_ulonglong,
    pub Teleconverter: [c_char; 128usize],
    pub AdapterID: c_ulonglong,
    pub Adapter: [c_char; 128usize],
    pub AttachmentID: c_ulonglong,
    pub Attachment: [c_char; 128usize],
    pub FocalUnits: c_ushort,
    pub FocalLengthIn35mmFormat: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_makernotes_t {
    pub canon: libraw_canon_makernotes_t,
    pub nikon: libraw_nikon_makernotes_t,
    pub hasselblad: libraw_hasselblad_makernotes_t,
    pub fuji: libraw_fuji_info_t,
    pub olympus: libraw_olympus_makernotes_t,
    pub sony: libraw_sony_info_t,
    pub kodak: libraw_kodak_makernotes_t,
    pub panasonic: libraw_panasonic_makernotes_t,
    pub pentax: libraw_pentax_makernotes_t,
    pub phaseone: libraw_p1_makernotes_t,
    pub ricoh: libraw_ricoh_makernotes_t,
    pub samsung: libraw_samsung_makernotes_t,
    pub common: libraw_metadata_common_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_canon_makernotes_t {
    pub ColorDataVer: c_int,
    pub ColorDataSubVer: c_int,
    pub SpecularWhiteLevel: c_int,
    pub NormalWhiteLevel: c_int,
    pub ChannelBlackLevel: [c_int; 4usize],
    pub AverageBlackLevel: c_int,
    pub multishot: [c_uint; 4usize],
    pub MeteringMode: c_short,
    pub SpotMeteringMode: c_short,
    pub FlashMeteringMode: c_uchar,
    pub FlashExposureLock: c_short,
    pub ExposureMode: c_short,
    pub AESetting: c_short,
    pub ImageStabilization: c_short,
    pub FlashMode: c_short,
    pub FlashActivity: c_short,
    pub FlashBits: c_short,
    pub ManualFlashOutput: c_short,
    pub FlashOutput: c_short,
    pub FlashGuideNumber: c_short,
    pub ContinuousDrive: c_short,
    pub SensorWidth: c_short,
    pub SensorHeight: c_short,
    pub AFMicroAdjMode: c_int,
    pub AFMicroAdjValue: f32,
    pub MakernotesFlip: c_short,
    pub RecordMode: c_short,
    pub SRAWQuality: c_short,
    pub wbi: c_uint,
    pub RF_lensID: c_short,
    pub AutoLightingOptimizer: c_int,
    pub HighlightTonePriority: c_int,
    pub Quality: c_short,
    pub CanonLog: c_int,
    pub DefaultCropAbsolute: libraw_area_t,
    pub RecommendedImageArea: libraw_area_t,
    pub LeftOpticalBlack: libraw_area_t,
    pub UpperOpticalBlack: libraw_area_t,
    pub ActiveArea: libraw_area_t,
    pub ISOgain: [c_short; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_hasselblad_makernotes_t {
    pub BaseISO: c_int,
    pub Gain: f64,
    pub Sensor: [c_char; 8usize],
    pub SensorUnit: [c_char; 64usize],
    pub HostBody: [c_char; 64usize],
    pub SensorCode: c_int,
    pub SensorSubCode: c_int,
    pub CoatingCode: c_int,
    pub uncropped: c_int,
    pub CaptureSequenceInitiator: [c_char; 32usize],
    pub SensorUnitConnector: [c_char; 64usize],
    pub format: c_int,
    pub nIFD_CM: [c_int; 2usize],
    pub RecommendedCrop: [c_int; 2usize],
    pub mnColorMatrix: [[f64; 3usize]; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_nikon_makernotes_t {
    pub ExposureBracketValue: f64,
    pub ActiveDLighting: c_ushort,
    pub ShootingMode: c_ushort,
    pub ImageStabilization: [c_uchar; 7usize],
    pub VibrationReduction: c_uchar,
    pub VRMode: c_uchar,
    pub FlashSetting: [c_char; 13usize],
    pub FlashType: [c_char; 20usize],
    pub FlashExposureCompensation: [c_uchar; 4usize],
    pub ExternalFlashExposureComp: [c_uchar; 4usize],
    pub FlashExposureBracketValue: [c_uchar; 4usize],
    pub FlashMode: c_uchar,
    pub FlashExposureCompensation2: c_schar,
    pub FlashExposureCompensation3: c_schar,
    pub FlashExposureCompensation4: c_schar,
    pub FlashSource: c_uchar,
    pub FlashFirmware: [c_uchar; 2usize],
    pub ExternalFlashFlags: c_uchar,
    pub FlashControlCommanderMode: c_uchar,
    pub FlashOutputAndCompensation: c_uchar,
    pub FlashFocalLength: c_uchar,
    pub FlashGNDistance: c_uchar,
    pub FlashGroupControlMode: [c_uchar; 4usize],
    pub FlashGroupOutputAndCompensation: [c_uchar; 4usize],
    pub FlashColorFilter: c_uchar,
    pub NEFCompression: c_ushort,
    pub ExposureMode: c_int,
    pub ExposureProgram: c_int,
    pub nMEshots: c_int,
    pub MEgainOn: c_int,
    pub ME_WB: [f64; 4usize],
    pub AFFineTune: c_uchar,
    pub AFFineTuneIndex: c_uchar,
    pub AFFineTuneAdj: i8,
    pub LensDataVersion: c_uint,
    pub FlashInfoVersion: c_uint,
    pub ColorBalanceVersion: c_uint,
    pub key: c_uchar,
    pub NEFBitDepth: [c_ushort; 4usize],
    pub HighSpeedCropFormat: c_ushort,
    pub SensorHighSpeedCrop: libraw_sensor_highspeed_crop_t,
    pub SensorWidth: c_ushort,
    pub SensorHeight: c_ushort,
    pub Active_D_Lighting: c_ushort,
    pub ShotInfoVersion: c_uint,
    pub MakernotesFlip: c_short,
    pub RollAngle: f64,
    pub PitchAngle: f64,
    pub YawAngle: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_sensor_highspeed_crop_t {
    pub cleft: c_ushort,
    pub ctop: c_ushort,
    pub cwidth: c_ushort,
    pub cheight: c_ushort,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_olympus_makernotes_t {
    pub CameraType2: [c_char; 6usize],
    pub ValidBits: c_ushort,
    pub SensorCalibration: [c_int; 2usize],
    pub DriveMode: [c_ushort; 5usize],
    pub ColorSpace: c_ushort,
    pub FocusMode: [c_ushort; 2usize],
    pub AutoFocus: c_ushort,
    pub AFPoint: c_ushort,
    pub AFAreas: [c_uint; 64usize],
    pub AFPointSelected: [f64; 5usize],
    pub AFResult: c_ushort,
    pub AFFineTune: c_uchar,
    pub AFFineTuneAdj: [c_short; 3usize],
    pub SpecialMode: [c_uint; 3usize],
    pub ZoomStepCount: c_ushort,
    pub FocusStepCount: c_ushort,
    pub FocusStepInfinity: c_ushort,
    pub FocusStepNear: c_ushort,
    pub FocusDistance: f64,
    pub AspectFrame: [c_ushort; 4usize],
    pub StackedImage: [c_uint; 2usize],
    pub isLiveND: c_uchar,
    pub LiveNDfactor: c_uint,
    pub Panorama_mode: c_ushort,
    pub Panorama_frameNum: c_ushort,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_panasonic_makernotes_t {
    pub Compression: c_ushort,
    pub BlackLevelDim: c_ushort,
    pub BlackLevel: [f32; 8usize],
    pub Multishot: c_uint,
    pub gamma: f32,
    pub HighISOMultiplier: [c_int; 3usize],
    pub FocusStepNear: c_short,
    pub FocusStepCount: c_short,
    pub ZoomPosition: c_uint,
    pub LensManufacturer: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_pentax_makernotes_t {
    pub DriveMode: [c_uchar; 4usize],
    pub FocusMode: [c_ushort; 2usize],
    pub AFPointSelected: [c_ushort; 2usize],
    pub AFPointSelected_Area: c_ushort,
    pub AFPointsInFocus_version: c_int,
    pub AFPointsInFocus: c_uint,
    pub FocusPosition: c_ushort,
    pub AFAdjustment: c_short,
    pub AFPointMode: c_uchar,
    pub MultiExposure: c_uchar,
    pub Quality: c_ushort,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_ricoh_makernotes_t {
    pub AFStatus: c_ushort,
    pub AFAreaXPosition: [c_uint; 2usize],
    pub AFAreaYPosition: [c_uint; 2usize],
    pub AFAreaMode: c_ushort,
    pub SensorWidth: c_uint,
    pub SensorHeight: c_uint,
    pub CroppedImageWidth: c_uint,
    pub CroppedImageHeight: c_uint,
    pub WideAdapter: c_ushort,
    pub CropMode: c_ushort,
    pub NDFilter: c_ushort,
    pub AutoBracketing: c_ushort,
    pub MacroMode: c_ushort,
    pub FlashMode: c_ushort,
    pub FlashExposureComp: f64,
    pub ManualFlashOutput: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_samsung_makernotes_t {
    pub ImageSizeFull: [c_uint; 4usize],
    pub ImageSizeCrop: [c_uint; 4usize],
    pub ColorSpace: [c_int; 2usize],
    pub key: [c_uint; 11usize],
    pub DigitalGain: f64,
    pub DeviceType: c_int,
    pub LensFirmware: [c_char; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_kodak_makernotes_t {
    pub BlackLevelTop: c_ushort,
    pub BlackLevelBottom: c_ushort,
    pub offset_left: c_short,
    pub offset_top: c_short,
    pub clipBlack: c_ushort,
    pub clipWhite: c_ushort,
    pub romm_camDaylight: [[f32; 3usize]; 3usize],
    pub romm_camTungsten: [[f32; 3usize]; 3usize],
    pub romm_camFluorescent: [[f32; 3usize]; 3usize],
    pub romm_camFlash: [[f32; 3usize]; 3usize],
    pub romm_camCustom: [[f32; 3usize]; 3usize],
    pub romm_camAuto: [[f32; 3usize]; 3usize],
    pub val018percent: c_ushort,
    pub val100percent: c_ushort,
    pub val170percent: c_ushort,
    pub MakerNoteKodak8a: c_short,
    pub ISOCalibrationGain: f32,
    pub AnalogISO: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_p1_makernotes_t {
    pub Software: [c_char; 64usize],
    pub SystemType: [c_char; 64usize],
    pub FirmwareString: [c_char; 256usize],
    pub SystemModel: [c_char; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_sony_info_t {
    pub CameraType: c_ushort,
    pub Sony0x9400_version: c_uchar,
    pub Sony0x9400_ReleaseMode2: c_uchar,
    pub Sony0x9400_SequenceImageNumber: c_uint,
    pub Sony0x9400_SequenceLength1: c_uchar,
    pub Sony0x9400_SequenceFileNumber: c_uint,
    pub Sony0x9400_SequenceLength2: c_uchar,
    pub AFAreaModeSetting: u8,
    pub AFAreaMode: u16,
    pub FlexibleSpotPosition: [c_ushort; 2usize],
    pub AFPointSelected: u8,
    pub AFPointSelected_0x201e: u8,
    pub nAFPointsUsed: c_short,
    pub AFPointsUsed: [u8; 10usize],
    pub AFTracking: u8,
    pub AFType: u8,
    pub FocusLocation: [c_ushort; 4usize],
    pub FocusPosition: c_ushort,
    pub AFMicroAdjValue: i8,
    pub AFMicroAdjOn: i8,
    pub AFMicroAdjRegisteredLenses: c_uchar,
    pub VariableLowPassFilter: c_ushort,
    pub LongExposureNoiseReduction: c_uint,
    pub HighISONoiseReduction: c_ushort,
    pub HDR: [c_ushort; 2usize],
    pub group2010: c_ushort,
    pub group9050: c_ushort,
    pub real_iso_offset: c_ushort,
    pub MeteringMode_offset: c_ushort,
    pub ExposureProgram_offset: c_ushort,
    pub ReleaseMode2_offset: c_ushort,
    pub MinoltaCamID: c_uint,
    pub firmware: f32,
    pub ImageCount3_offset: c_ushort,
    pub ImageCount3: c_uint,
    pub ElectronicFrontCurtainShutter: c_uint,
    pub MeteringMode2: c_ushort,
    pub SonyDateTime: [c_char; 20usize],
    pub ShotNumberSincePowerUp: c_uint,
    pub PixelShiftGroupPrefix: c_ushort,
    pub PixelShiftGroupID: c_uint,
    pub nShotsInPixelShiftGroup: c_char,
    pub numInPixelShiftGroup: c_char,
    pub prd_ImageHeight: c_ushort,
    pub prd_ImageWidth: c_ushort,
    pub prd_Total_bps: c_ushort,
    pub prd_Active_bps: c_ushort,
    pub prd_StorageMethod: c_ushort,
    pub prd_BayerPattern: c_ushort,
    pub SonyRawFileType: c_ushort,
    pub RAWFileType: c_ushort,
    pub RawSizeType: c_ushort,
    pub Quality: c_uint,
    pub FileFormat: c_ushort,
    pub MetaVersion: [c_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_fuji_info_t {
    pub ExpoMidPointShift: f32,
    pub DynamicRange: c_ushort,
    pub FilmMode: c_ushort,
    pub DynamicRangeSetting: c_ushort,
    pub DevelopmentDynamicRange: c_ushort,
    pub AutoDynamicRange: c_ushort,
    pub DRangePriority: c_ushort,
    pub DRangePriorityAuto: c_ushort,
    pub DRangePriorityFixed: c_ushort,
    pub BrightnessCompensation: f32,
    pub FocusMode: c_ushort,
    pub AFMode: c_ushort,
    pub FocusPixel: [c_ushort; 2usize],
    pub PrioritySettings: c_ushort,
    pub FocusSettings: c_uint,
    pub AF_C_Settings: c_uint,
    pub FocusWarning: c_ushort,
    pub ImageStabilization: [c_ushort; 3usize],
    pub FlashMode: c_ushort,
    pub WB_Preset: c_ushort,
    pub ShutterType: c_ushort,
    pub ExrMode: c_ushort,
    pub Macro: c_ushort,
    pub Rating: c_uint,
    pub CropMode: c_ushort,
    pub SerialSignature: [c_char; 13usize],
    pub SensorID: [c_char; 5usize],
    pub RAFVersion: [c_char; 5usize],
    pub RAFDataGeneration: c_int,
    pub RAFDataVersion: c_ushort,
    pub isTSNERDTS: c_int,
    pub DriveMode: c_short,
    pub BlackLevel: [c_ushort; 9usize],
    pub RAFData_ImageSizeTable: [c_uint; 32usize],
    pub AutoBracketing: c_int,
    pub SequenceNumber: c_int,
    pub SeriesLength: c_int,
    pub PixelShiftOffset: [f32; 2usize],
    pub ImageCount: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_metadata_common_t {
    pub FlashEC: f32,
    pub FlashGN: f32,
    pub CameraTemperature: f32,
    pub SensorTemperature: f32,
    pub SensorTemperature2: f32,
    pub LensTemperature: f32,
    pub AmbientTemperature: f32,
    pub BatteryTemperature: f32,
    pub exifAmbientTemperature: f32,
    pub exifHumidity: f32,
    pub exifPressure: f32,
    pub exifWaterDepth: f32,
    pub exifAcceleration: f32,
    pub exifCameraElevationAngle: f32,
    pub real_ISO: f32,
    pub exifExposureIndex: f32,
    pub ColorSpace: c_ushort,
    pub firmware: [c_char; 128usize],
    pub ExposureCalibrationShift: f32,
    pub afdata: [libraw_afinfo_item_t; 4usize],
    pub afcount: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_area_t {
    pub t: c_short,
    pub l: c_short,
    pub b: c_short,
    pub r: c_short,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_afinfo_item_t {
    pub AFInfoData_tag: c_uint,
    pub AFInfoData_order: c_short,
    pub AFInfoData_version: c_uint,
    pub AFInfoData_length: c_uint,
    pub AFInfoData: *mut c_uchar,
}
