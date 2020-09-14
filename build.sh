echo "Building project"

cargo build --release
cp target/release/libknapsack.so knapsack
mv knapsack/libknapsack.so knapsack/knapsack.so

echo "Project builded"

#python3.7 knapsack/__init__.py
