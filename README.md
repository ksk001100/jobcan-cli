# [**WIP**] jobcan cli

## install

```bash
$ git clone https://github.com/ksk001100/jobcan-cli
$ cd jobcan-cli
$ cargo install --path .
```

## usage
```bash
$ jobcan
未出勤 -> 勤務中

$ jobcan --email test@test.com --password hogefuga
勤務中 -> 退室中

$ jobcan -e test@test.com -p hogefuga
退室中 -> 勤務中
```