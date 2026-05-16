# Double Free

## What It Is

Double free occurs when free() is called on a pointer which has already 
been freed. Since free() adds the chunk to a free list, the second call 
can corrupt the list as the allocator is not expecting to find a chunk 
it's already tracking.

Consequences vary. If the chunk has been reallocated between the two 
free calls, the second deallocates memory now in use elsewhere — 
potentially destroying live data. Otherwise, the same address may end 
up in the free list twice, leading to subsequent malloc calls returning 
pointers to the same address, giving two separate parts of the program 
simultaneous write access to the same memory.

## How It Happens

```c
int *p = (int *)malloc(20);
int *q = (int *)malloc(20);
// ...do some things with *q
free(p);
free(p);
// ...continue using q
```

Here, q is given the same address as p since p was the last chunk to 
be freed. The second free() deallocates that memory again and all 
following usages of q will access memory which is now available for 
allocation, risking overwriting of data used for operations on q. 
Furthermore, q now holds a pointer to a chunk within the free list 
and therefore has the potential to corrupt the free list to gain 
access to arbitrary memory locations.

## Why It's Dangerous

Where two malloc calls return the same address, two separate parts of 
a program gain simultaneous write access to the same memory. If an 
attacker can control the timing and contents of allocations around a 
double free, they can manipulate the free list to make malloc return 
a pointer to attacker-controlled memory — giving arbitrary write 
capability.

In practice this has been used to overwrite function pointers with an 
attacker-controlled address to redirect execution, in the most severe 
cases giving an attacker an interactive shell on the target system.

Heap feng shui is an exploit technique involving deliberate arrangement 
of heap layout through carefully sized allocations and deallocations. 
Combined with a double free vulnerability, this can be used to 
circumvent modern allocator checks — for example, filling the tcache 
to capacity to push freed chunks into the fastbin where double free 
detection is weaker, then manipulating free list pointers to redirect 
subsequent malloc calls to arbitrary memory locations. The term was 
coined by researcher Alexander Sotirov in 2007 and the technique 
remains relevant in modern exploitation.

## Notable History

#### CVE-2024-1086

A vulnerability in the Linux kernel's netfilter component allowed 
attackers to trigger a double free, obtain the kernel base address, 
bypass KASLR, and gain read/write access to the modprobe_path kernel 
variable. This could be used to obtain shell access, giving attackers 
full control of the system. The vulnerability was present in kernels 
from 2014 and was actively exploited in ransomware campaigns before 
being patched.

#### CVE-2017-6074

A double free in the Linux kernel's DCCP socket implementation, 
discovered and documented by researcher Andrey Konovalov. Local 
privilege escalation via heap manipulation. A full writeup and 
proof-of-concept exploit were published, making it one of the more 
thoroughly documented examples of kernel double free exploitation.

## What Valknir Detects

Currently Valknir detects only basic instances of double free — two 
direct calls to free() on the same identifier within the same scope. 
Expansion is planned to include scope awareness, null check detection, 
alias tracking, and interprocedural analysis.

## How To Fix It

The simplest fix is to set the pointer to NULL immediately after 
freeing it. Since free(NULL) is a no-op, any subsequent accidental 
free of the same pointer becomes harmless:

```c
free(p);
p = NULL;
```

More broadly: avoid storing the same raw pointer in multiple places 
without clear ownership.
