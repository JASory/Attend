# Attend
GTK4 application for recording club attendance

# Installing

Installing requires building from source, to do that you need rustc, cargo, and libgtk-4-dev

Clone the repository

`` git clone https://github.com/JASory/Attend ``
Then run the installer script

`` ./installer.sh ``
If you use su rather than sudo then you will have to modify the installer script by commenting out the sudo lines and uncommenting the su line

The program will be installed as well as a folder labeled "attendance" with a "attendees.csv" and a "members.dat" file in the user's home directory

Members.dat is a list of members and their information, to be used for quickly filling out an attendance form. The attendees.csv file is the list of people who are actually present and filled out the attendance form, members or not. 

To delete you run the uninstall script
`` ./uninstall.sh ``
Like the installer you may have to change the sudo lines to su

The uninstaller does not delete the attendance folder with submitted data

# Usage
## User
 The end-user has to follow the steps to log their attendance
 
 1. Start typing their name, starting with first name, if they are a listed member they will show up on in the drop down
 2. If they are a member then hitting the Enter key, or Lookup button will propagate their information to the rest of the forms, if they are not then they have to fill them out
 3. User presses the "Submit" button to submit their information, which is then written to a CSV file

## Recorder
 The secretary has to manually update the members.dat list, this can be done by copy-pasting the attendees you want from the attendees.csv file to the members.dat file. 
 Both files follow the CSV format. 
