fn main() {
	//quantity
   let tq:f64 = 2.0;
   let mq:f64 = 1.0;
   let hq:f64 = 3.0;
   let dq:f64 = 3.0;
   let aq:f64 = 1.0;
   
   //amount
   let ta:f64 = 450_000.00;
   let ma:f64 = 1_500_000.00;
   let ha:f64 = 750_000.00;
   let da:f64 = 2_850_000.00;
   let aa:f64 = 250_000.00;

   //sum
   let s =(tq * ta) + (mq * ma) + (hq * ha) + (dq * da) + (aq * aa);

   println!("the sum is {} ", s);

   //average
   let q = tq + mq + hq + dq + aq;

   let avg = s/q;

   println!("the average is {} ", avg);
}
