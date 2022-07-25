use ndarray::{s, Array3, Array4};

pub fn convolution(input: Array3<f32>, kernel: Array4<f32>) -> (Array3<f32>, usize, String) {
    // height, width, channels
    let (h, w, c) = input.dim();

    // output channels, kernel height, kernel width, input channels
    let (c_out, hf, wf, c_in) = kernel.dim();

    // input channels must match
    assert_eq!(c, c_in);

    // height and width of kernel must be an uneven number
    assert!(hf % 2 == 1);
    assert!(wf % 2 == 1);

    let dh = hf / 2;
    let dw = wf / 2;

    let mut output = Array3::zeros((h - 2 * dh, w - 2 * dw, c_out));

    // run convolution
    // go over image height - kernel padding (2*dh)
    for i in dh..h - dh {
        //  go over image width - kernel padding (2*dw)
        for j in dw..w - dw {
            // kernel slice
            let a = &input.slice(s![i - dh..i + dh + 1, j - dw..j + dw + 1, ..]);
            // for output channels (number of kernels)
            for k in 0..c_out {
                // filter channel
                let b = &kernel.slice(s![k, .., .., ..]);
                // apply filter on kernel slice
                output[[i - dh, j - dw, k]] = (a * b).sum();
            }
        }
    }

    let n_params = kernel.len();
    let name = String::from(format!("conv {}x{}x{}x{}", c_out, hf, wf, c_in));
    // how to get a.len()?
    // let n_multiplications = a.len() * c_out * (w-2*dw) * (h-2*dh);

    return (output, n_params, name);
}
