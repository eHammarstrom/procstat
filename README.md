# procstat

process information and statistics

## Installation



## Usage


## Protocol

_A: starts procstat_
_B: requests websocket to procstat_

_A_ may specify the `disk_poll` rate and minimum `min_ws_poll_rate` of the /proc/stat file.

_B_ may request a poll rate across the socket that is `min(disk_poll_rate, min_ws_poll_rate)`,
procstat will try to deliver to the best of its abilities (and the computer's).


## todo?


* sleep latency

Account for the kernel interrupt latency when sleeping by averaging
the interrupt latency and sleeping for `interval - avg(kernel_intr_latency)`.

* websocket

Setup a websocket service to stream data to requestee at set interval.

* http request

Query procstat of the currently accumulated stats and state.
