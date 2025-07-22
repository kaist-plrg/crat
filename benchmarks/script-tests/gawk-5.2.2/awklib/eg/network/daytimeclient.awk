BEGIN {
  daytime_server     = "time-a-g.nist.gov"
  daytime_connection = "/inet/tcp/0/" daytime_server "/daytime"
  daytime_connection |& getline
  print $0
  daytime_connection |& getline
  print $0
  close(daytime_connection)
}
