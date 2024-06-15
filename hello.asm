  .global main
main:
  li   t0, 1
  li   t1, 2
  sgt   t0,t0,t1
  seqz   t0,t0,
  mv a0, t0
  ret
