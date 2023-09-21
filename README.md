# Nektar

A fast and lightweight command line interface for the [Apache Hive](https://hive.apache.org/) metastore.

## Motivation

Hive's metastore is ubiquitous in many big data deployments, whether the deployment in question makes use
of the Hive execution engine or not.  It is a common component that provides table virtualization for other 
distributed processing engines such as [Flink](https://nightlies.apache.org/flink/flink-docs-master/docs/connectors/table/hive/hive_catalog/), [Spark](https://spark.apache.org/docs/latest/sql-data-sources-hive-tables.html) and [Trino](https://trino.io/docs/current/connector/hive.html).  

More modern analytic table formats such as [Iceberg](https://iceberg.apache.org/docs/latest/hive/) and [Hudi](https://hudi.apache.org/docs/syncing_metastore/) leverage Hive's metastore to some extent for the serving and storage layer of their metadata.  Cloud data catalogs such as GCP's [DataProc Metastore](https://cloud.google.com/dataproc-metastore/docs/hive-metastore) and [AWS's Glue Data Catalog](https://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-hive-metastore-glue.html) also implement the metastore interface.

Typically, operations against the metastore are done via these engines' catalog procedures, or in SQL DDL 
using the Hive Cli itself.  There are limited options for calling the metastore's endpoints themselves directly.  This
project provides a lightweight command line interface to the metastore's Thrift endpoints directly, to allow lower-level
debugging of metastore payloads without JVM startup overhead.


# Usage

Install via `cargo install` or download a binary directly from releases.

```
A fast, lightweight CLI for Hive Metastore

Usage: nektar [OPTIONS] <METASTORE_URI> <COMMAND>

Commands:
  get-table
  get-catalog
  get-partitions
  get-partition-names-by-parts
  get-databases
  help                          Print this message or the help of the given subcommand(s)

Arguments:
  <METASTORE_URL>  Thrift metastore endpoint, eg: host.com:9083

Options:
      --format <FORMAT>  [default: json] [possible values: json, yaml]
  -h, --help             Print help
  -V, --version          Print version
```
