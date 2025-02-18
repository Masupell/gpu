use wgpu::util::DeviceExt;

pub async fn run_compute_shader(input: &[f32]) -> (Vec<f32>, Vec<f32>)
{
    let input_data = input;
    
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor 
    {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions 
    {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }).await.unwrap();

    let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();

    // let input_data: Vec<f32> = (1..=15000000).map(|x| x as f32).collect();

    let workgroup_size = 128;
    let num_values = input_data.len() as u32;

    let buffer_size = (input_data.len() * std::mem::size_of::<u32>()) as wgpu::BufferAddress;

    // Input Buffer (Read Only Storage)
    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor
    {   
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&input_data),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    // Output Buffer (Read and Write Storage)
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor 
    {
        label: Some("Output Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor 
    {
        label: Some("Readback Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor 
    {
        label: Some("Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor 
    {
        label: Some("Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry //Input
        {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true }, //Input-data read only
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },
        wgpu::BindGroupLayoutEntry //Output
        {
            binding: 1,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false }, //Output-data read and write
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor 
    {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry //Input
        {
            binding: 0,
            resource: input_buffer.as_entire_binding(),
        },
        wgpu::BindGroupEntry //Output
        {
            binding: 1,
            resource: output_buffer.as_entire_binding(),
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor 
    {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor 
    {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader_module, 
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor 
    {
        label: Some("Compute Encoder"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor 
        {
            label: Some("Compute Pass"),
            timestamp_writes: None  
        });
    
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
    
        // let workgroups = (num_values + workgroup_size - 1) / workgroup_size;
        let workgroups_x = 65535;
        let workgroups_y = (num_values + workgroup_size - 1) / workgroups_x;
        compute_pass.dispatch_workgroups(workgroups_x, workgroups_y, 1); //compute_pass.dispatch_workgroups(workgroups, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback_buffer, 0, buffer_size);

    queue.submit(Some(encoder.finish()));
    let buffer_slice = readback_buffer.slice(..);
    let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| 
    {
        tx.send(result).unwrap();
    });

    device.poll(wgpu::Maintain::Wait);

    rx.receive().await.unwrap().unwrap();

    let data = buffer_slice.get_mapped_range();
    let result: &[f32] = bytemuck::cast_slice(&data);
    (result.to_vec(), input_data.to_vec())
}