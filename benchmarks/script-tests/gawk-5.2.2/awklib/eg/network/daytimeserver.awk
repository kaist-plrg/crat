BEGIN {
  print strftime() |& "/inet/tcp/8888/0/0"
  close("/inet/tcp/8888/0/0")
}
