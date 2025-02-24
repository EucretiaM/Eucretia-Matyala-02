use burn::tensor::{Tensor, TensorExt, Shape};
use burn::module::Module;
use burn::optim::{Optimizer, SGD};
use burn::nn::{Linear, MSELoss};
use burn_ndarray::NdArrayBackend;
use rand::distributions::{Normal, Distribution};
use rand::thread_rng;
use textplots::{Chart, Plot};
// 1. Generate Synthetic Data with Noise
fn generate_data(num_points: usize) -> (Vec<f32>, Vec<f32>) {
    let mut rng = thread_rng();
    let normal = Normal::new(0.0, 1.0); // For noise

    let mut x_vals = Vec::with_capacity(num_points);
    let mut y_vals = Vec::with_capacity(num_points);

 for _ in 0..num_points {
        let x = rand::random::<f32>() * 10.0; // Random x between 0 and 10
        let noise = normal.sample(&mut rng); // Random noise
        let y = 2.0 * x + 1.0 + noise; // y = 2x + 1 + noise
        x_vals.push(x);
        y_vals.push(y);
    }

    (x_vals, y_vals)
}

// 2. Define the Model using the Burn library
#[derive(Debug, Clone)]
struct LinearRegression {
    linear: Linear<NdArrayBackend<f32>>,
}

impl LinearRegression {
    fn new() -> Self {
        Self {
            linear: Linear::new(1, 1), // Input size and output size
        }
    }
}

impl Module<NdArrayBackend<f32>> for LinearRegression {
    fn forward(&self, input: Tensor<NdArrayBackend<f32>>) -> Tensor<NdArrayBackend<f32>> {
        self.linear.forward(input)
    }
}

// 3. Train the Model
fn train_model() {
    let num_points = 100;
    let (x_vals, y_vals) = generate_data(num_points);

    // Convert data to tensors
    let x_tensor = Tensor::<NdArrayBackend<f32>>::from_data(x_vals).reshape(Shape::new([num_points, 1]));
    let y_tensor = Tensor::<NdArrayBackend<f32>>::from_data(y_vals).reshape(Shape::new([num_points, 1]));

    let model = LinearRegression::new();
    let mut optimizer = SGD::new(&model, 0.01); // Learning rate = 0.01
    let loss_fn = MSELoss::new();

    let epochs = 500;
    for epoch in 0..epochs {
        optimizer.zero_grad();
        let y_pred = model.forward(x_tensor.clone());
        let loss = loss_fn.forward(y_pred, y_tensor.clone());
        optimizer.step(loss.clone());
>>>>>>> 2ff5b76 (Initial commit)
