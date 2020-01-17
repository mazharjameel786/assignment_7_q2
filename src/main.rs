#![allow(dead_code)]
#![allow(unused_variables)]

use assignment_7_q2::geometry::shaps;

fn main() {
    
   let rec= shaps::Shap::Rectangle(10,20);
   let cir= shaps::Shap::Circle(20.0);
   let sqr= shaps::Shap::Squre(10);

  println!("{}",rec.area());
  println!("{}",cir.area());
  println!("{}",sqr.area());
   


}
