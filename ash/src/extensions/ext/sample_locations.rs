use crate::vk;
use crate::{Device, Entry, Instance};
use std::ffi::CStr;
use std::mem;

/// High-level device function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_sample_locations.html>
#[derive(Clone)]
pub struct SampleLocationsDevice {
    fp: vk::ExtSampleLocationsDeviceFn,
}

impl SampleLocationsDevice {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let fp = vk::ExtSampleLocationsDeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_locations(
        &self,
        command_buffer: vk::CommandBuffer,
        sample_locations_info: &vk::SampleLocationsInfoEXT,
    ) {
        (self.fp.cmd_set_sample_locations_ext)(command_buffer, sample_locations_info)
    }

    pub const NAME: &'static CStr = vk::ExtSampleLocationsDeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtSampleLocationsDeviceFn {
        &self.fp
    }
}

/// High-level instance function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_sample_locations.html>
#[derive(Clone)]
pub struct SampleLocationsInstance {
    fp: vk::ExtSampleLocationsInstanceFn,
}

impl SampleLocationsInstance {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let fp = vk::ExtSampleLocationsInstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_multisample_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        samples: vk::SampleCountFlags,
        multisample_properties: &mut vk::MultisamplePropertiesEXT,
    ) {
        (self.fp.get_physical_device_multisample_properties_ext)(
            physical_device,
            samples,
            multisample_properties,
        )
    }

    pub const NAME: &'static CStr = vk::ExtSampleLocationsInstanceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtSampleLocationsInstanceFn {
        &self.fp
    }
}
