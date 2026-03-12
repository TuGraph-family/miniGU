# Generate Dataset for LDBC SNB
1. Pull SNB Datagen image from Docker Hub.
```bash
$ docker pull ldbc/datagen-standalone:0.5.1-19-829b7a04-2.12_spark3.2
```

2. Run Datagen script with specified parallelism and maximum scale factor.
```bash
$ PARALLELISM=8 MAX_SF=10 ./generate.sh
```

3. Run preprocessing script to convert the data to CSV format.
```bash
$ ./process.sh
```  

4. To generate an all-in-one CSV for CEG-related experiments, run `ceg.sh`.
```bash
$ ./ceg.sh
```

For more details about LDBC SNB Datagen, please refer to the [official repository](https://github.com/ldbc/ldbc_snb_datagen_spark).