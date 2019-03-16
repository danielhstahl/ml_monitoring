use slice_deque::SliceDeque;
pub struct MonitoringInstructions{
    num_new_elements:usize
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
    /*fn is_within_threshold(drift_statistic:f64, instructions:&MonitoringInstructions)->bool{
        drift_statistic<instructions.drift_threshold
    }*/
}



pub struct Outcome<T>{
    predicted:T,
    actual:T
}
fn specificity(t_n:f64, f_p:f64)->f64{
    t_n/(t_n+f_p)
}
fn sensitivity(t_p:f64, f_n:f64)->f64{
    t_p/(t_p+f_n)
}
fn precision(t_p:f64, f_p:f64)->f64{
    t_p/(t_p+f_p)
}
fn accuracy(t_p:f64, t_n:f64, f_p:f64, f_n:f64)->f64{
    (t_p+t_n)/(t_p+t_n+f_p+f_n)
}
pub struct ConfusionMatrix{
    true_positive:usize,
    true_negative:usize,
    false_positive:usize,
    false_negative:usize
}
impl ConfusionMatrix{
    fn create(data:&[Outcome<usize>])-> Self{
        let true_positive=data.iter().filter(|outcome|{outcome.predicted==outcome.actual&&outcome.predicted==1}).count();
        let true_negative=data.iter().filter(|outcome|{outcome.predicted==outcome.actual&&outcome.predicted==0}).count();
        let false_positive=data.iter().filter(|outcome|{outcome.predicted!=outcome.actual&&outcome.predicted==1}).count();
        let false_negative=data.iter().filter(|outcome|{outcome.predicted!=outcome.actual&&outcome.predicted==0}).count();
        ConfusionMatrix{
            true_positive,
            true_negative,
            false_negative,
            false_positive
        }
    }
    fn specificity(&self)->f64{
        specificity(self.true_negative as f64, self.false_positive as f64)
    }
    fn sensitivity(&self)->f64{
        sensitivity(self.true_positive as f64, self.false_negative as f64)
    }
    fn precision(&self)->f64{
        precision(self.true_positive as f64, self.false_positive as f64)
    }
    fn accuracy(&self)->f64{
        accuracy(self.true_positive as f64, self.true_negative as f64, self.false_positive as f64, self.false_negative as f64)
    }
}

fn compute_mse(values: &SliceDeque<Outcome<f64>>)->f64{
    let n=values.len();
    values.iter().map(|outcome|{(outcome.predicted-outcome.actual).powi(2)}).sum::<f64>()/(n as f64)
}


pub struct HoldOutputDataUsize{
    data:SliceDeque<Outcome<usize>>
}

impl HoldOutputDataUsize{
    fn new()->Self{
        HoldOutputDataUsize{
            data:SliceDeque::new()
        }
    }
    fn push(&mut self, predicted:usize, actual:usize, instructions:&MonitoringInstructions){
        let outcome=Outcome{predicted, actual};
        push_fixed_length(&mut self.data, outcome, instructions.num_new_elements);
    }
    fn compute_confusion_matrix(&self)->ConfusionMatrix{
        ConfusionMatrix::create(&self.data)
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
    use approx::*;
    #[test]
    fn creates_new_data(){
        let mut x=HoldInputData::new();
        let instructions=MonitoringInstructions{
            num_new_elements:10
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
            num_new_elements:10
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
    fn confusion_matrix_works(){
        let x=vec![
            Outcome::<usize>{predicted:1, actual:0},
            Outcome::<usize>{predicted:1, actual:1}
        ];
        let cm=ConfusionMatrix::create(&x);
        assert_eq!(cm.accuracy(), 0.5);
        assert_eq!(cm.specificity(), 0.0);
        assert_eq!(cm.sensitivity(), 1.0);
        assert_eq!(cm.precision(), 0.5);
    }
    #[test]
    fn hold_output_usize_works(){
        let mut x=HoldOutputDataUsize::new();
        let instructions=MonitoringInstructions{
            num_new_elements:10
        };
        x.push(1, 0, &instructions);
        x.push(1, 1, &instructions);
        let cm=x.compute_confusion_matrix();
        assert_eq!(cm.accuracy(), 0.5);
        assert_eq!(cm.specificity(), 0.0);
        assert_eq!(cm.sensitivity(), 1.0);
        assert_eq!(cm.precision(), 0.5);
    }
    #[test]
    fn hold_output_f64_works(){
        let mut x=HoldOutputDataF64::new();
        let instructions=MonitoringInstructions{
            num_new_elements:10
        };
        x.push(1.5, 1.2, &instructions);
        x.push(1.2, 1.6, &instructions);
        let mse=x.compute_mse();
        abs_diff_eq!(mse, 0.125, epsilon=0.000001);
        
    }
}