#[cfg(doc)]
use super::{DeviceGroupDevice, DeviceGroupInstance};
use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Device, Entry, Instance};
use std::ffi::CStr;
use std::mem;

/// High-level device function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html>
#[derive(Clone)]
pub struct SwapchainDevice {
    handle: vk::Device,
    fp: vk::KhrSwapchainDeviceFn,
}

impl SwapchainDevice {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrSwapchainDeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html>
    #[inline]
    pub unsafe fn create_swapchain(
        &self,
        create_info: &vk::SwapchainCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SwapchainKHR> {
        let mut swapchain = mem::zeroed();
        (self.fp.create_swapchain_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut swapchain,
        )
        .result_with_success(swapchain)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html>
    #[inline]
    pub unsafe fn destroy_swapchain(
        &self,
        swapchain: vk::SwapchainKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp.destroy_swapchain_khr)(self.handle, swapchain, allocation_callbacks.as_raw_ptr());
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html>
    #[inline]
    pub unsafe fn get_swapchain_images(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<Vec<vk::Image>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_swapchain_images_khr)(self.handle, swapchain, count, data)
        })
    }

    /// On success, returns the next image's index and whether the swapchain is suboptimal for the surface.
    ///
    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html>
    #[inline]
    pub unsafe fn acquire_next_image(
        &self,
        swapchain: vk::SwapchainKHR,
        timeout: u64,
        semaphore: vk::Semaphore,
        fence: vk::Fence,
    ) -> VkResult<(u32, bool)> {
        let mut index = 0;
        let err_code = (self.fp.acquire_next_image_khr)(
            self.handle,
            swapchain,
            timeout,
            semaphore,
            fence,
            &mut index,
        );
        match err_code {
            vk::Result::SUCCESS => Ok((index, false)),
            vk::Result::SUBOPTIMAL_KHR => Ok((index, true)),
            _ => Err(err_code),
        }
    }

    /// On success, returns whether the swapchain is suboptimal for the surface.
    ///
    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html>
    #[inline]
    pub unsafe fn queue_present(
        &self,
        queue: vk::Queue,
        present_info: &vk::PresentInfoKHR,
    ) -> VkResult<bool> {
        let err_code = (self.fp.queue_present_khr)(queue, present_info);
        match err_code {
            vk::Result::SUCCESS => Ok(false),
            vk::Result::SUBOPTIMAL_KHR => Ok(true),
            _ => Err(err_code),
        }
    }

    /// Only available since [Vulkan 1.1].
    ///
    /// Also available as [`DeviceGroupDevice::get_device_group_present_capabilities()`]
    /// when [`VK_KHR_surface`] is enabled.
    ///
    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    ///
    /// [Vulkan 1.1]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_surface`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html
    #[inline]
    pub unsafe fn get_device_group_present_capabilities(
        &self,
        device_group_present_capabilities: &mut vk::DeviceGroupPresentCapabilitiesKHR,
    ) -> VkResult<()> {
        (self.fp.get_device_group_present_capabilities_khr)(
            self.handle,
            device_group_present_capabilities,
        )
        .result()
    }

    /// Only available since [Vulkan 1.1].
    ///
    /// Also available as [`DeviceGroupDevice::get_device_group_surface_present_modes()`]
    /// when [`VK_KHR_surface`] is enabled.
    ///
    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html>
    ///
    /// [Vulkan 1.1]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_surface`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html
    #[inline]
    pub unsafe fn get_device_group_surface_present_modes(
        &self,
        surface: vk::SurfaceKHR,
    ) -> VkResult<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut modes = mem::zeroed();
        (self.fp.get_device_group_surface_present_modes_khr)(self.handle, surface, &mut modes)
            .result_with_success(modes)
    }

    /// On success, returns the next image's index and whether the swapchain is suboptimal for the surface.
    ///
    /// Only available since [Vulkan 1.1].
    ///
    /// Also available as [`DeviceGroupDevice::acquire_next_image2()`]
    /// when [`VK_KHR_swapchain`] is enabled.
    ///
    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html>
    ///
    /// [Vulkan 1.1]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_swapchain`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html
    #[inline]
    pub unsafe fn acquire_next_image2(
        &self,
        acquire_info: &vk::AcquireNextImageInfoKHR,
    ) -> VkResult<(u32, bool)> {
        let mut index = 0;
        let err_code = (self.fp.acquire_next_image2_khr)(self.handle, acquire_info, &mut index);
        match err_code {
            vk::Result::SUCCESS => Ok((index, false)),
            vk::Result::SUBOPTIMAL_KHR => Ok((index, true)),
            _ => Err(err_code),
        }
    }

    pub const NAME: &'static CStr = vk::KhrSwapchainDeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrSwapchainDeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}

/// High-level instance function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html>
#[derive(Clone)]
pub struct SwapchainInstance {
    fp: vk::KhrSwapchainInstanceFn,
}

impl SwapchainInstance {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let fp = vk::KhrSwapchainInstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// Only available since [Vulkan 1.1].
    ///
    /// Also available as [`DeviceGroupInstance::get_physical_device_present_rectangles()`]
    /// when [`VK_KHR_surface`] is enabled.
    ///
    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html>
    ///
    /// [Vulkan 1.1]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_surface`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html
    #[inline]
    pub unsafe fn get_physical_device_present_rectangles(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Vec<vk::Rect2D>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_present_rectangles_khr)(
                physical_device,
                surface,
                count,
                data,
            )
        })
    }

    pub const NAME: &'static CStr = vk::KhrSwapchainInstanceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrSwapchainInstanceFn {
        &self.fp
    }
}
