  .text
  .globl main
main:
  li t0, 5
  li t1, 5
  add t1, t0, t1
  li t0, 5
  sub t0, t1, t0
  li t1, 6
  sub t1, t0, t1
  seqz t1, t1
  li t0, 0
  sub t0, t1, t0
  snez t0, t0
  li t1, 7
  li t2, 0
  sub t2, t1, t2
  snez t2, t2
  snez t0, t0
  snez t2, t2
  and t2, t0, t2
  li t0, 0
  li t1, 0
  sub t1, t0, t1
  seqz t1, t1
  li t0, 8
  li t3, 0
  sub t3, t0, t3
  snez t3, t3
  li t0, 0
  sub t0, t1, t0
  snez t0, t0
  snez t3, t3
  snez t0, t0
  and t0, t3, t0
  li t1, 7
  li t3, 8
  sub t3, t1, t3
  snez t3, t3
  li t1, 0
  sub t1, t0, t1
  snez t1, t1
  li t0, 0
  sub t0, t3, t0
  snez t0, t0
  snez t1, t1
  snez t0, t0
  and t0, t1, t0
  li t1, 0
  sub t1, t2, t1
  snez t1, t1
  li t2, 0
  sub t2, t0, t2
  snez t2, t2
  snez t1, t1
  snez t2, t2
  or t2, t1, t2
  li t0, 7
  li t1, 8
  sgt t1, t0, t1
  seqz t1, t1
  li t0, 0
  sub t0, t2, t0
  snez t0, t0
  li t2, 0
  sub t2, t1, t2
  snez t2, t2
  snez t0, t0
  snez t2, t2
  or t2, t0, t2
  mv a0, t2
  ret
