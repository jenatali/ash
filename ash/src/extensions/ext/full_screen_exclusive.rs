use crate::prelude::*;
use crate::vk;
use crate::{Device, Entry, Instance};
use std::ffi::CStr;
use std::mem;

/// High-level device function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html>
#[derive(Clone)]
pub struct FullScreenExclusiveDevice {
    handle: vk::Device,
    fp: vk::ExtFullScreenExclusiveDeviceFn,
}

impl FullScreenExclusiveDevice {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ExtFullScreenExclusiveDeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html>
    #[inline]
    pub unsafe fn acquire_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<()> {
        (self.fp.acquire_full_screen_exclusive_mode_ext)(self.handle, swapchain).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html>
    #[inline]
    pub unsafe fn release_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<()> {
        (self.fp.release_full_screen_exclusive_mode_ext)(self.handle, swapchain).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html>
    #[inline]
    pub unsafe fn get_device_group_surface_present_modes2(
        &self,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> VkResult<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut present_modes = mem::zeroed();
        (self.fp.get_device_group_surface_present_modes2_ext)(
            self.handle,
            surface_info,
            &mut present_modes,
        )
        .result_with_success(present_modes)
    }

    pub const NAME: &'static CStr = vk::ExtFullScreenExclusiveDeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtFullScreenExclusiveDeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}

/// High-level instance function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html>
#[derive(Clone)]
pub struct FullScreenExclusiveInstance {
    fp: vk::ExtFullScreenExclusiveInstanceFn,
}

impl FullScreenExclusiveInstance {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::ExtFullScreenExclusiveInstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>
    #[inline]
    pub unsafe fn get_physical_device_surface_present_modes2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> VkResult<Vec<vk::PresentModeKHR>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_surface_present_modes2_ext)(
                physical_device,
                surface_info,
                count,
                data,
            )
        })
    }

    pub const NAME: &'static CStr = vk::ExtFullScreenExclusiveInstanceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtFullScreenExclusiveInstanceFn {
        &self.fp
    }
}
