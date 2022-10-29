# debussy
CLI emitting json when song changes on Linux. Uses MPRIS and dbus to receive events

## demo

In the example below, I'm using the Gnome application _Shortwave_ to listen to 
internet radio. The app is detected, and then each track change is received, 
and logged, and a json payload generated:

```bash
$ RUST_LOG=info cargo run
   Compiling itoa v1.0.4
   Compiling ryu v1.0.11
   Compiling serde v1.0.147
   Compiling serde_json v1.0.87
   Compiling debussy v0.1.0 (/home/caleb/tmp/dbustest/debussy)
    Finished dev [unoptimized + debuginfo] target(s) in 4.28s
     Running `target/debug/debussy`
 INFO  debussy > Found Shortwave (on bus org.mpris.MediaPlayer2.de.haeckerfelix.Shortwave)
 INFO  debussy > Radio Paradise (320k)::::Mark Knopfler - Don't You Get It
{"player":"Radio Paradise (320k)","artist":"Radio Paradise (320k)","title":"Mark Knopfler - Don't You Get It"}
```

The logs go to stderr, but the json goes to stdout. This means that 
output can be received by other tools in a unix pipeline. For example, streaming 
the json data to the program _jq_:

```bash
$ cargo run -- | jq
   Compiling debussy v0.1.0 (/home/caleb/tmp/dbustest/debussy)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/debussy`
{
  "player": "Radio Paradise (320k)",
  "artist": "Radio Paradise (320k)",
  "title": "Heilung - Anona"
}
{
  "player": "Radio Paradise (320k)",
  "artist": "Radio Paradise (320k)",
  "title": "The Beatles - Help!"
}
```
