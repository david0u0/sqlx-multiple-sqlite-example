Run the script `./run.sh` (or `./run.sh --release` for release build) in bash. You can see the following result:

```
+ cargo run --release
Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/sqlx-join`

inspect db1: should have 5 records
+ sqlite3 output/db1.db select count(1) from my_data
5

inspect db2: should have 5 records (but instead we only get 4!)
+ sqlite3 output/db2.db select count(1) from my_data
4
```

# Explain
In `main.rs`, I opened two sqlite pools (`output/db1.db` and `output/db2.db`), and insert 5 data into both of them.

However, if inspect the two db after the program exit, we'll see that **db2 only have 4 records**. One of the record is missing.

(It's clear that exactly 5 records are inserted in function `insert_stuff`, and I apply the same function to both sqlite pool)

Also made sure the two db pool are closed properly.

![image](https://user-images.githubusercontent.com/3937480/216809053-81b6456a-d305-4df6-8afe-7a84c4b1b8e0.png)
