
/* RADIX!!(N) */
int *
function_2 (var_0_0)
     int var_0_0;
{
  int _local_iters0_, _size0_;
  int * var_1_0, * map1_1;
  struct shape_1d _shape0_, * var_2_0;
  var_2_0 = make_shape_1d (var_0_0);
  _shape0_ = var_2_0;
  _size0_ = shape_size (_shape0_);
  _local_iters0_ = shape_local_iters (_shape0_);
  {
    int _i_ = 0;
    int _self_address_ = _size0_ * SELF_PE_INT;
    int * _dest0_;
    int map1_2 = 65535;
    map1_1 = get_pvar (_shape0_);
    _dest0_ = pvar_values (map1_1);
    for (; _i_ < _local_iters0_; _i_++, _self_address_++)
      {
	_dest0_[_i_] = ((((_self_address_ * _self_address_) * 57) + (_self_address_ * 17)) % map1_2);
      }
  }
  dec_count_shape (var_2_0);
  var_1_0 = function_1 (map1_1, 65536);
  return (var_1_0);
}