  .global main
main:
  li   t0, 11
  xor   t0,t0,x0
  snez  t0,t0
  li   t1, 1
  xor   t1,x0,t1
  snez  t1,t1
  and   t0,t0,t1
  mv a0, t0
  ret
