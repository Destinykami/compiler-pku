  .global main
main:
  li   t0, 2
  li   t1, 3
  mul   t0,t0,t1
  li   t1, 1
  add   t0,t1,t0
  mv a0, t0
  ret
