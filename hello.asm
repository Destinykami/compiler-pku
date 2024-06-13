  .global main
main:
  li   t0, 6
  xor   t0,t0,x0
  seqz  t0,t0
  sub   t0,x0,t0
  sub   t0,x0,t0
  mv a0, t0
  ret
