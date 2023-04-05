use crate::prelude::*;
use crate::vk;
use crate::{Device, Entry, Instance};
use std::ffi::CStr;
use std::mem;
use std::ptr;

/// High-level device function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_performance_query.html>
#[derive(Clone)]
pub struct PerformanceQueryDevice {
    handle: vk::Device,
    fp: vk::KhrPerformanceQueryDeviceFn,
}

impl PerformanceQueryDevice {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrPerformanceQueryDeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html>
    #[inline]
    pub unsafe fn acquire_profiling_lock(
        &self,
        info: &vk::AcquireProfilingLockInfoKHR,
    ) -> VkResult<()> {
        (self.fp.acquire_profiling_lock_khr)(self.handle, info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html>
    #[inline]
    pub unsafe fn release_profiling_lock(&self) {
        (self.fp.release_profiling_lock_khr)(self.handle)
    }

    pub const NAME: &'static CStr = vk::KhrPerformanceQueryDeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrPerformanceQueryDeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}

/// High-level instance function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_performance_query.html>
#[derive(Clone)]
pub struct PerformanceQueryInstance {
    fp: vk::KhrPerformanceQueryInstanceFn,
}

impl PerformanceQueryInstance {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let fp = vk::KhrPerformanceQueryInstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// Retrieve the number of elements to pass to [`enumerate_physical_device_queue_family_performance_query_counters()`][Self::enumerate_physical_device_queue_family_performance_query_counters()]
    #[inline]
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_len(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> VkResult<usize> {
        let mut count = 0;
        (self
            .fp
            .enumerate_physical_device_queue_family_performance_query_counters_khr)(
            physical_device,
            queue_family_index,
            &mut count,
            ptr::null_mut(),
            ptr::null_mut(),
        )
        .result_with_success(count as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html>
    ///
    /// Call [`enumerate_physical_device_queue_family_performance_query_counters_len()`][Self::enumerate_physical_device_queue_family_performance_query_counters_len()] to query the number of elements to pass to `out_counters` and `out_counter_descriptions`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        out_counters: &mut [vk::PerformanceCounterKHR],
        out_counter_descriptions: &mut [vk::PerformanceCounterDescriptionKHR],
    ) -> VkResult<()> {
        assert_eq!(out_counters.len(), out_counter_descriptions.len());
        let mut count = out_counters.len() as u32;
        (self
            .fp
            .enumerate_physical_device_queue_family_performance_query_counters_khr)(
            physical_device,
            queue_family_index,
            &mut count,
            out_counters.as_mut_ptr(),
            out_counter_descriptions.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out_counters.len());
        assert_eq!(count as usize, out_counter_descriptions.len());
        Ok(())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_queue_family_performance_query_passes(
        &self,
        physical_device: vk::PhysicalDevice,
        performance_query_create_info: &vk::QueryPoolPerformanceCreateInfoKHR,
    ) -> u32 {
        let mut num_passes = 0;
        (self
            .fp
            .get_physical_device_queue_family_performance_query_passes_khr)(
            physical_device,
            performance_query_create_info,
            &mut num_passes,
        );
        num_passes
    }

    pub const NAME: &'static CStr = vk::KhrPerformanceQueryInstanceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrPerformanceQueryInstanceFn {
        &self.fp
    }
}
