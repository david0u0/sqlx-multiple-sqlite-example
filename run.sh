set -e

(set -x; DATABASE_URL=sqlite:schema.db cargo run $@)
echo

echo "inspect db1: should have 5 records"
(set -x; sqlite3 output/db1.db 'select count(1) from my_data')

echo

echo "inspect db2: should have 5 records (but instead we only get 4!)"
(set -x; sqlite3 output/db2.db 'select count(1) from my_data')
