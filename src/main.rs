use core::time;

use futures_intrusive::timer;
// use futures_intrusive::buffer;
use wgpu::util::DeviceExt;
use bytemuck;

pub mod compute_shader;

// async fn run() 
// {
//     let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor 
//     {
//         backends: wgpu::Backends::all(),
//         ..Default::default()
//     });
//     let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions 
//     {
//         power_preference: wgpu::PowerPreference::default(),
//         compatible_surface: None,
//         force_fallback_adapter: false,
//     }).await.unwrap();

//     // let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();
//     let required_features = wgpu::Features::POLYGON_MODE_LINE | wgpu::Features::POLYGON_MODE_POINT;

//     let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor 
//     {
//         label: None,
//         required_features,
//         required_limits: wgpu::Limits::default(),
//         memory_hints: wgpu::MemoryHints::default(),
//     },
//     None,).await.unwrap();

//     let texture_size = 512u32;
//     let texture_desc = wgpu::TextureDescriptor 
//     {
//         size: wgpu::Extent3d 
//         {
//             width: texture_size,
//             height: texture_size,
//             depth_or_array_layers: 1,
//         },
//         mip_level_count: 1,
//         sample_count: 1,
//         dimension: wgpu::TextureDimension::D2,
//         format: wgpu::TextureFormat::Rgba8UnormSrgb,
//         usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
//         label: None,
//         view_formats: &[],
//     };
//     let texture = device.create_texture(&texture_desc);
//     let texture_view = texture.create_view(&Default::default());

//     let output_buffer = device.create_buffer(&wgpu::BufferDescriptor 
//     {
//         size: (4 * texture_size * texture_size) as wgpu::BufferAddress,
//         usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
//         label: None,
//         mapped_at_creation: false,
//     });

//     let vs_src = include_str!("shader.vert");
//     let fs_src = include_str!("shader.frag");
//     let compiler = shaderc::Compiler::new().unwrap();
//     let vs_spirv = compiler.compile_into_spirv(vs_src, shaderc::ShaderKind::Vertex, "shader.vert", "main", None).unwrap();
//     let fs_spirv = compiler.compile_into_spirv(fs_src, shaderc::ShaderKind::Fragment, "shader.frag", "main", None).unwrap();
//     let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor 
//     {
//         label: Some("Vertex Shader"),
//         source: wgpu::util::make_spirv(vs_spirv.as_binary_u8()),
//     });
//     let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor 
//     {
//         label: Some("Fragment Shader"),
//         source: wgpu::util::make_spirv(fs_spirv.as_binary_u8()),
//     });

//     let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor 
//     {
//         label: Some("Render Pipeline Layout"),
//         bind_group_layouts: &[],
//         push_constant_ranges: &[],
//     });

//     let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor 
//     {
//         label: Some("Render Pipeline"),
//         layout: Some(&render_pipeline_layout),
//         vertex: wgpu::VertexState 
//         {
//             module: &vs_module,
//             entry_point: Some("main"),
//             buffers: &[],
//             compilation_options: Default::default(),
//         },
//         fragment: Some(wgpu::FragmentState 
//         {
//             module: &fs_module,
//             entry_point: Some("main"),
//             targets: &[Some(wgpu::ColorTargetState 
//             {
//                 format: texture_desc.format,
//                 blend: Some(wgpu::BlendState 
//                 {
//                     alpha: wgpu::BlendComponent::REPLACE,
//                     color: wgpu::BlendComponent::REPLACE,
//                 }),
//                 write_mask: wgpu::ColorWrites::ALL,
//             })],
//             compilation_options: Default::default(),
//         }),
//         primitive: wgpu::PrimitiveState 
//         {
//             topology: wgpu::PrimitiveTopology::TriangleList,
//             strip_index_format: None,
//             front_face: wgpu::FrontFace::Ccw,
//             cull_mode: Some(wgpu::Face::Back),
//             polygon_mode: wgpu::PolygonMode::Fill,
//             unclipped_depth: false,
//             conservative: false,
//         },
//         depth_stencil: None,
//         multisample: wgpu::MultisampleState 
//         {
//             count: 1,
//             mask: !0,
//             alpha_to_coverage_enabled: false,
//         },
//         multiview: None,
//         cache: None,
//     });

//     let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//     {
//         let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor 
//         {
//             label: Some("Render Pass"),
//             color_attachments: &[Some(wgpu::RenderPassColorAttachment 
//             {
//                 view: &texture_view,
//                 resolve_target: None,
//                 ops: wgpu::Operations 
//                 {
//                     load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.0, b: 1.0, a: 1.0 }),
//                     store: wgpu::StoreOp::Store,
//                 },
//             })],
//             depth_stencil_attachment: None,
//             occlusion_query_set: None,
//             timestamp_writes: None,
//         });
//         render_pass.set_pipeline(&render_pipeline);
//         render_pass.draw(0..3, 0..1);
//     }

//     encoder.copy_texture_to_buffer(
//         wgpu::TexelCopyTextureInfo
//         {
//             aspect: wgpu::TextureAspect::All,
//             texture: &texture,
//             mip_level: 0,
//             origin: wgpu::Origin3d::ZERO,
//         },
//         wgpu::TexelCopyBufferInfo 
//         {
//             buffer: &output_buffer,
//             layout: wgpu::TexelCopyBufferLayout 
//             {
//                 offset: 0,
//                 bytes_per_row: Some(4 * texture_size),
//                 rows_per_image: Some(texture_size),
//             },
//         },
//         texture_desc.size,
//     );

//     queue.submit(Some(encoder.finish()));

//     let buffer_slice = output_buffer.slice(..);
//     let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
//     buffer_slice.map_async(wgpu::MapMode::Read, move |result| { tx.send(result).unwrap(); });
//     device.poll(wgpu::Maintain::Wait);
//     rx.receive().await.unwrap().unwrap();

//     let data = buffer_slice.get_mapped_range();
//     use image::{ImageBuffer, Rgba};
//     ImageBuffer::<Rgba<u8>, _>::from_raw(texture_size, texture_size, data).unwrap().save("image.png").unwrap();
//     output_buffer.unmap();
// }




// async fn run_compute_shader()
// {
//     let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor 
//     {
//         backends: wgpu::Backends::all(),
//         ..Default::default()
//     });

//     let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions 
//     {
//         power_preference: wgpu::PowerPreference::HighPerformance,
//         compatible_surface: None,
//         force_fallback_adapter: false,
//     }).await.unwrap();

//     let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();

//     // let input_data: Vec<u32> = (1..=15000000).collect();//vec![1, 2, 3, 4, 5];
//     let input_data: Vec<f32> = (1..=15000000).map(|x| x as f32).collect();

//     let workgroup_size = 128;
//     let num_values = input_data.len() as u32;
//     // let num_dispatches = (num_values + workgroup_size - 1) / workgroup_size;

//     let buffer_size = (input_data.len() * std::mem::size_of::<u32>()) as wgpu::BufferAddress;

//     // Input Buffer (Read Only Storage)
//     let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor
//     {   
//         label: Some("Storage Buffer"),
//         contents: bytemuck::cast_slice(&input_data),
//         usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
//     });

//     // Output Buffer (Read and Write Storage)
//     let output_buffer = device.create_buffer(&wgpu::BufferDescriptor 
//     {
//         label: Some("Output Buffer"),
//         size: buffer_size,
//         usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
//         mapped_at_creation: false,
//     });

//     let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor 
//     {
//         label: Some("Readback Buffer"),
//         size: buffer_size,
//         usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
//         mapped_at_creation: false,
//     });
//     let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor 
//     {
//         label: Some("Compute Shader"),
//         source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
//     });

//     let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor 
//     {
//         label: Some("Bind Group Layout"),
//         entries: &[wgpu::BindGroupLayoutEntry //Input
//         {
//             binding: 0,
//             visibility: wgpu::ShaderStages::COMPUTE,
//             ty: wgpu::BindingType::Buffer {
//                 ty: wgpu::BufferBindingType::Storage { read_only: true }, //Input-data read only
//                 has_dynamic_offset: false,
//                 min_binding_size: None,
//             },
//             count: None,
//         },
//         wgpu::BindGroupLayoutEntry //Output
//         {
//             binding: 1,
//             visibility: wgpu::ShaderStages::COMPUTE,
//             ty: wgpu::BindingType::Buffer {
//                 ty: wgpu::BufferBindingType::Storage { read_only: false }, //Output-data read and write
//                 has_dynamic_offset: false,
//                 min_binding_size: None,
//             },
//             count: None,
//         }],
//     });

//     let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor 
//     {
//         label: Some("Bind Group"),
//         layout: &bind_group_layout,
//         entries: &[wgpu::BindGroupEntry //Input
//         {
//             binding: 0,
//             resource: input_buffer.as_entire_binding(),
//         },
//         wgpu::BindGroupEntry //Output
//         {
//             binding: 1,
//             resource: output_buffer.as_entire_binding(),
//         }],
//     });

//     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor 
//     {
//         label: Some("Compute Pipeline Layout"),
//         bind_group_layouts: &[&bind_group_layout],
//         push_constant_ranges: &[],
//     });

//     let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor 
//     {
//         label: Some("Compute Pipeline"),
//         layout: Some(&pipeline_layout),
//         module: &shader_module, 
//         entry_point: Some("main"),
//         compilation_options: Default::default(),
//         cache: None
//     });

//     let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor 
//     {
//         label: Some("Compute Encoder"),
//     });

//     {
//         let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor 
//         {
//             label: Some("Compute Pass"),
//             timestamp_writes: None  
//         });
    
//         compute_pass.set_pipeline(&compute_pipeline);
//         compute_pass.set_bind_group(0, &bind_group, &[]);
    
//         // let workgroups = (num_values + workgroup_size - 1) / workgroup_size;
//         let workgroups_x = 65535;
//         let workgroups_y = (num_values + workgroup_size - 1) / workgroups_x;
//         compute_pass.dispatch_workgroups(workgroups_x, workgroups_y, 1); //compute_pass.dispatch_workgroups(workgroups, 1, 1);
//     }

//     encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback_buffer, 0, buffer_size);

//     queue.submit(Some(encoder.finish()));
//     let buffer_slice = readback_buffer.slice(..);
//     let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
//     buffer_slice.map_async(wgpu::MapMode::Read, move |result| 
//     {
//         tx.send(result).unwrap();
//     });

//     device.poll(wgpu::Maintain::Wait);

//     rx.receive().await.unwrap().unwrap();

//     let data = buffer_slice.get_mapped_range();
//     let result: &[f32] = bytemuck::cast_slice(&data);
//     println!("{:?}", result);
//     println!()
// }




fn main() 
{
    // pollster::block_on(run());
    println!("Start");
    let start = std::time::Instant::now();
    let input_data: Vec<f32> = (1..=15000000).map(|x| x as f32).collect();
    let (test, test2) = pollster::block_on(compute_shader::run_compute_shader(&input_data));
    println!("{:?}", test);
    println!("1: {}\n2: {}", test.len(), test2.len());


    // let input_data: Vec<u32> = (1..=15000000).collect();
    // let input_data: Vec<f32> = (1..=15000000).map(|x| x as f32).collect();
    // let mut output_data: Vec<f32> = vec![0.0; input_data.len()];

    // for (i, value) in input_data.iter().enumerate()
    // {
    //     output_data[i] = value*1.0001*value.sin();
    // }
    // print!("{:?}", output_data);
    // println!(); 

    
    let duration = start.elapsed().as_millis();
    println!("Total Time: {}ms", duration);
}