/*

 Software to aid in taking attendance for clubs at Mesa Community College 
 
 Uses GTK4
 
 Copyright JA Sory @2024 GPL v3 
 
 
 https://www.gnu.org/licenses/gpl-3.0.en.html
 

*/



use gtk4::{
    glib::{self, clone},
    prelude::*,
};

use std::{
    cell::RefCell, 
      io::{BufReader,BufRead,Write}
          };
          
          
const CONFIG : &str = "/opt/attendance/location.conf";
const DATABASE : &str = "members.dat";
const ATTENDEE : &str = "attendees.csv";
const APP_NAME : &str = "Attendance Recorder";
const SEPARATOR : &str = ",";

#[derive(Clone)]
struct Data{
  name: String,
  meid: String,
  email: String,
}

impl Data{

   fn to_string(&self) -> String{
      self.name.clone() + "," + &self.meid + "," + &self.email + "\n"
   }
   
   fn from_string(x: &str) -> Self{
      let interim = x.split(SEPARATOR).collect::<Vec<&str>>();
     
      if interim.len() != 3{
        panic!("Len is {} but you need a length of 3",interim.len());
      }
      Data{name: interim[0].to_string(), meid: interim[1].to_string(), email: interim[2].to_string()}
   }
   
}


fn retrieve(conf: String) -> Vec<Data>{
  
  let dbase = std::fs::File::open(conf+DATABASE).unwrap();
  
  let d_reader = BufReader::new(dbase);
  
  let mut res : Vec<Data> = vec![];
  
     for i in d_reader.lines(){
       let interim = i.unwrap();
     
       if interim == ""{
         continue;
       }
       
    res.push(Data::from_string(&interim));
   }
   
  res
  }
  
  fn search_name(x: &Vec<Data>, name: String) -> Option<Data>{
     for i in x{
        if i.name == name{
           return Some(i.clone())
        }
     }
     None
  }
  
  
  fn name_completion(database: &Vec<Data>) -> gtk4::ListStore{
       let store = gtk4::ListStore::new(&[glib::Type::STRING]);
    for d in database.iter() {
        store.set(&store.append(), &[(0, &d.name)]);
    }
    store

  }

fn build_ui(application: &gtk4::Application) {

    let window = gtk4::ApplicationWindow::new(application);
    window.set_title(Some(APP_NAME));
    window.set_default_size(600,400);
    
    
    let name_entry = gtk4::Entry::new();
    let meid_entry = gtk4::Entry::new();
    let email_entry = gtk4::Entry::new();
    
    email_entry.set_placeholder_text(Some("Optional"));
    let lookup_button = gtk4::Button::with_label("Lookup");
    let submit_button = gtk4::Button::with_label("Submit");
    let button_space = gtk4::Label::new(None);
    let name_label = gtk4::Label::new(Some("Full Name"));
    let meid_label = gtk4::Label::new(Some("MEID"));
    let email_label = gtk4::Label::new(Some("Email"));
    let completion = gtk4::EntryCompletion::new();
    
    // Use the first column
    completion.set_text_column(0);
    // Minimum keystrokes
    completion.set_minimum_key_length(1);
    // Set suggestions to popup
    completion.set_popup_completion(true);
    
    
   // Read the file pointing to where the data files are 
    let conf = std::fs::read_to_string(CONFIG).unwrap().trim().to_string();
    
    let dataset = retrieve(conf.clone());
    
    let ls = name_completion(&dataset);
    
    completion.set_model(Some(&ls));

    name_entry.set_completion(Some(&completion));
    
    
    name_entry.set_margin_top(10);
    
    let row = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();
    
    row.append(&name_label);
    row.append(&name_entry);
    row.append(&meid_label);
    row.append(&meid_entry);
    row.append(&email_label);
    row.append(&email_entry);
    row.append(&lookup_button);
    row.append(&button_space);
    row.append(&submit_button); 
    
    
  let out = std::fs::OpenOptions::new().create(true).append(true).open(conf+ATTENDEE).unwrap();
  let ofile = RefCell::new(out);
  
    submit_button.connect_clicked(clone!(@weak name_entry, @weak meid_entry, @weak email_entry => move |_|{
        let name = name_entry.text().to_string();
        let meid = meid_entry.text().to_string();
        let email = email_entry.text().to_string();
        
        let d = Data{name,meid,email};
        let p_str = d.to_string();
        // Write out the attendee information
        let _ = ofile.borrow_mut().write_all(&p_str.as_bytes()[..]);
        // Clear the input information
        name_entry.set_text("");
        meid_entry.set_text("");
        email_entry.set_text("");
        
    }));
    
    let dset = dataset.clone();
    
     lookup_button.connect_clicked(clone!(@weak name_entry, @weak meid_entry, @weak email_entry => move |_|{    
     let name = name_entry.text();
     match search_name(&dset,name.to_string()){
       Some(x) =>{
         meid_entry.set_text(&x.meid);
         email_entry.set_text(&x.email);
       }
       None => {}
     }
     }));
     
    let dset2 = dataset.clone();
    
    name_entry.connect_activate(clone!(@weak name_entry, @weak meid_entry, @weak email_entry => move |name|{
     
     match search_name(&dset2,name.text().to_string()){
       Some(x) =>{
         meid_entry.set_text(&x.meid);
         email_entry.set_text(&x.email);
       }
       None => {}
     } 
    }));
    
    window.set_child(Some(&row));


    window.present();

}



fn main() -> glib::ExitCode{

    let application = gtk4::Application::builder()
        .application_id("attendance")
        .build();

    application.connect_activate(build_ui);
    application.run()
}
