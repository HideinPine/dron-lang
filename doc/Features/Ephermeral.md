## *Ephemeral Regions* 
Are regions that promises all objects allocated within it are deallocated atomically or simutaneously when the region's lexical scope ends. 

This means:
1. The objects are freed at the same time i.e instantly. (Simutaneously)
2. Atomically- the operation is indivisible. No partial deallocation. 

> It ***only*** handles when to free memory allocated. ie when the region itself ends..

### My vision
```
  eph_region {
    //code
  }
```