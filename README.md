# RustGoBenchmark
Benchmarking Hello World and Factorial Requests in Rust(Actix) and Go

# Running the Web Servers

## Rust (Port 8000)

In the root directory of the project run
```
cargo run --release
```

## Go (Port 8080)

In the root directory of the project run
```
go build main.go
./main
```

# Benchmarking Hello World Endpoint

## Rust (Port 8000)

```
wrk -t12 -c1000 -d10s http://localhost:8000/
```

## Go (Port 8080)

```
wrk -t12 -c1000 -d10s http://localhost:8080/
```

# Benchmarking Factorial Endpoint

## Rust (Port 8000)

```
wrk -t12 -c1000 -d10s http://localhost:8000/factorial/5000
```

## Go (Port 8080)

```
wrk -t12 -c1000 -d10s http://localhost:8080/factorial/5000
```

# Results on Ryzen 1600 and 16GB RAM - Hello World Endpoint

## Rust

```
# nithin at pc in ~ [22:37:45]
→ wrk -t12 -c1000 -d10s http://localhost:8000/                                                                                                                                                                                    0s 
Running 10s test @ http://localhost:8000/
  12 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.01ms    2.37ms  44.91ms   89.94%
    Req/Sec    47.21k     9.97k   70.12k    67.25%
  5655646 requests in 10.07s, 695.78MB read
Requests/sec: 561892.52
Transfer/sec:     69.13MB
```

## Go
```
# nithin at pc in ~ [22:37:58]
→ wrk -t12 -c1000 -d10s http://localhost:8080/                                                                                                                                                                                   10s 
Running 10s test @ http://localhost:8080/
  12 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.17ms    4.27ms 206.62ms   91.35%
    Req/Sec    20.64k     3.32k   38.23k    71.87%
  2474623 requests in 10.10s, 304.44MB read
Requests/sec: 245038.99
Transfer/sec:     30.15MB
```

# Results on Ryzen 1600 and 16GB RAM - Factorial Endpoint

## Rust

```
# nithin at pc in ~ [22:38:13]
→ wrk -t12 -c1000 -d10s http://localhost:8000/factorial/5000                                                                                                                                                                     11s 
Running 10s test @ http://localhost:8000/factorial/5000
  12 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   268.85ms   29.07ms 652.31ms   95.99%
    Req/Sec   306.18    115.77   760.00     68.95%
  36563 requests in 10.05s, 573.46MB read
Requests/sec:   3639.63
Transfer/sec:     57.08MB
```

## Go

```
# nithin at pc in ~ [22:38:36]
→ wrk -t12 -c1000 -d10s http://localhost:8080/factorial/5000                                                                                                                                                                     10s 
Running 10s test @ http://localhost:8080/factorial/5000
  12 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   375.44ms  480.14ms   2.00s    82.55%
    Req/Sec   174.16     86.47   535.00     68.87%
  20751 requests in 10.03s, 325.82MB read
  Socket errors: connect 0, read 0, write 0, timeout 669
Requests/sec:   2069.22
Transfer/sec:     32.49MB

```
