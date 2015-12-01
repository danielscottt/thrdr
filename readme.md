thrdr
=====

A tiny program that starts and stops some threads.  Part of an experiment about programming languages hoarding OS threads.


## The example

In one window, after building:

```
$ ./thrdr
```

in the other:

```
$ cat /proc/<thrdr PID>/status | grep -i threads
Threads: 3
$ touch /tmp/tstart
$ touch /tmp/tstart
$ touch /tmp/tstart
$ touch /tmp/tstart
$ cat /proc/<thrdr PID>/status | grep -i threads
Threads: 7
$ touch /tmp/tstop
$ cat /proc/<thrdr PID>/status | grep -i threads
Threads: 6
```
