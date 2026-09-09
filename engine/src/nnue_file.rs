use super::*;


// for testing 
pub static NNUE: Nnue = Nnue {
    feature_weights: [[0.2; 128]; 49163],
    feature_bias: [0.3; 128],

    hidden0_weights: [0.0; 32 * 256],
    hidden0_bias: [0.0; 32],

    hidden1_weights: [0.0; 32 * 32],
    hidden1_bias: [0.0; 32],

    output_weights: [0.0; 32],
    output_bias: 0.0,
};
