# Sample code as support to ask for help on how to use tokio and tokio-signal

I wrote a small program which control a GPU fan speed manually via PWM according to temperature.

However it is missing a crucial feature which is to restore the fan speed control to "auto"
whenever the progam is interrupted (Ctrl+C, SIGINT) or terminated (killall program, SIGTERM)

Possibly select https://docs.rs/futures/0.1/futures/stream/trait.Stream.html#method.select should
be used to combinate SIGINT and SIGTERM tokio-signal streams,
while the fan controlling loop checking the temperature every second is running.
