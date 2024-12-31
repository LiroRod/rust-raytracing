fn main() {
    // Image size
    let image_width: u16 = 256;
    let image_height: u16 = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    /*
     *
     * this was the dumb way I was trying to do it
    let render_height = loop {
        if j < image_height {
            j += 1;
            let render_width = loop {
                if i < image_width {
                    i += 1;
                }
            };
        }
    };
    *
    * the following is the correct way
    */
    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f64) / (image_width - 1) as f64; // "as f64" casts the value type to
                                                           // be a floating-point f64
            let g = (j as f64) / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32; // "as i32" casts the value type
                                           // // back to an integer i32

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
