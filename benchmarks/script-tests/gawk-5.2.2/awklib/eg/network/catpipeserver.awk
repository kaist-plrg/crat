BEGIN {
  NetService = "/inet/tcp/8888/0/0"
  NetService |& getline       # sets $0 and the fields
  CatPipe    = ("cat " $1)
  while ((CatPipe | getline) > 0)
    print $0 |& NetService
  close(NetService)
}
