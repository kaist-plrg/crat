BEGIN {
  finger_server     = "andrew.cmu.edu"
  finger_connection = "/inet/tcp/0/" finger_server "/finger"
  print "wnace" |& finger_connection
  while ((finger_connection |& getline) > 0)
    print $0
  close(finger_connection)
}
