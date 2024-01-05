use std::io::Write;

fn main(){
    let name = vec!["\n1)Aigbogun Ajamba Dauda\n","2)Murtala Afeez Bendu \n","3)Okorocha Calistus Ogbona\n,4)Adewale Jimoh Akanbi\n","5)Osazuwa Faith Etieye\n"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geopolitical_zone = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = std::fs::File::create("project_3.txt").expect("create failed");

        file.write_all("\nThe information below displays information of convicted ministers.".as_bytes()).expect("write failed");


    for i in 0..name.len(){ 
        
        file.write_all(name[i].as_bytes()).expect("write failed");
        file.write_all("Ministry.:".as_bytes()).expect("write failed");
        file.write_all(ministry[i].as_bytes()).expect("write failed");
        file.write_all("\ngeopolotical zone:".as_bytes()).expect("write failed");
        file.write_all(geopolitical_zone[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
        
        
    }
    println!("\nData written to file.");

    
}
