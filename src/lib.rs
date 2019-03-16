use slice_deque::SliceDeque;
pub struct MonitoringInstructions{
    num_new_elements:usize,
    drift_threshold:f64,
    fp_rate_threshold:f64,
    fn_rate_threshold:f64 //can combine to get accuracy threshold, specificty threshold, etc
}


fn push_fixed_length<T>(data:&mut SliceDeque<T>, new_element:T, max_num:usize){
    data.push_back(new_element);
    if data.len()>max_num{
        data.pop_front();
    }
}

pub struct HoldInputData{
    data:SliceDeque<f64>
}
impl HoldInputData{
    fn new()->Self{
        HoldInputData{
            data:SliceDeque::new()
        }
    }
    fn push(&mut self, new_element:f64, instructions:&MonitoringInstructions){
        push_fixed_length(&mut self.data, new_element, instructions.num_new_elements);
    }
    fn compute_drift(&self, original_data:&[f64])->f64{
        let result = kolmogorov_smirnov::test_f64(&self.data, original_data, 0.05);
        result.statistic
    }
}


pub struct Outcome<T>{
    predicted:T,
    actual:T
}

fn convert_bool_to_float(b:bool)->f64{
    if b { 1.0 } else { 0.0 }
}

fn compute_accuracy(values: &SliceDeque<Outcome<usize>>)->f64{
    let n=values.len();
    values.iter().map(|outcome|{((outcome.predicted-outcome.actual) as f64).abs()}).sum::<f64>()/(n as f64)
}
//only relevant for binary
fn compute_fp_rate(values: &SliceDeque<Outcome<usize>>)->f64{
    let n=values.len();
    values.iter().map(|outcome|{convert_bool_to_float(outcome.predicted==1 && outcome.actual==0)}).sum::<f64>()/(n as f64)
}
//only relevant for binary
fn compute_fn_rate(values: &SliceDeque<Outcome<usize>>)->f64{
    let n=values.len();
    values.iter().map(|outcome|{convert_bool_to_float(outcome.predicted==0 && outcome.actual==1)}).sum::<f64>()/(n as f64)
}
fn compute_mse(values: &SliceDeque<Outcome<f64>>)->f64{
    let n=values.len();
    values.iter().map(|outcome|{(outcome.predicted-outcome.actual).powi(2)}).sum::<f64>()/(n as f64)
}

pub struct HoldoutputdataUsize{
    data:SliceDeque<Outcome<usize>>
}

impl HoldoutputdataUsize{
    fn new()->Self{
        HoldoutputdataUsize{
            data:SliceDeque::new()
        }
    }
    fn push(&mut self, predicted:usize, actual:usize, instructions:&MonitoringInstructions){
        let outcome=Outcome{predicted, actual};
        push_fixed_length(&mut self.data, outcome, instructions.num_new_elements);
    }
    fn compute_accuracy(&self)->f64{
        compute_accuracy(&self.data)
    }
    fn compute_fn_rate(&self)->f64{
        compute_fn_rate(&self.data)
    }
    fn compute_fp_rate(&self)->f64{
        compute_fp_rate(&self.data)
    }
}
pub struct HoldOutputDataF64{
    data:SliceDeque<Outcome<f64>>
}

impl HoldOutputDataF64{
    fn new()->Self{
        HoldOutputDataF64{
            data:SliceDeque::new()
        }
    }
    fn push(&mut self, predicted:f64, actual:f64, instructions:&MonitoringInstructions){
        let outcome=Outcome{predicted, actual};
        push_fixed_length(&mut self.data, outcome, instructions.num_new_elements);
    }
    fn compute_mse(&self)->f64{
        compute_mse(&self.data)
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creates_new_data(){
        let mut x=HoldInputData::new();
        let instructions=MonitoringInstructions{
            num_new_elements:10,
            drift_threshold:3.0,
            fp_rate_threshold:3.0,
            fn_rate_threshold:3.0
        };
        for i in 0..10 {
            x.push(i as f64, &instructions);
        }
        let original_data=vec![
            -1.0, 0.0, 1.0, 3.0, 4.0, 3.0, 5.0, 3.0, 5.4, 2.3, 4.3
        ];
        let statistic=x.compute_drift(&original_data);
        println!("this is statistic {}", statistic);
        assert!(true);
    }
    #[test]
    fn creates_new_data_with_more_data_than_max(){
        let mut x=HoldInputData::new();
        let instructions=MonitoringInstructions{
            num_new_elements:10,
            drift_threshold:3.0,
            fp_rate_threshold:3.0,
            fn_rate_threshold:3.0
        };
        for i in 0..100 {
            x.push(i as f64, &instructions);
        }
        let original_data=vec![
            -1.0, 0.0, 1.0, 3.0, 4.0, 3.0, 5.0, 3.0, 5.4, 2.3, 4.3
        ];
        let statistic=x.compute_drift(&original_data);
        println!("this is statistic {}", statistic);
        assert!(true);
    }
    #[test]
    fn push_fixed_length_test(){
        let mut data:SliceDeque<f64>=SliceDeque::new();
        push_fixed_length(&mut data, 5.0, 3);
        assert_eq!(data[0], 5.0);
        push_fixed_length(&mut data, 3.0, 3);
        assert_eq!(data[0], 5.0);
        assert_eq!(data[1], 3.0);
        push_fixed_length(&mut data, 2.0, 3);
        assert_eq!(data[0], 5.0);
        assert_eq!(data[1], 3.0);
        assert_eq!(data[2], 2.0);
        push_fixed_length(&mut data, 1.0, 3);
        assert_eq!(data[0], 3.0);
        assert_eq!(data[1], 2.0);
        assert_eq!(data[2], 1.0);
    }
    #[test]
    fn accuracy_works(){
        let mut x=HoldoutputdataUsize::new();
        let instructions=MonitoringInstructions{
            num_new_elements:10,
            drift_threshold:3.0,
            fp_rate_threshold:3.0,
            fn_rate_threshold:3.0
        };
        x.push(1, 0, &instructions);
        x.push(1, 1, &instructions);
        assert_eq!(x.compute_accuracy(), 0.5);
    }
    #[test]
    fn compute_fp_rate_works(){
        let mut x=HoldoutputdataUsize::new();
        let instructions=MonitoringInstructions{
            num_new_elements:10,
            drift_threshold:3.0,
            fp_rate_threshold:3.0,
            fn_rate_threshold:3.0
        };
        x.push(1, 0, &instructions);
        x.push(1, 1, &instructions);
        assert_eq!(x.compute_fp_rate(), 0.5);
    }
    #[test]
    fn compute_fn_rate_works(){
        let mut x=HoldoutputdataUsize::new();
        let instructions=MonitoringInstructions{
            num_new_elements:10,
            drift_threshold:3.0,
            fp_rate_threshold:3.0,
            fn_rate_threshold:3.0
        };
        x.push(1, 0, &instructions);
        x.push(1, 1, &instructions);
        assert_eq!(x.compute_fn_rate(), 0.0);
    }
}