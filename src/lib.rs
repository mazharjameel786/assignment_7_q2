#![allow(dead_code)]
#![allow(unused_variables)]
pub mod geometry {

    pub mod shaps {

        #[derive(Debug)]
        pub enum Shap {
            Rectangle(u32,u32),
            Circle(f64),
            Squre(u32),
        }

        impl Shap {
            pub fn area(&self)->f64{
                match *self {
                    Shap::Rectangle(x,y) => (x*y) as f64,
                    Shap::Circle(x) => (3.14*x*x),
                    Shap::Squre(x) => (x*x) as f64,
                }
    
            }
        }
        
    }
    
}