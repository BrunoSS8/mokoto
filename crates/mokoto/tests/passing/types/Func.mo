A -> B
---
(A, B) -> C
---
A -> (C, B)
---
A -> C -> B
---
(a : A, b : B) -> (c : C, d : D)
---
query A -> B
---
shared A -> B
---
shared query A -> B
---
<K <: Int, V> A -> B
