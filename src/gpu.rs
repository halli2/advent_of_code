use core::slice;

use ash::vk;

/// Casts reference slice of a type `T` into a slice of bytes.
///
/// # Safety
/// Uses `std::slice::from_raw_parts`
pub const unsafe fn cast_bytes<T>(data: &[T]) -> &[u8] {
    std::slice::from_raw_parts(
        (data as *const [T]).cast::<u8>(),
        data.len() * std::mem::size_of::<T>(),
    )
}
/// Casts reference slice of a type `T` into a slice of bytes.
///
/// # Safety
/// Uses `std::slice::from_raw_parts`
pub const unsafe fn cast_bytes_from_type<T>(data: &T) -> &[u8] {
    std::slice::from_raw_parts((data as *const T).cast::<u8>(), std::mem::size_of::<T>())
}

/// Casts reference slice of a type `u8` into a slice of `T`.
///
/// # Safety
/// Uses `std::slice::from_raw_parts`
pub const unsafe fn cast_slice<T>(data: &[u8]) -> &[T] {
    std::slice::from_raw_parts(
        (data as *const [u8]).cast::<T>(),
        data.len() / std::mem::size_of::<T>(),
    )
}

pub const fn get_slice_size<T>(data: &[T]) -> usize {
    data.len() * std::mem::size_of::<T>()
}

// pub struct Context {
//     pub adapter: wgpu::Adapter,
//     pub device: wgpu::Device,
//     pub instance: wgpu::Instance,
//     pub surface: Option<wgpu::Surface>,
//     pub queue: wgpu::Queue,
// }

// impl Context {
//     pub async fn new(window: Option<&winit::window::Window>) -> Option<Self> {
//         let instance = wgpu::Instance::new(wgpu::Backends::all());
//         let surface = window.map(|w| unsafe { instance.create_surface(&w) });

//         let adapter = instance
//             .request_adapter(&wgpu::RequestAdapterOptions {
//                 compatible_surface: surface.as_ref(),
//                 ..Default::default()
//             })
//             .await?;

//         let (device, queue) = adapter
//             .request_device(
//                 &wgpu::DeviceDescriptor {
//                     label: None,
//                     features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
//                     limits: wgpu::Limits::default(),
//                 },
//                 None,
//             )
//             .await
//             .unwrap();

//         Some(Context {
//             adapter,
//             device,
//             instance,
//             surface,
//             queue,
//         })
//     }

//     pub fn compute_pipeline(
//         &self,
//         shader_code: Cow<'_, str>,
//         bindgroup_entries: &[wgpu::BindGroupEntry],
//     ) -> ComputePipeline {
//         let shader = self
//             .device
//             .create_shader_module(wgpu::ShaderModuleDescriptor {
//                 label: None,
//                 source: wgpu::ShaderSource::Wgsl(shader_code),
//             });

//         let pipeline = self
//             .device
//             .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//                 label: None,
//                 layout: None,
//                 module: &shader,
//                 entry_point: "main",
//             });

//         let bind_group_layout = pipeline.get_bind_group_layout(0);
//         let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label: None,
//             layout: &bind_group_layout,
//             entries: bindgroup_entries,
//         });

//         ComputePipeline {
//             raw: pipeline,
//             bind_group,
//         }
//     }
// }

// pub struct ComputePipeline {
//     pub raw: wgpu::ComputePipeline,
//     pub bind_group: wgpu::BindGroup,
// }

// pub struct Viewer {
//     event_loop: winit::event_loop::EventLoop<()>,
//     pub window: winit::window::Window,
//     pub ctx: Context,

//     blit_pipeline: wgpu::RenderPipeline,
//     blit_bind_group: wgpu::BindGroup,

//     pub texture: wgpu::Texture,
//     pub texture_view: wgpu::TextureView,
//     pub sampler: wgpu::Sampler,
// }

// impl<'a> Viewer {
//     pub async fn new(texture_size: (u32, u32)) -> color_eyre::Result<Self> {
//         let event_loop = winit::event_loop::EventLoop::new();
//         let window = winit::window::WindowBuilder::new()
//             .with_inner_size(winit::dpi::PhysicalSize::new(1080, 1080))
//             .build(&event_loop)?;

//         let ctx = Context::new(Some(&window)).await.unwrap();

//         let surface = ctx.surface.as_ref().unwrap();
//         let swapchain_format = surface.get_supported_formats(&ctx.adapter)[0];
//         let alpha_mode = surface.get_supported_alpha_modes(&ctx.adapter)[0];

//         let shader = ctx
//             .device
//             .create_shader_module(wgpu::ShaderModuleDescriptor {
//                 label: None,
//                 source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/blit.wgsl"))),
//             });
//         let pipeline = ctx
//             .device
//             .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//                 label: Some("Blit"),
//                 layout: None,
//                 vertex: wgpu::VertexState {
//                     module: &shader,
//                     entry_point: "vs_main",
//                     buffers: &[],
//                 },
//                 fragment: Some(wgpu::FragmentState {
//                     module: &shader,
//                     entry_point: "fs_main",
//                     targets: &[Some(swapchain_format.into())],
//                 }),
//                 primitive: wgpu::PrimitiveState {
//                     topology: wgpu::PrimitiveTopology::TriangleList,
//                     ..Default::default()
//                 },
//                 depth_stencil: None,
//                 multisample: wgpu::MultisampleState::default(),
//                 multiview: None,
//             });

//         let texture_extent = wgpu::Extent3d {
//             width: texture_size.0,
//             height: texture_size.1,
//             depth_or_array_layers: 1,
//         };
//         let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
//             label: None,
//             size: texture_extent,
//             mip_level_count: 1,
//             sample_count: 1,
//             dimension: wgpu::TextureDimension::D2,
//             format: wgpu::TextureFormat::Rgba8Unorm,
//             usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
//         });
//         let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

//         let avail_present_mode = surface.get_supported_present_modes(&ctx.adapter);
//         let present_mode = if avail_present_mode.contains(&wgpu::PresentMode::Mailbox) {
//             wgpu::PresentMode::Mailbox
//         } else {
//             wgpu::PresentMode::Fifo
//         };
//         let config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: swapchain_format,
//             width: 1080,
//             height: 1080,
//             present_mode,
//             alpha_mode,
//         };

//         surface.configure(&ctx.device, &config);

//         let bind_group_layout = pipeline.get_bind_group_layout(0);
//         let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
//             label: Some("blit"),
//             address_mode_u: wgpu::AddressMode::ClampToEdge,
//             address_mode_v: wgpu::AddressMode::ClampToEdge,
//             address_mode_w: wgpu::AddressMode::ClampToEdge,
//             mag_filter: wgpu::FilterMode::Nearest,
//             min_filter: wgpu::FilterMode::Nearest,
//             ..Default::default()
//         });

//         let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label: None,
//             layout: &bind_group_layout,
//             entries: &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: wgpu::BindingResource::TextureView(&texture_view),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: wgpu::BindingResource::Sampler(&sampler),
//                 },
//             ],
//         });

//         Ok(Self {
//             event_loop,
//             window,
//             ctx,
//             blit_pipeline: pipeline,
//             blit_bind_group: bind_group,
//             texture,
//             texture_view,
//             sampler,
//         })
//     }

//     pub fn run<F: Fn(&wgpu::Device, &wgpu::Queue) + 'static>(self, compute_pass: F) {
//         self.event_loop.run(move |event, _, control_flow| {
//             *control_flow = winit::event_loop::ControlFlow::Poll;
//             match event {
//                 winit::event::Event::WindowEvent {
//                     event: winit::event::WindowEvent::CloseRequested,
//                     ..
//                 } => {
//                     *control_flow = winit::event_loop::ControlFlow::Exit;
//                 }
//                 winit::event::Event::MainEventsCleared => {
//                     compute_pass(&self.ctx.device, &self.ctx.queue);
//                     let frame = self
//                         .ctx
//                         .surface
//                         .as_ref()
//                         .unwrap()
//                         .get_current_texture()
//                         .unwrap();
//                     let view = frame
//                         .texture
//                         .create_view(&wgpu::TextureViewDescriptor::default());
//                     let mut encoder = self
//                         .ctx
//                         .device
//                         .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//                     {
//                         let mut render_pass =
//                             encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                                 label: None,
//                                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                                     view: &view,
//                                     resolve_target: None,
//                                     ops: wgpu::Operations {
//                                         load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
//                                         store: true,
//                                     },
//                                 })],
//                                 depth_stencil_attachment: None,
//                             });
//                         render_pass.set_pipeline(&self.blit_pipeline);
//                         render_pass.set_bind_group(0, &self.blit_bind_group, &[]);
//                         render_pass.draw(0..3, 0..1);
//                     }
//                     self.ctx.queue.submit(Some(encoder.finish()));
//                     frame.present();
//                 }
//                 _ => (),
//             }
//         });
//     }
// }

// #[cfg(test)]
// mod tests {
//     use wgpu::util::DeviceExt;

//     use super::*;

//     async fn compute_shader_run(
//         ctx: Context,
//         workgroup_x: u32,
//         size: wgpu::BufferAddress,
//         pipeline: ComputePipeline,
//         dst_buffer: wgpu::Buffer,
//         staging_buffer: wgpu::Buffer,
//     ) -> Vec<u32> {
//         let mut encoder = ctx
//             .device
//             .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//         {
//             let mut cpass =
//                 encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
//             cpass.set_pipeline(&pipeline.raw);
//             cpass.set_bind_group(0, &pipeline.bind_group, &[]);
//             cpass.insert_debug_marker("compute +2");
//             cpass.dispatch_workgroups(workgroup_x, 1, 1);
//         }
//         encoder.copy_buffer_to_buffer(&dst_buffer, 0, &staging_buffer, 0, size);

//         ctx.queue.submit(Some(encoder.finish()));

//         let buffer_slice = staging_buffer.slice(..);
//         let (sender, reciever) = tokio::sync::oneshot::channel();
//         buffer_slice.map_async(wgpu::MapMode::Read, |v| sender.send(v).unwrap());

//         // Poll in a blocking manner, so our future resolves. in actual app `device.poll(..)`
//         // should be called in an event loop or another thread!
//         ctx.device.poll(wgpu::Maintain::Wait);

//         match reciever.await {
//             Ok(_) => {
//                 let data = buffer_slice.get_mapped_range();
//                 let result = unsafe { cast_slice(&data).to_vec() };
//                 drop(data);
//                 staging_buffer.unmap();
//                 result
//             }

//             Err(err) => panic!("{:?}", err),
//         }
//     }

//     #[test]
//     fn compute_shader() {
//         let shader = Cow::Borrowed(
//             "
// @group(0)
// @binding(0)
// var<storage, read> src_buffer: array<u32>;

// @group(0)
// @binding(1)
// var<storage, write> dst_buffer: array<u32>;

// @compute
// @workgroup_size(1, 1, 1)
// fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
//     dst_buffer[global_id.x] = src_buffer[global_id.x] + u32(2);
// }
// ",
//         );
//         let data = [2, 4, 6, 8];
//         let ctx = pollster::block_on(Context::new(None)).unwrap();
//         let size = get_slice_size(&data) as wgpu::BufferAddress;
//         let device = &ctx.device;
//         let src_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: None,
//             contents: unsafe { cast_bytes(&data) },
//             usage: wgpu::BufferUsages::STORAGE,
//         });

//         let dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label: None,
//             size,
//             usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
//             mapped_at_creation: false,
//         });

//         let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label: None,
//             size: 1024, // 1 MB
//             usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
//             mapped_at_creation: false,
//         });

//         let compute_pipeline = ctx.compute_pipeline(
//             shader,
//             &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: src_buffer.as_entire_binding(),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: dst_buffer.as_entire_binding(),
//                 },
//             ],
//         );

//         let result = pollster::block_on(compute_shader_run(
//             ctx,
//             data.len() as u32,
//             get_slice_size(&data) as wgpu::BufferAddress,
//             compute_pipeline,
//             dst_buffer,
//             staging_buffer,
//         ));
//         let mut t = Vec::new();
//         for (i, inp) in data.into_iter().enumerate() {
//             t.push(result[i]);
//             assert_eq!(result[i], inp + 2);
//         }
//     }
// }

pub struct Gpu {
    pub device: ash::Device,
    pub queue: vk::Queue,
    pub family: u32,
    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,
    pub timeline_semaphore: vk::Semaphore,

    pub instance: ash::Instance,
    pub entry: ash::Entry,
}

impl Drop for Gpu {
    fn drop(&mut self) {
        unsafe {
            let device = &self.device;
            device
                .reset_command_pool(self.command_pool, vk::CommandPoolResetFlags::empty())
                .unwrap();
            device.destroy_semaphore(self.timeline_semaphore, None);
            device.destroy_command_pool(self.command_pool, None);
            device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

impl Gpu {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new() -> color_eyre::Result<Self> {
        let entry = ash::Entry::load()?;
        let instance = entry.create_instance(
            &vk::InstanceCreateInfo::builder()
                .application_info(&vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_3)),
            None,
        )?;

        let physical_device = instance.enumerate_physical_devices()?[0]; // Pray the first is the right :p
        let queue_properties =
            instance.get_physical_device_queue_family_properties(physical_device);
        let (family, _queue_props) = queue_properties
            .iter()
            .filter(|f| f.queue_count > 0)
            .enumerate()
            .find(|(_, family)| {
                family
                    .queue_flags
                    .contains(vk::QueueFlags::GRAPHICS & vk::QueueFlags::COMPUTE)
            })
            .unwrap();
        let family = family as u32;

        let mut features_vk_1_2 =
            vk::PhysicalDeviceVulkan12Features::builder().timeline_semaphore(true);
        let mut features_vk_1_3 = vk::PhysicalDeviceVulkan13Features::builder()
            .synchronization2(true)
            .dynamic_rendering(true);

        let mut device_features = vk::PhysicalDeviceFeatures2::builder()
            .push_next(&mut features_vk_1_2)
            .push_next(&mut features_vk_1_3);
        let queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(family)
            .queue_priorities(&[1.0]);

        let device = instance.create_device(
            physical_device,
            &vk::DeviceCreateInfo::builder()
                .queue_create_infos(slice::from_ref(&queue_info))
                .push_next(&mut device_features),
            None,
        )?;
        let queue = device.get_device_queue(family, 0);
        let command_pool = device.create_command_pool(
            &vk::CommandPoolCreateInfo::builder().queue_family_index(family),
            None,
        )?;
        let command_buffer = device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::builder()
                .command_buffer_count(1)
                .command_pool(command_pool)
                .level(vk::CommandBufferLevel::PRIMARY),
        )?[0];
        let timeline_semaphore = device.create_semaphore(
            &vk::SemaphoreCreateInfo::builder().push_next(
                &mut vk::SemaphoreTypeCreateInfo::builder()
                    .semaphore_type(vk::SemaphoreType::TIMELINE)
                    .initial_value(0),
            ),
            None,
        )?;

        Ok(Self {
            device,
            queue,
            family,
            command_pool,
            command_buffer,
            timeline_semaphore,
            instance,
            entry,
        })
    }
}
