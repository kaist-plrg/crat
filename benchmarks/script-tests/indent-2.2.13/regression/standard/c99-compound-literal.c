* rn = (chrename_t)
{
  .chat_id = chat_id,
  .newname = sl_dup (newname_utf8),
  .cb = cb,
  .udata = udata,
};
