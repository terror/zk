---
name: binary-search
tags:
 - algorithms
 - software
links:
 - 1625016407-two-sum.md
 - 1625018068-xd.md
---
This search algorithm can search a sorted array in O(n)!

```python
def binary_search(arr, el):
  l = 0; r = len(arr) - 1; m = 0
  while l <= r:
    m = (h + l) // 2

    if arr[m] == el:
      return m

    if arr[m] < el:
      l = m + 1
    else:
      r = m - 1

  return - 1
```
