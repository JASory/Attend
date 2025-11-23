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
const AUTHOR_NAME : &str = "J.A Sory";
const PROGRAM_COMMENT : &str = "Application to assist in taking club attendance";
const LICENSE : gtk4::License  = gtk4::License::Gpl30;



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


fn retrieve(locale: String) -> Vec<Data>{
  
  let dbase = std::fs::File::open(locale).unwrap();
  
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

fn add_actions(
    application: &gtk4::Application,
    window: &gtk4::ApplicationWindow,
) {
    let about = gtk4::gio::SimpleAction::new("about", None);
    about.connect_activate(clone!(#[weak] window, move |_, _| {
        let p = gtk4::AboutDialog::new();
        p.set_authors(&[AUTHOR_NAME]);
        p.set_license_type(LICENSE);
        //p.set_logo(None);
        p.set_logo_icon_name(None);
        p.set_program_name(Some(APP_NAME));
        p.set_copyright(Some("© 2024 J.A Sory"));
        p.set_version(Some("1.0.0"));
        p.set_comments(Some(PROGRAM_COMMENT));
        p.set_transient_for(Some(&window));
        p.show();
    }));
   
    let present = gtk4::gio::SimpleAction::new("present", None);
    
    present.connect_activate(
      clone!(#[weak] window, move |_, _| {
    
            let conf = std::fs::read_to_string(CONFIG).unwrap().trim().to_string();
            let dataset = retrieve(conf+ATTENDEE);
            let mut veccy = vec![];
            
                for i in dataset{
                  veccy.push(i.name)
                }
                
                let textout = veccy.join("\n");
                
        let textbuff = gtk4::TextBuffer::builder()
                       .text(&textout)
                       .build();
                       
        let textview = gtk4::TextView::builder()
                       .buffer(&textbuff)
                       .build();
        
                let p = gtk4::Dialog::builder()
                        .default_widget(&textview)
                        .build();
                        
        p.set_title(Some("Attendees"));
        p.set_child(Some(&textview));
        p.set_transient_for(Some(&window));
        p.show();
    }));

    application.add_action(&about);
    application.add_action(&present);

}

fn build_header_menu(header: &gtk4::HeaderBar){
     let menu = gtk4::gio::Menu::new();
        menu.append(Some("Present List"),Some("app.present"));
        menu.append(Some("About"), Some("app.about"));
        let p = gtk4::MenuButton::new();
        p.set_menu_model(Some(&menu));
        header.pack_end(&p);
 }

  

fn build_ui(application: &gtk4::Application) {

    let window = gtk4::ApplicationWindow::new(application);

    window.set_default_size(600,400);
    
    let header_title = gtk4::Label::new(Some(APP_NAME));
    
    let top = gtk4::HeaderBar::builder()
                  .show_title_buttons(true)
                  .title_widget(&header_title)
                  .build();
 
   build_header_menu(&top);    
    
   window.set_titlebar(Some(&top));
    
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
    
    let dataset = retrieve(conf.clone()+DATABASE);
    
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
  
    submit_button.connect_clicked(clone!(#[weak] name_entry, #[weak] meid_entry,#[weak] email_entry, move |_|{
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
    
     lookup_button.connect_clicked(clone!(#[weak] name_entry, #[weak] meid_entry, #[weak] email_entry, move |_|{    
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
    
    name_entry.connect_activate(clone!(#[weak] meid_entry,#[weak] email_entry, move |name|{
     
     match search_name(&dset2,name.text().to_string()){
       Some(x) =>{
         meid_entry.set_text(&x.meid);
         email_entry.set_text(&x.email);
       }
       None => {}
     } 
    }));
    
    window.set_child(Some(&row));

    add_actions(application,&window);
    
    window.present();

}



fn main() -> glib::ExitCode{

    let application = gtk4::Application::builder()
        .application_id("com.github.jasory.attendance")
        .build();

    application.connect_activate(build_ui);
    application.run()
}
