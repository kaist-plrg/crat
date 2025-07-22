void a ()
{
  id = source ? (f ? f->id_src_dev : fwr->id_src_dev) : (f ? f->id_dst_dev : fwr->id_dst_dev);
  
  strbuf_appendf(sb, "%s\n",inet_ntoa(*((struct in_addr *) &ifr->ifr_addr.sa_data[sizeof(sa.sin_port)])));
}

