# export-setup-parameters
This repo contains scripts that export the setup parameters to the arkworks-rs format.

- `cargo run --bin export_srs --release 13` generate 2^13

## ark bls12_381 srs size

| n  | size      | file size        |
| -- | --------- | ---------------- |
| 10 | 2^10 + 3  | 57736 (57K)      |
| 11 | 2^11 + 3  | 115080 (113K)    |
| 12 | 2^12 + 3  | 229768 (225K)    |
| 13 | 2^13 + 3  | 459144 (449K)    |
| 14 | 2^14 + 3  | 917896 (897K)    |
| 15 | 2^15 + 3  | 1835400 (1.8M)   |
| 16 | 2^16 + 3  | 3670408 (3.6M)   |
| 17 | 2^17 + 3  | 7340424 (7.1M)   |
| 18 | 2^18 + 3  | 14680456 (15M)   |
| 19 | 2^19 + 3  | 29360520 (29M)   |
| 20 | 2^20 + 3  | 58720648 (57M)   |
| 21 | 2^21 + 3  | 117440904 (113M) |

## File

https://aztec-ignition.s3.eu-west-2.amazonaws.com/MAIN+IGNITION/sealed/transcript00.dat
https://aztec-ignition.s3.eu-west-2.amazonaws.com/MAIN+IGNITION/sealed/transcript01.dat
...

## Step
```
cargo run --bin import_g1 --release
cargo run --bin import_g2 --release (can skip)
cargo run --bin import_srs --release
cargo run --bin import_lagrange --release
cargo run --bin export_srs_padding --release
```