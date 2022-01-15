use image::{GenericImage, GenericImageView, DynamicImage, Rgba};

fn main() {
    let base_image = image::open("image.jpg").unwrap();

    let d_image = dither(base_image);

    d_image.save("dithered_image.jpg").unwrap();

}

fn quantize(col: Rgba<u8>) -> Rgba<u8> {

    let n_bits = 2;

    let f_levels: f32 = ((1 << n_bits) - 1) as f32;

    let nr = f32::clamp(f32::round(col[0] as f32 / 255.0 * f_levels) / f_levels * 255.0, 0.0, 255.0) as u8;
    let ng = f32::clamp(f32::round(col[1] as f32 / 255.0 * f_levels) / f_levels * 255.0, 0.0, 255.0) as u8;
    let nb = f32::clamp(f32::round(col[2] as f32 / 255.0 * f_levels) / f_levels * 255.0, 0.0, 255.0) as u8;
    let na = col[3];

    Rgba::from([nr,ng,nb,na])

}

fn dither(source: DynamicImage) -> DynamicImage {
    let mut dest = source.clone();

    for y in 0..source.height() {
        for x in 0..source.width() {
            let op = dest.get_pixel(x, y);
            let qp = quantize(op);

            let error: [i32; 3] = [
                op[0] as i32 - qp[0] as i32,
                op[1] as i32 - qp[1] as i32,
                op[2] as i32 - qp[2] as i32    
            ];

            dest.put_pixel(x, y, qp);

            let mut update_pixel = |x_off: i32, y_off: i32, error_bias: f32| {

                let temp_x = (x as i32 + x_off) as u32;
                let temp_y = (y as i32 + y_off) as u32;

                let clamp_x = u32::clamp(temp_x, 0, dest.width() - 1);
                let clamp_y = u32::clamp(temp_y, 0, dest.height() - 1);

                if temp_x == clamp_x && temp_y == clamp_y {

                    let p = dest.get_pixel(temp_x, temp_y);
                    let mut k = [p[0] as f32, p[1] as f32, p[2] as f32, p[3] as f32];

                    k[0] += error[0] as f32 * error_bias;
                    k[1] += error[1] as f32 * error_bias;
                    k[2] += error[2] as f32 * error_bias;

                    dest.put_pixel(temp_x, temp_y, Rgba::from(k.map(|x| f32::clamp(x, 0.0, 255.0) as u8)));
                }
            };

            update_pixel( 1, 0, 7.0/16.0);
            update_pixel(-1, 1, 3.0/16.0);
            update_pixel( 0, 1, 5.0/16.0);
            update_pixel( 1, 1, 1.0/16.0);
        }
    }

    dest
}
