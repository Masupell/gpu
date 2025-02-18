@group(0) @binding(0) var<storage, read> input_buffer: array<f32>;
@group(0) @binding(1) var<storage, read_write> output_buffer: array<f32>;

@compute @workgroup_size(128)
fn main(@builtin(global_invocation_id) id: vec3<u32>) 
{
    // let index = id.x; // 1D
    let index = id.x + id.y * 65535 * 128; // 2D
    if (u32(index) < arrayLength(&input_buffer))
    {
        output_buffer[index] = input_buffer[index]*1.0001*(sin(input_buffer[index] % (2.0 * 3.14159265359)));
    }
}