pub mod compute_shader;

fn main() 
{
    println!("Start");
    let start = std::time::Instant::now();
    let input_data: Vec<f32> = (1..=33554432).map(|x| x as f32).collect();
    let test = pollster::block_on(compute_shader::compute_shader(&input_data, "src/shader.wgsl", false));
    println!("{:?}", test);

    // let input_data: Vec<f32> = (1..=33554432).map(|x| x as f32).collect();
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