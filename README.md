# procstat

process information and statistics

## Installation



## Usage



## todo?


* sleep latency

Account for the kernel interrupt latency when sleeping by averaging
the interrupt latency and sleeping for `interval - avg(kernel_intr_latency)`.

* websocket

Setup a websocket service to stream data to requestee at set interval.

* http request

Query procstat of the currently accumulated stats and state.
