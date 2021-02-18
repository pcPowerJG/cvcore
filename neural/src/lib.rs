extern crate rand;
use rand::{thread_rng, Rng};



pub struct Neuron {
    weight: Vec<f32>,
    out: f32,
    input: Vec<f32>
}
impl Neuron {
    pub fn new(impt_cnt: usize) -> Neuron {
        let mut v1: Vec<f32> = Vec::new();
        let mut v2: Vec<f32> = Vec::new();
        let mut rnd = thread_rng();
        for i in 0..impt_cnt {
            v1.push(rnd.gen::<f32>() * 
                if rnd.gen::<i8>() % 2 == 0 { 1.0 } else { -1.0 });
            v2.push(0.0);
        }
        Neuron { weight: v1, out: 0.0, input: v2 }
    }
    pub fn print(&self) {
        println!("{:?}", self.weight)
    }
    pub fn clone(&self) -> Neuron {
        Neuron { weight: self.weight.clone(), out: self.out, input: self.input.clone() }
    }
}
pub struct Layer {
    neurons: Vec<Neuron>,
}

pub struct Sheme {
    layers: Vec<Vec<Layer>>,
}

pub mod neural_network {
    extern crate rand;
    use rand::{thread_rng, Rng};
    use std::{
        f32,
    };
    pub fn image_derive() {
        // разделение изображения
    }        
	pub struct Net{
		data_base: Vec<Neywork>,
	}
	pub struct Neywork{
		weight: Vec<f32>,
		inputs: Vec<f32>,
		learn_speed: f32,

		result: f32,

	}
	pub fn new()->Net{
		Net { data_base: Vec::new() }
	}

	impl Neywork{
		pub fn proceed(&mut self){
			let mut r: f32 = 0.0;

			for i in 0..self.inputs.len(){
				r += (self.inputs[i] * self.weight[i]) + self.learn_speed;
			}
			self.result = 1.0/(r.clone().exp() * -1.0).sin();
		}
		pub fn on_error(&mut self, true_result: f32){
			let delta: f32 = true_result - self.result;
			for i in 0..self.inputs.len(){
				self.weight[i] = self.weight[i] + (self.inputs[i] * delta * self.learn_speed);				
			}
        }
        pub fn set_inputs(&mut self, inputs: Vec<f32>) {
            if self.inputs.len() == inputs.len() { 
                self.inputs = inputs.clone();
            }
        }
        pub fn get_result(&self) -> f32 { self.result.clone() }
        pub fn get_weight(&self) -> Vec<f32> { self.weight.clone() }
	}
	impl Net{
        pub fn new_conv2d(&mut self, sqrv_cnt: usize, learn_speed: f32) {
            let mut t1: Vec<f32> = Vec::new();
            let mut t2: Vec<f32> = Vec::new();
            let mut rnd = thread_rng();
			for i in 0..sqrv_cnt {
				t1.push(rnd.gen::<f32>() * 
                if rnd.gen::<i8>() % 2 == 0 { 1.0 } else { -1.0 });
            }		
			let temp: Neywork = 
                Neywork{ weight: t1.clone(), inputs: t1.clone(), 
                    learn_speed: learn_speed, result: 0.0 };
			self.data_base.push(temp);
        }
		pub fn new_neyron(&mut self, weight_count: usize, learn_speed: f32)->bool{
            let mut t1: Vec<f32> = Vec::new();
            let mut t2: Vec<f32> = Vec::new();
            let mut rnd = thread_rng();
			for i in 0..weight_count{
				t1.push(rnd.gen::<f32>() * 
                if rnd.gen::<i8>() % 2 == 0 { 1.0 } else { -1.0 });
                t2.push(0.0);
            }		
			let temp: Neywork = 
                Neywork{ weight: t1.clone(), inputs: t1.clone(), 
                    learn_speed: learn_speed, result: 0.0 };
			self.data_base.push(temp);
			true
		}
		pub fn remove_neyron(&mut self, index: usize){
			self.data_base.remove(index);
		}
        pub fn len(&self)->usize{ self.data_base.len() }
        pub fn proceed(&mut self, input: Vec<f32>) -> Vec<f32> {  
            let mut j: usize = self.data_base.len();
            let mut results: Vec<f32> = Vec::new();
            if j > 1 {                
                for i in 0..j{
                    self.data_base[i].set_inputs(input.clone());
                    self.data_base[i].proceed();
                    results.push(self.data_base[i].get_result().clone());
                }
            } else { /* none */ }
            results
        }
        pub fn proceed_2d(&mut self, input_card: Vec<Vec<f32>>, mut distanse: usize) -> Vec<f32> {
            if distanse == 0 { 
                panic!("fuck off");
            }
            let mut j: usize = self.data_base.len();
            let mut results: Vec<Vec<f32>> = Vec::new();
            if j > 1 {                
                for i in 0..j{
                    //self.data_base[i].set_inputs(input_card.clone());
                    //self.data_base[i].proceed();
                    //results.push(self.data_base[i].get_result().clone());
                }
            } else { /* none */ }
            //results
            Vec::new()
        }
	}
}