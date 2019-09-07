use serde::{Deserialize, Serialize};

/// Rust structure representatation of .VRM json extensions.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmJson {
    exporter_version: Option<String>,
    meta: VrmMeta,
    humanoid: VrmHumanoid,
    first_person: VrmFirstPerson,
    blend_shape_master: VrmBlendShape,
    secondary_animation: VrmSecondaryAnimation,
    material_properties: Vec<VrmMaterial>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmMeta {
    /// Title of VRM model.
    title: String,
    /// Version of VRM model.
    version: String,
    /// Author(s) of VRM model.
    author: String,
    contact_information: String,
    reference: String,
    texture: usize,
    #[serde(rename = "allowedUserName")]
    allowed_users: VrmAllowedUser,
    #[serde(rename = "violentUssageName")]
    allow_violence: VrmAllowFlag,
    #[serde(rename = "sexualUssageName")]
    allow_sexual_use: VrmAllowFlag,
    #[serde(rename = "commercialUssageName")]
    allow_commercial_use: VrmAllowFlag,
    other_permission_url: Option<String>,
    license_name: VrmLicenseType,
    other_license_url: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VrmAllowedUser {
    OnlyAuthor,
    ExplicitlyLicensedPerson,
    Everyone,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VrmAllowFlag {
    Allow,
    Disallow,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VrmLicenseType {
    #[serde(rename = "Redistribution_Prohibited")]
    NoRedist,
    #[serde(rename = "CC_0")]
    CreativeCommonsZero,
    #[serde(rename = "CC_BY")]
    CreativeCommonsBy,
    #[serde(rename = "CC_BY_NC")]
    CreativeCommonsByNc,
    #[serde(rename = "CC_BY_SA")]
    CreativeCommonsBySa,
    #[serde(rename = "CC_BY_NC_SA")]
    CreativeCommonsByNcSa,
    #[serde(rename = "CC_BY_ND")]
    CreativeCommonsByNd,
    #[serde(rename = "CC_BY_NC_ND")]
    CreativeCommonsByNcNd,
    Other,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmHumanoid {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmFirstPerson {
    first_person_bone: usize,
    first_person_bone_offset: Vec3d,
    mesh_annotations: Vec<()>,
    #[serde(rename = "lookAtTypeName")]
    eye_mode: String, // Bone, BlendShape
    #[serde(rename = "lookAtHorizontalInner")]
    horizontal_inner: VrmDegreeMap,
    #[serde(rename = "lookAtHorizontalOuter")]
    horizontal_outer: VrmDegreeMap,
    #[serde(rename = "lookAtVerticalDown")]
    vertical_down: VrmDegreeMap,
    #[serde(rename = "lookAtVerticalUp")]
    vertical_up: VrmDegreeMap,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmDegreeMap {
    curve: Vec<f64>,
    x_range: f64,
    y_range: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmBlendShape {
    blend_shape_groups: Vec<VrmBlendShapeGroup>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmBlendShapeGroup {
    name: String,
    preset_name: String,
    binds: Vec<VrmBind>,
    material_values: Vec<VrmMaterialBind>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmBind {
    mesh: i64,
    index: i64,
    weight: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmMaterialBind {
    material_name: String,
    property_name: String,
    target_value: Vec<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmSecondaryAnimation {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmMaterial {
    name: String,
    shader: String,
    render_queue: usize,
}

/*
{
    "title": "vrm.material",
    "type": "object",
    "properties": {
        "name": {
            "type": "string"
        },
        "shader": {
            "type": "string"
        },
        "renderQueue": {
            "type": "integer"
        },
        "floatProperties": {
            "type": "object"
        },
        "vectorProperties": {
            "type": "object"
        },
        "textureProperties": {
            "type": "object"
        },
        "keywordMap": {
            "type": "object"
        },
        "tagMap": {
            "type": "object"
        }
    }
}*/

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}
