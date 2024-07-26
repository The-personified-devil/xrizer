use crate::vr;
use glam::{Mat3, Mat4, Quat, Vec3};
use openxr as xr;

pub fn space_relation_to_openvr_pose(
    location: xr::SpaceLocation,
    velocity: xr::SpaceVelocity,
) -> vr::TrackedDevicePose_t {
    if !location.location_flags.contains(
        xr::SpaceLocationFlags::POSITION_VALID | xr::SpaceLocationFlags::ORIENTATION_VALID,
    ) {
        return vr::TrackedDevicePose_t {
            bPoseIsValid: false,
            ..Default::default()
        };
    }

    let location = vr::HmdMatrix34_t::from(location.pose);
    let linear_velo = velocity
        .velocity_flags
        .contains(xr::SpaceVelocityFlags::LINEAR_VALID)
        .then(|| velocity.linear_velocity.into());
    let angular_velo = velocity
        .velocity_flags
        .contains(xr::SpaceVelocityFlags::ANGULAR_VALID)
        .then(|| velocity.angular_velocity.into());

    vr::TrackedDevicePose_t {
        mDeviceToAbsoluteTracking: location,
        vVelocity: linear_velo.unwrap_or_default(),
        vAngularVelocity: angular_velo.unwrap_or_default(),
        eTrackingResult: vr::ETrackingResult::TrackingResult_Running_OK,
        bPoseIsValid: true,
        bDeviceIsConnected: true,
    }
}

impl From<Mat4> for vr::HmdMatrix44_t {
    fn from(value: Mat4) -> Self {
        // OpenVR wants data in row major order, so we transpose it
        Self {
            m: value.transpose().to_cols_array_2d(),
        }
    }
}

impl From<xr::Vector3f> for vr::HmdVector3_t {
    fn from(value: xr::Vector3f) -> Self {
        Self {
            v: [value.x, value.y, value.z],
        }
    }
}

impl From<Vec3> for vr::HmdVector3_t {
    fn from(value: Vec3) -> Self {
        Self {
            v: value.to_array(),
        }
    }
}

impl From<Quat> for vr::HmdQuaternionf_t {
    fn from(value: Quat) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl From<xr::Posef> for vr::HmdMatrix34_t {
    fn from(pose: xr::Posef) -> Self {
        // openvr matrices are row major, glam matrices are column major

        let rot = Mat3::from_quat(Quat::from_xyzw(
            pose.orientation.x,
            pose.orientation.y,
            pose.orientation.z,
            pose.orientation.w,
        ))
        .transpose();

        let gen_array = |translation, rot_axis: Vec3| {
            std::array::from_fn(|i| if i == 3 { translation } else { rot_axis[i] })
        };

        Self {
            m: [
                gen_array(pose.position.x, rot.x_axis),
                gen_array(pose.position.y, rot.y_axis),
                gen_array(pose.position.z, rot.z_axis),
            ],
        }
    }
}
