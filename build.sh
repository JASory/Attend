cargo build --release
echo $HOME/attendance/ > location.conf
cd $HOME
mkdir attendance
touch attendance/members.dat
touch attendance/attendees.csv
