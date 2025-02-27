# Accidentally Including The World

We actually did this one (somewhat on purpose, really), so let's clean it up.

We've seen what happens when you do this. Let's go ahead and clean up real quick.

Our header file needs only to import:

```c
#pragma once

#include <stdbool.h>

// Remainder unchanged
```

Our C body file gains the removed includes:

```c
#include <stdlib.h>
#include <string.h>
#include "clib.h"
```

You won't always get the luxury of cleaning up other people's messes. But when you can, it helps a lot. The warnings you see now stem from the *library* not having actually used functions, and a couple of places where `mut` wasn't needed.
