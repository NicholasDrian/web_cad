TODO: Factor device, queue, and adapter out of renderer.
TODO: Build cutom allocator, VRAM allocation is slow and should be done upfront.
TODO: Threading through web workers
TODO: event loop js?
TODO: make samplers and renderer async better
TODO: start removing unwraps
TODO: gumball as aditional feature
TODO: raytracing as aditional feature
TODO: frustum tracing as aditional feature
TODO: 42 warnings, lest chop this in half
TODO: remove copy src flags used for debug
TODO: add workgroup sizes

BUG: writing over data that we dont own ( due to worgroups size not being perfect cube)