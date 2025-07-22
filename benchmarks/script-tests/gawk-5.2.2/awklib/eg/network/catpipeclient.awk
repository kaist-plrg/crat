BEGIN {
  NetService = "/inet/tcp/0/localhost/8888"
  print "README" |& NetService
  while ((NetService |& getline) > 0)
    print $0
  close(NetService)
}
