use super::*;

mod transformer;
pub use transformer::Transformer;




pub struct Nnue {
    // transformer: feature -> accumulator
    pub feature_weights: [[f32; 128]; 49163],
    pub feature_bias:    [f32; 128],

    // 256 -> 32
    pub hidden0_weights: [f32; 32 * 256],
    pub hidden0_bias:    [f32; 32],

    // 32 -> 32
    pub hidden1_weights: [f32; 32 * 32],
    pub hidden1_bias:    [f32; 32],

    // 32 -> 1
    pub output_weights:  [f32; 32],
    pub output_bias:     f32,
}



impl Nnue {
    fn eval(&self, transformer: &Transformer, active: Color) -> i32 {

        // get one input [f32; 256]
        let mut input: [f32; 256] = [0.0; 256];
        let (active, inactive) = match active {
            Color::White => (&transformer.0, &transformer.1),
            Color::Black => (&transformer.1, &transformer.0),
        };
        for i in 0..128 {
            input[i]       = active[i].clamp(0.0, 1.0);
            input[i + 128] = inactive[i].clamp(0.0, 1.0);
        }



        // first hidden layer [f32; 256] -> [f32; 32]
        let mut h1: [f32; 32] = [0.0; 32];
        for i in 0..32 {
            h1[i] = self.hidden0_bias[i] as f32;
            for j in 0..256 {
                h1[i] += input[j] * self.hidden0_weights[i * 256 + j] as f32;
            }
            h1[i] = h1[i].clamp(0.0, 1.0);
        }



        // second hidden layer [f32; 32] -> [f32; 32]
        let mut h2: [f32; 32] = [0.0; 32];
        for i in 0..32 {
            h2[i] = self.hidden1_bias[i] as f32;
            for j in 0..32 {
                h2[i] += h1[j] * self.hidden1_weights[i * 32 + j] as f32;
            }
            h2[i] = h2[i].clamp(0.0, 1.0);
        }



        // output layer [f32; 32] -> f32
        let mut output = self.output_bias as f32;
        for i in 0..32 {
            output += h2[i] * self.output_weights[i] as f32;
        }



        output as i32
    }
}



impl Game {
    pub fn nnue_eval(&self) -> i32 {
        NNUE.eval(&self.transformer, self.state.player)
    }
}
