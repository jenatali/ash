use crate::prelude::*;
use crate::vk;
use crate::{Device, Entry, Instance};
use std::ffi::CStr;
use std::mem;

/// High-level device function wrapper for
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html>
#[derive(Clone)]
pub struct CalibratedTimestampsDevice {
    handle: vk::Device,
    fp: vk::ExtCalibratedTimestampsDeviceFn,
}

impl CalibratedTimestampsDevice {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ExtCalibratedTimestampsDeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html>
    ///
    /// Returns a tuple containing `(timestamps, max_deviation)`
    #[inline]
    pub unsafe fn get_calibrated_timestamps(
        &self,
        info: &[vk::CalibratedTimestampInfoEXT],
    ) -> VkResult<(Vec<u64>, Vec<u64>)> {
        let mut timestamps = vec![0u64; info.len()];
        let mut max_deviation = vec![0u64; info.len()];
        (self.fp.get_calibrated_timestamps_ext)(
            self.handle,
            info.len() as u32,
            info.as_ptr(),
            timestamps.as_mut_ptr(),
            max_deviation.as_mut_ptr(),
        )
        .result_with_success((timestamps, max_deviation))
    }

    pub const NAME: &'static CStr = vk::ExtCalibratedTimestampsDeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtCalibratedTimestampsDeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}

/// High-level instance function wrapper for
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html>
#[derive(Clone)]
pub struct CalibratedTimestampsInstance {
    fp: vk::ExtCalibratedTimestampsInstanceFn,
}

impl CalibratedTimestampsInstance {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::ExtCalibratedTimestampsInstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_calibrateable_time_domains(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::TimeDomainEXT>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_calibrateable_time_domains_ext)(
                physical_device,
                count,
                data,
            )
        })
    }

    pub const NAME: &'static CStr = vk::ExtCalibratedTimestampsInstanceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtCalibratedTimestampsInstanceFn {
        &self.fp
    }
}
